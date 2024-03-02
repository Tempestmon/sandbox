mod database;
mod databases;
mod schema;

#[macro_use] extern crate rocket;

use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::Request;
use databases::sqlite::establish_connection;
use crate::schema::users::dsl::users;
use databases::sqlite::User;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_users() -> Box<str> {
    let connection = &mut establish_connection();
    let result = users.select(User::as_select()).load(connection).expect("Got an error selecting users");
    let mut str_result: Box<str> = Box::from("no result");
    for kek in result {
        println!("{}", kek.name);
        str_result = Box::from(kek.name);
    }
    str_result
}

#[post("/user")]
fn create_user() -> &'static str {

    "Done"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_users, create_user])
        .register("/", catchers![not_found])
}
