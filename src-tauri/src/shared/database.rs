use crate::error::Result;
use rusqlite::Connection;

pub trait IDb {
    fn new() -> Self;
    fn open(&mut self) -> &mut Self;
    fn close(&mut self) -> Result<()>;
}

pub struct Db {
    pub connection: Option<Connection>, // state: DbConnection,
}

impl IDb for Db {
    fn new() -> Self {
        Db { connection: None }
    }
    
    fn open(&mut self) -> &mut Self {
        let path = "db.sqlite";
        let connection = Connection::open(path).expect("error with connection open");
        self.connection = Some(connection);
        self
    }

    fn close(&mut self) -> Result<()> {
        if let Some(connection) = self.connection.take() {
            connection.close().unwrap();
        }
        Ok(())
    }
}