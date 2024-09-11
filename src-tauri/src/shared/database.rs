use crate::error::Result;
use crate::error::Error;
use rusqlite::Connection;

pub trait IDb {
    fn new() -> Self;
    fn open(&mut self) -> Result<&mut Self>;
    fn close(&mut self) -> Result<()>;
}

pub struct Db {
    pub connection: Option<Connection>, // state: DbConnection,
}

impl IDb for Db {
    fn new() -> Self {
        Db { connection: None }
    }

    fn open(&mut self) -> Result<&mut Self> {
        let path = "db.sqlite";
        let connection = Connection::open(path).map_err(|e| Error::Db(e.to_string()))?;
        self.connection = Some(connection);
        Ok(self)
    }

    fn close(&mut self) -> Result<()> {
        if let Some(connection) = self.connection.take() {
            connection.close().unwrap();
        }
        Ok(())
    }
}
