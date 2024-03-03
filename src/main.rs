mod databases;

#[macro_use] extern crate rocket;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::Request;
use databases::sqlite::establish_connection;
use crate::databases::schema::users::dsl::*;
use databases::sqlite::User;
use crate::databases::schema::users::name;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_users() -> String {
    let connection = &mut establish_connection();
    let result = users.select(User::as_select()).load(connection).expect("Got an error selecting users");
    let mut string_result = String::new();
    for user in result {
        string_result.push_str(&*user.name);
        string_result.push_str("\n");
    }
    string_result
}

#[post("/user", data = "<user_name>")]
fn create_user(user_name: String) -> String {
    let connection = &mut establish_connection();
    let result = insert_into(users).values(name.eq(user_name)).execute(connection);
    result.unwrap().to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_users, create_user])
        .register("/", catchers![not_found])
}
