use rusqlite::{Connection, Result as ResultQL};

pub(crate) struct Database {
    connection: Connection,
}

impl Database {
    pub fn create_in_memory_connection() -> Self {
        let connection = Connection::open_in_memory().expect("Could not create in-memory database");
        Database {
            connection
        }
    }

    pub fn create_user(&mut self, name: &str) -> ResultQL<()> {
        let transaction = self.connection.transaction()?;
        transaction.execute("create table users (id integer primary key, name text)", ())?;
        transaction.execute("insert into users values (?1, ?2)", (1, &name))?;
        transaction.commit()
    }

    pub fn show_all_users(&mut self) -> ResultQL<()> {
        let transaction = self.connection.transaction()?;
        transaction.execute("select * from users", ())?;
        transaction.commit()
    }
}