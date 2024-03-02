use diesel::{Connection, Queryable, Selectable, SqliteConnection};

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("data.db").unwrap_or_else(|_| panic!("keeeek!"))
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct User {
    id: i32,
    pub(crate) name: String,
}