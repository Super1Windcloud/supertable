use serde::{Deserialize, Serialize};
use std::{env, fs, io, path::PathBuf};

use crate::i18n::Locale;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Connection {
    pub kind: ConnectionKind,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub file_path: String,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum ConnectionKind {
    MySql,
    Sqlite,
    PostgreSql,
    Redis,
    MongoDb,
}

impl ConnectionKind {
    pub const ALL: [ConnectionKind; 5] = [
        ConnectionKind::MySql,
        ConnectionKind::Sqlite,
        ConnectionKind::PostgreSql,
        ConnectionKind::Redis,
        ConnectionKind::MongoDb,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ConnectionKind::MySql => "MySQL",
            ConnectionKind::Sqlite => "SQLite",
            ConnectionKind::PostgreSql => "PostgreSQL",
            ConnectionKind::Redis => "Redis",
            ConnectionKind::MongoDb => "MongoDB",
        }
    }

    pub fn badge(self) -> &'static str {
        match self {
            ConnectionKind::MySql => "MYSQL",
            ConnectionKind::Sqlite => "SQLITE",
            ConnectionKind::PostgreSql => "PG",
            ConnectionKind::Redis => "REDIS",
            ConnectionKind::MongoDb => "MONGO",
        }
    }

    pub fn default_port(self) -> u16 {
        match self {
            ConnectionKind::MySql => 3306,
            ConnectionKind::Sqlite => 0,
            ConnectionKind::PostgreSql => 5432,
            ConnectionKind::Redis => 6379,
            ConnectionKind::MongoDb => 27017,
        }
    }
}

impl Connection {
    pub fn endpoint(&self) -> String {
        match self.kind {
            ConnectionKind::Sqlite => {
                if self.file_path.is_empty() {
                    "sqlite:///choose-a-file.db".to_string()
                } else {
                    format!("sqlite://{}", self.file_path)
                }
            }
            ConnectionKind::MySql => format!("mysql://{}:{}", self.host, self.port),
            ConnectionKind::PostgreSql => format!("postgres://{}:{}", self.host, self.port),
            ConnectionKind::Redis => format!("redis://{}:{}", self.host, self.port),
            ConnectionKind::MongoDb => format!("mongodb://{}:{}", self.host, self.port),
        }
    }

    #[allow(dead_code)]
    pub fn meta(&self, locale: Locale) -> String {
        match self.kind {
            ConnectionKind::Sqlite => {
                if self.file_path.is_empty() {
                    locale.sqlite_file().to_string()
                } else {
                    self.file_path.clone()
                }
            }
            ConnectionKind::Redis => {
                if self.database.is_empty() {
                    locale.memory_cache().to_string()
                } else {
                    format!("db {}", self.database)
                }
            }
            _ => {
                if self.database.is_empty() {
                    locale.endpoint_label(self.kind.label())
                } else {
                    self.database.clone()
                }
            }
        }
    }
}

pub fn load_connections() -> Vec<Connection> {
    let path = connections_file_path();
    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };

    serde_json::from_str::<Vec<Connection>>(&contents).unwrap_or_default()
}

pub fn save_connections(connections: &[Connection]) -> io::Result<()> {
    let path = connections_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(connections)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(path, json)
}

fn connections_file_path() -> PathBuf {
    if let Some(appdata) = env::var_os("APPDATA") {
        return PathBuf::from(appdata)
            .join("SuperTable")
            .join("connections.json");
    }

    if let Some(home) = env::var_os("HOME") {
        return PathBuf::from(home)
            .join(".config")
            .join("supertable")
            .join("connections.json");
    }

    PathBuf::from("connections.json")
}
