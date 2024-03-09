use chrono::NaiveDateTime;
use diesel::{Connection, Queryable, Selectable, SqliteConnection, Insertable};
use serde::{Deserialize, Serialize};

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("data.db").unwrap_or_else(|_| panic!("keeeek!"))
}


#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::databases::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct User {
    id: i32,
    pub(crate) name: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::databases::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub(crate) struct Post {
    id: i32,
    creation_timestamp: NaiveDateTime,
    title: String,
    pub(crate) body: String,
    user_id: i32,
}