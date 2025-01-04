use std::path::PathBuf;

use rusqlite::{Connection, Error as RusqliteError, Result};
use thiserror::Error;

use crate::record::Record;

#[derive(Debug, Default)]
pub struct Client {
    pub connection: Option<Connection>,
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Could not get connection")]
    GetConnectionError(),
    #[error("Could not open connection: {0}")]
    OpenConnectionError(RusqliteError),
    #[error("Could not close connection")]
    CloseConnectionError(),
    #[error("Could not create records table: {0}")]
    CreateRecordsTableError(RusqliteError),
    #[error("Could not get records")]
    GetRecordsError(),
    #[error("Could not insert record")]
    InsertRecordError(RusqliteError),
}

impl Client {
    pub fn get_connection(&self) -> Result<&Connection, ClientError> {
        match &self.connection {
            Some(connection) => Ok(connection),
            None => Err(ClientError::GetConnectionError()),
        }
    }

    pub fn open_connection(
        &mut self,
        mut app_config_path: PathBuf,
        db_name: &str,
    ) -> Result<(), ClientError> {
        app_config_path.push(db_name);

        match Connection::open(app_config_path) {
            Ok(connection) => {
                self.connection = Some(connection);
                Ok(())
            }
            Err(e) => Err(ClientError::OpenConnectionError(e)),
        }
    }

    pub fn close_connection(&mut self) -> Result<(), ClientError> {
        match self.connection.take() {
            Some(connection) => connection
                .close()
                .map_err(|_| ClientError::CloseConnectionError()),
            None => Err(ClientError::GetConnectionError()),
        }
    }

    pub fn create_records_table(&self) -> Result<usize, ClientError> {
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
                    Err(e) => Err(ClientError::CreateRecordsTableError(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_records(&self) -> Result<Vec<Record>, ClientError> {
        let mut stmt =
            if let Ok(statement) = self.get_connection()?.prepare("SELECT * FROM records") {
                statement
            } else {
                return Err(ClientError::GetRecordsError());
            };
        let rows = if let Ok(rows) = stmt.query_map([], |row| {
            Ok(Record {
                id: row.get(0)?,
                wpm: row.get(1)?,
                cpm: row.get(2)?,
                date: row.get(3)?,
            })
        }) {
            rows
        } else {
            return Err(ClientError::GetRecordsError());
        };

        let mut records = Vec::new();
        for record in rows {
            match record {
                Ok(r) => records.push(r),
                Err(_) => Err(ClientError::GetRecordsError())?,
            }
        }

        Ok(records)
    }

    pub fn create_record(&self, wpm: i64, cpm: i64, date: String) -> Result<usize, ClientError> {
        self.get_connection()?
            .execute(
                "INSERT INTO records (wpm, cpm, date) VALUES(?1, ?2, ?3)",
                (wpm, cpm, date),
            )
            .map_err(ClientError::InsertRecordError)
    }
}
