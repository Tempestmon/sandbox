mod databases;

#[macro_use] extern crate rocket;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::{json::Json};
use rocket::Request;
use databases::models::establish_connection;
use crate::databases::schema::users::dsl::*;
use databases::models::{User, Post};
use crate::databases::models::{NewPost, NewUser};
use crate::databases::schema::posts::dsl::posts;
use crate::databases::schema::posts::user_id;


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let connection = &mut establish_connection();
    let result = users.select(User::as_select())
        .load(connection)
        .expect("Got an error selecting users");
    Json(result)
}

#[get("/users/<u_id>")]
fn get_user(u_id: i32) -> Json<User> {
    let connection = &mut establish_connection();
    let result = users.find(u_id)
        .select(User::as_select())
        .first(connection)
        .expect("Got an error selecting user by id");
    Json(result)
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<NewUser>) -> Status {
    let connection = &mut establish_connection();
    let result = insert_into(users)
        .values(user.into_inner())
        .execute(connection);
    match result {
        Ok(_) => {Status::Ok}
        Err(_) => {Status::NotAcceptable}
    }
}

#[post("/users/<u_id>/posts", data = "<post>")]
fn create_post(u_id: i32, post: Json<NewPost>) -> Status {
    let new_post = NewPost::new(post.title.clone(), post.body.clone(), u_id);
    let connection = &mut establish_connection();
    let result = insert_into(posts)
        .values(new_post)
        .execute(connection);
    match result {
        Ok(_) => {Status::Ok}
        Err(_) => {Status::NotAcceptable}
    }
}
// TODO: Можно создать пост от имени несуществующего пользователя

#[get("/users/<u_id>/posts")]
fn get_user_posts(u_id: i32) -> Json<Vec<Post>> {
    let connection = &mut establish_connection();
    let result = posts.filter(user_id.eq(u_id))
        .select(Post::as_select())
        .load(connection)
        .expect("No posts found");
    Json(result)
}

#[get("/posts/<p_id>")]
fn get_posts(p_id: i32) -> Json<Post> {
    let connection = &mut establish_connection();
    let result = posts.find(p_id)
        .select(Post::as_select())
        .first(connection)
        .expect("Post is not found");
    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,
                                        get_users,
                                        create_user,
                                        create_post,
                                        get_user,
                                        get_posts,
                                        get_user_posts])
        .register("/", catchers![not_found])
}
