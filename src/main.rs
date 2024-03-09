mod databases;

#[macro_use] extern crate rocket;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::serde::{json::Json};
use rocket::Request;
use databases::models::establish_connection;
use crate::databases::schema::users::dsl::*;
use databases::models::{User, Post};
use crate::databases::schema::posts::dsl::posts;
use crate::databases::schema::posts::{body, title, user_id};
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
    let result = users.select(User::as_select())
        .load(connection)
        .expect("Got an error selecting users");
    let mut string_result = String::new();
    for user in result {
        string_result.push_str(&*user.name);
        string_result.push_str("\n");
    }
    string_result
}

#[get("/users/<u_id>")]
fn get_user(u_id: i32) -> String {
    let connection = &mut establish_connection();
    let result = users.find(u_id)
        .select(User::as_select())
        .first(connection)
        .expect("Got an error selecting user by id");
    result.name
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<User>) -> String {
    let connection = &mut establish_connection();
    let result = insert_into(users)
        .values(name.eq(user.name.clone()))
        .execute(connection);
    result.unwrap().to_string()
}

#[post("/users/<u_id>/posts", data = "<text>")]
fn create_post(u_id: i32, text: String) -> String {
    let connection = &mut establish_connection();
    let result = insert_into(posts)
        .values((title.eq("Test"),
                        body.eq(text),
                        user_id.eq(u_id)))
        .execute(connection);
    result.unwrap().to_string()
}

#[get("/posts/<p_id>")]
fn get_posts(p_id: i32) -> String {
    let connection = &mut establish_connection();
    let result = posts.find(p_id)
        .select(Post::as_select())
        .first(connection)
        .expect("Post is not found");
    result.body
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,
                                        get_users,
                                        create_user,
                                        create_post,
                                        get_user,
                                        get_posts])
        .register("/", catchers![not_found])
}
