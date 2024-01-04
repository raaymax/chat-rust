use rocket::local::blocking::Client;
use rocket::http::{ContentType, Status};
use rocket::serde::{json::Json};

#[test]
fn ping() {
    let client = Client::new(server::rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}

#[test]
fn users() {
    let client = Client::new(server::rocket()).expect("valid rocket instance");
    let response = client.get("/users").dispatch();
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::Ok);
    //println!("{:?}", response.into_string());
    println!("{:?}", response.into_json::<server::UserList>());
    assert!(false);
}
