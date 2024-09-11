use crate::error::Error;
use crate::error::Result;
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

impl Db {
    pub fn setup() {
        let path = "db.sqlite";
        if !std::path::Path::exists(std::path::Path::new(&path)) {
            let connection = Connection::open(&path).expect("error with connection open");

            let queries = std::fs::read_to_string("assets/sql/init.sqlite3-query")
                .expect("sql init file doesn't exists");
            let queries = queries.as_str();

            connection
                .execute_batch(queries)
                .expect("ERROR RUNNING QUERIES");
            connection.close().expect("ERROR CLOSING");
        }
    }
}
