use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::record::Record;

#[derive(Debug, Default)]
pub struct Client {
    pub connection: Option<Connection>,
}

impl Client {
    pub fn get_connection(&self) -> Result<&Connection, Error> {
        match &self.connection {
            Some(connection) => Ok(connection),
            None => Err(Error::new(
                ErrorKind::Other,
                String::from("Could not open connection"),
            )),
        }
    }

    pub fn open_connection(
        &mut self,
        mut app_config_path: PathBuf,
        db_name: &str,
    ) -> Result<(), Error> {
        app_config_path.push(db_name);

        match Connection::open(app_config_path) {
            Ok(connection) => {
                self.connection = Some(connection);
                Ok(())
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Could not open connection, e: {}", e),
            )),
        }
    }

    pub fn close_connection(&mut self) -> Result<(), Error> {
        match self.connection.take() {
            Some(connection) => connection
                .close()
                .map_err(|_| Error::new(ErrorKind::Other, "Could not close connection")),
            None => Err(Error::new(ErrorKind::Other, "Could not find connection")),
        }
    }

    pub fn create_records_table(&self) -> Result<usize, Error> {
        let query = "CREATE TABLE IF NOT EXISTS records (
                     id INTEGER NOT NULL PRIMARY KEY,
                     wpm INTEGER NOT NULL,
                     cpm INTEGER NOT NULL,
                     date TEXT NOT NULL
                    );";
        let connection = self.get_connection();

        match connection {
            Ok(c) => {
                let result = c.execute(query, []);
                match result {
                    Ok(r) => Ok(r),
                    // How do i test here
                    Err(e) => Err(Error::new(
                        ErrorKind::Other,
                        format!("Could not create todos table, e: {}", e),
                    )),
                }
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Could not get connection, e: {}", e),
            )),
        }
    }

    pub fn get_records(&self) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
        let mut stmt = self.get_connection()?.prepare("SELECT * FROM records")?;
        let rows = stmt.query_map([], |row| {
            Ok(Record {
                id: row.get(0)?,
                wpm: row.get(1)?,
                cpm: row.get(2)?,
                date: row.get(3)?,
            })
        })?;

        let mut records = Vec::new();
        for record in rows {
            records.push(record?);
        }

        Ok(records)
    }

    pub fn create_record(&self, wpm: i64, cpm: i64, date: String) -> Result<usize, Error> {
        self.get_connection()?
            .execute(
                "INSERT INTO records (wpm, cpm, date) VALUES(?1, ?2, ?3)",
                (wpm, cpm, date),
            )
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Could not insert record, e: {}", e),
                )
            })
    }
}
