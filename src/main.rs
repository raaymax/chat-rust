#[macro_use] extern crate rocket;
use rocket_db_pools::{Database};
use std::{error::Error, collections::HashMap};
use serde::Deserialize;
use mongodb::{Client, options::ClientOptions, bson::{doc, Document, oid::ObjectId, DateTime}};

#[derive(Clone, Debug, Deserialize)]
struct NotificationsSettings {
    mobile: Option<String>,
    //refreshedAt: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Key {
    p256dh: String,
    auth: String,
}

#[derive(Clone, Debug, Deserialize)]
struct WebPushSettings {
    endpoint: String,
    expirationTime: Option<usize>,
    keys: Key,
}

#[derive(Clone, Debug, Deserialize)]
struct User {
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


#[derive(Database)]
#[database("db")]
struct Db(mongodb::Client);


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
}
/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb://chat:chat@localhost:27017/chat?authSource=admin").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("chat");
    let col = db.collection::<User>("users");
        
    let mut cursor = col.find(doc! { }, None).await?;
    while cursor.advance().await? {
        println!("{:?}", cursor.deserialize_current());
    }
    Ok(())
}
*/
/*
Ok(Document({
    "_id": ObjectId("6255a4156c28443c92c26d7e"),
    "clientId": String("7ed5c52c-35f8-4a27-929d-ff5eb1a74924"),
    "login": String("melisa"),
    "password": String("$2b$10$TUjjhWmrPeOv/0K9wswAPODNB84ueJ5NdymserUOfnIuVADSlpfZS"),
    "name": String("Melisa"), 
    "avatarUrl": String("https://codecat-test.imgix.net/d0a3fcdb-7cd3-47c5-a093-1e7cb86d3f33"),
    "notifications": Document({"melisaToken": Document({"mobile": Null, "refreshedAt": DateTime(2023-02-24 22:37:36.771 +00:00:00)})}),
    "mainChannelId": ObjectId("62799d2cbd5c61a36a7cd9e0"),
    "passport": String("$2b$10$TUjjhWmrPeOv/0K9wswAPODNB84ueJ5NdymserUOfnIuVADSlpfZS"),
    "avatarFileId": String("d0a3fcdb-7cd3-47c5-a093-1e7cb86d3f33"),
    "lastSeen": DateTime(2023-07-18 9:07:48.515 +00:00:00)}))
    */
