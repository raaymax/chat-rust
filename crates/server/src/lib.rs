#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<ID, String>>;
use rocket_db_pools::{Database};
use std::{error::Error};
use serde::Deserialize;
use mongodb::{Client, options::ClientOptions, bson::{doc, Document, oid::ObjectId, DateTime}};
use rocket::serde::{json::Json, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct NotificationsSettings {
    mobile: Option<String>,
    //refreshedAt: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Key {
    p256dh: String,
    auth: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct WebPushSettings {
    endpoint: String,
    expirationTime: Option<usize>,
    keys: Key,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(rename(serialize = "id", deserialize = "_id"))]
    id: ObjectId,
    name: String,
    avatarUrl: Option<String>,
    login: String,
    password: Option<String>,
    clientId: Option<String>,
    mainChannelId: Option<ObjectId>,
    avatarFileId: Option<String>,
    lastSeen: Option<DateTime>,
    system: Option<bool>,
    //notifications: Option<HashMap<String, NotificationsSettings>>,
    webPush: Option<HashMap<String, WebPushSettings>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserList {
    users: Vec<User>,
}

impl Default for UserList {
    fn default() -> Self {
        UserList::new()
    }
}

impl UserList {
    pub fn new() -> Self {
        UserList {
            users: Vec::new(),
        }
    }
    pub fn push(&mut self, user: User) {
        self.users.push(user);
    }
}

#[derive(Clone, Debug, Serialize)]
struct ErrorCode {
    code: u32,
    message: String,
}

#[derive(Database)]
#[database("db")]
struct Db(mongodb::Client);

#[get("/users")]
async fn users() -> Result<Json<UserList>, Json<ErrorCode>> {
    let client_options = ClientOptions::parse("mongodb://chat:chat@localhost:27017/chat?authSource=admin").await.map_err(|e| Json(ErrorCode { code: 500, message: e.to_string() }))?;
    let client = Client::with_options(client_options).map_err(|e| Json(ErrorCode { code: 500, message: e.to_string() }))?;
    let db = client.database("chat");
    let col = db.collection::<User>("users");
        
    let mut cursor = col.find(doc! { }, None).await.map_err(|e| Json(ErrorCode { code: 500, message: e.to_string() }))?;
    let mut users = UserList::default();
    while cursor.advance().await.map_err(|e| Json(ErrorCode { code: 500, message: e.to_string() }))? {
        users.push(cursor.deserialize_current().map_err(|e| Json(ErrorCode { code: 500, message: e.to_string() }))?);
    }
    Ok(Json(users))
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, users])
}
