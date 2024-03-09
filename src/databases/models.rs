use chrono::{NaiveDateTime, Utc};
use diesel::{Connection, Queryable, Selectable, SqliteConnection, Insertable};
use serde::{Deserialize, Serialize};

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("data.db").unwrap_or_else(|_| panic!("keeeek!"))
}


#[derive(Queryable, Selectable, Serialize, )]
#[diesel(table_name = crate::databases::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::databases::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUser {
    pub name: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::databases::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Post {
    id: i32,
    creation_timestamp: NaiveDateTime,
    title: String,
    pub body: String,
    user_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::databases::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct NewPost {
    pub title: String,
    pub body: String,
    creation_timestamp: Option<NaiveDateTime>,
    user_id: Option<i32>,
}

impl NewPost {
    pub fn new(title: String, body: String, user_id: i32) -> Self {
        NewPost {
            title,
            body,
            creation_timestamp: Some(Utc::now().naive_utc()),
            user_id: Some(user_id),
        }
    }
}