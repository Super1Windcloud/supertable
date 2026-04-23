use std::fmt;

use mongodb::{
    bson::Document,
    options::ClientOptions,
    sync::Client as MongoClient,
};
use mysql::{Opts, Pool, PooledConn, Row as MySqlRow, Value as MySqlValue, prelude::Queryable};
use postgres::{Client as PostgresClient, NoTls, Row as PostgresRow, types::Type};
use redis::{Commands, ConnectionLike};
use rusqlite::{Connection as SqliteConnection, types::ValueRef};

use crate::data::{Connection, ConnectionKind};

#[derive(Clone, Default)]
pub struct DataPreview {
    pub source_label: String,
    pub status_label: String,
    pub query_lines: Vec<String>,
    pub schema_items: Vec<SchemaEntry>,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Clone)]
pub struct SchemaEntry {
    pub name: String,
    pub count: usize,
    pub active: bool,
}

#[derive(Debug)]
pub struct PreviewError(pub String);

impl fmt::Display for PreviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn load_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    match connection.kind {
        ConnectionKind::MySql => load_mysql_preview(connection),
        ConnectionKind::PostgreSql => load_postgres_preview(connection),
        ConnectionKind::Sqlite => load_sqlite_preview(connection),
        ConnectionKind::Redis => load_redis_preview(connection),
        ConnectionKind::MongoDb => load_mongodb_preview(connection),
    }
}

fn load_mysql_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        connection.username,
        connection.password,
        connection.host,
        connection.port,
        connection.database
    );
    let opts = Opts::from_url(&url).map_err(|err| PreviewError(err.to_string()))?;
    let pool = Pool::new(opts).map_err(|err| PreviewError(err.to_string()))?;
    let mut conn = pool.get_conn().map_err(|err| PreviewError(err.to_string()))?;

    let tables: Vec<String> = conn
        .query_map("SHOW TABLES", |name: String| name)
        .map_err(|err| PreviewError(err.to_string()))?;

    sql_preview_from_mysql(&mut conn, "MySQL", tables, connection.database.clone())
}

fn sql_preview_from_mysql(
    conn: &mut PooledConn,
    label: &str,
    tables: Vec<String>,
    database_name: String,
) -> Result<DataPreview, PreviewError> {
    let first_table = tables.first().cloned();
    let mut preview = DataPreview {
        source_label: format!("{label} / {database_name}"),
        status_label: format!("{} tables", tables.len()),
        query_lines: first_table
            .as_ref()
            .map(|table| vec![format!("SELECT * FROM `{table}` LIMIT 50;")])
            .unwrap_or_else(|| vec!["-- No table found in current database".to_string()]),
        schema_items: vec![SchemaEntry {
            name: "Tables".to_string(),
            count: tables.len(),
            active: true,
        }],
        columns: Vec::new(),
        rows: Vec::new(),
    };

    if let Some(table) = first_table {
        let query = format!("SELECT * FROM `{table}` LIMIT 50");
        let result = conn.query_iter(query).map_err(|err| PreviewError(err.to_string()))?;
        let columns = result
            .columns()
            .as_ref()
            .iter()
            .map(|col| col.name_str().to_string())
            .collect::<Vec<_>>();
        let rows = result
            .map(|row| row.map(mysql_row_to_strings).map_err(|err| PreviewError(err.to_string())))
            .collect::<Result<Vec<_>, _>>()?;
        preview.columns = columns;
        preview.rows = rows;
    }

    Ok(preview)
}

fn mysql_row_to_strings(row: MySqlRow) -> Vec<String> {
    row.unwrap()
        .into_iter()
        .map(mysql_value_to_string)
        .collect()
}

fn mysql_value_to_string(value: MySqlValue) -> String {
    match value {
        MySqlValue::NULL => "NULL".to_string(),
        MySqlValue::Bytes(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        MySqlValue::Int(v) => v.to_string(),
        MySqlValue::UInt(v) => v.to_string(),
        MySqlValue::Float(v) => v.to_string(),
        MySqlValue::Double(v) => v.to_string(),
        MySqlValue::Date(y, m, d, h, i, s, us) => {
            format!("{y:04}-{m:02}-{d:02} {h:02}:{i:02}:{s:02}.{}", us)
        }
        MySqlValue::Time(neg, d, h, i, s, us) => {
            let sign = if neg { "-" } else { "" };
            format!("{sign}{d}d {h:02}:{i:02}:{s:02}.{}", us)
        }
    }
}

fn load_postgres_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    let url = format!(
        "host={} port={} user={} password={} dbname={}",
        connection.host, connection.port, connection.username, connection.password, connection.database
    );
    let mut client =
        PostgresClient::connect(&url, NoTls).map_err(|err| PreviewError(err.to_string()))?;

    let tables = client
        .query(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name",
            &[],
        )
        .map_err(|err| PreviewError(err.to_string()))?
        .into_iter()
        .map(|row| row.get::<usize, String>(0))
        .collect::<Vec<_>>();

    let first_table = tables.first().cloned();
    let mut preview = DataPreview {
        source_label: format!("PostgreSQL / {}", connection.database),
        status_label: format!("{} tables", tables.len()),
        query_lines: first_table
            .as_ref()
            .map(|table| vec![format!("SELECT * FROM \"{table}\" LIMIT 50;")])
            .unwrap_or_else(|| vec!["-- No table found in public schema".to_string()]),
        schema_items: vec![SchemaEntry {
            name: "Tables".to_string(),
            count: tables.len(),
            active: true,
        }],
        columns: Vec::new(),
        rows: Vec::new(),
    };

    if let Some(table) = first_table {
        let query = format!("SELECT * FROM \"{table}\" LIMIT 50");
        let rows = client
            .query(&query, &[])
            .map_err(|err| PreviewError(err.to_string()))?;
        if let Some(first) = rows.first() {
            preview.columns = first
                .columns()
                .iter()
                .map(|column| column.name().to_string())
                .collect();
        }
        preview.rows = rows.iter().map(postgres_row_to_strings).collect();
    }

    Ok(preview)
}

fn postgres_row_to_strings(row: &PostgresRow) -> Vec<String> {
    row.columns()
        .iter()
        .enumerate()
        .map(|(index, col)| postgres_value_to_string(row, index, col.type_()))
        .collect()
}

fn postgres_value_to_string(row: &PostgresRow, index: usize, ty: &Type) -> String {
    if *ty == Type::BOOL {
        row.try_get::<usize, Option<bool>>(index)
            .ok()
            .flatten()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string())
    } else if *ty == Type::INT2 || *ty == Type::INT4 || *ty == Type::INT8 {
        row.try_get::<usize, Option<i64>>(index)
            .ok()
            .flatten()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string())
    } else if *ty == Type::FLOAT4 || *ty == Type::FLOAT8 || *ty == Type::NUMERIC {
        row.try_get::<usize, Option<f64>>(index)
            .ok()
            .flatten()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "NULL".to_string())
    } else {
        row.try_get::<usize, Option<String>>(index)
            .ok()
            .flatten()
            .unwrap_or_else(|| "<binary>".to_string())
    }
}

fn load_sqlite_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    let db_path = if connection.file_path.is_empty() {
        return Err(PreviewError("SQLite file path is empty".to_string()));
    } else {
        connection.file_path.clone()
    };

    let conn = SqliteConnection::open(db_path).map_err(|err| PreviewError(err.to_string()))?;
    let tables = {
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .map_err(|err| PreviewError(err.to_string()))?;
        stmt.query_map([], |row| row.get::<usize, String>(0))
            .map_err(|err| PreviewError(err.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| PreviewError(err.to_string()))?
    };

    let first_table = tables.first().cloned();
    let mut preview = DataPreview {
        source_label: format!("SQLite / {}", connection.name),
        status_label: format!("{} tables", tables.len()),
        query_lines: first_table
            .as_ref()
            .map(|table| vec![format!("SELECT * FROM \"{table}\" LIMIT 50;")])
            .unwrap_or_else(|| vec!["-- No table found in current file".to_string()]),
        schema_items: vec![SchemaEntry {
            name: "Tables".to_string(),
            count: tables.len(),
            active: true,
        }],
        columns: Vec::new(),
        rows: Vec::new(),
    };

    if let Some(table) = first_table {
        let query = format!("SELECT * FROM \"{table}\" LIMIT 50");
        let mut stmt = conn.prepare(&query).map_err(|err| PreviewError(err.to_string()))?;
        preview.columns = stmt.column_names().iter().map(|name| name.to_string()).collect();
        let col_count = preview.columns.len();
        let mapped = stmt
            .query_map([], |row| {
                (0..col_count)
                    .map(|idx| {
                        row.get_ref(idx).map(|value| match value {
                            ValueRef::Null => "NULL".to_string(),
                            ValueRef::Integer(v) => v.to_string(),
                            ValueRef::Real(v) => v.to_string(),
                            ValueRef::Text(v) => String::from_utf8_lossy(v).to_string(),
                            ValueRef::Blob(_) => "<blob>".to_string(),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|err| PreviewError(err.to_string()))?;
        preview.rows = mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| PreviewError(err.to_string()))?;
    }

    Ok(preview)
}

fn load_redis_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    let url = format!("redis://{}:{}/", connection.host, connection.port);
    let client = redis::Client::open(url).map_err(|err| PreviewError(err.to_string()))?;
    let mut conn = client
        .get_connection()
        .map_err(|err| PreviewError(err.to_string()))?;

    let _: () = if connection.database.is_empty() {
        Ok(())
    } else {
        redis::cmd("SELECT")
            .arg(connection.database.clone())
            .query(&mut conn)
            .map_err(|err| PreviewError(err.to_string()))
    }?;

    let keys: Vec<String> = conn.keys("*").map_err(|err| PreviewError(err.to_string()))?;
    let mut rows = Vec::new();

    for key in keys.iter().take(50) {
        let kind = redis::cmd("TYPE")
            .arg(key)
            .query::<String>(&mut conn)
            .map_err(|err| PreviewError(err.to_string()))?;
        let value = match kind.as_str() {
            "string" => conn.get::<_, String>(key).unwrap_or_else(|_| "<binary>".to_string()),
            "list" => redis::cmd("LRANGE")
                .arg(key)
                .arg(0)
                .arg(4)
                .query::<Vec<String>>(&mut conn)
                .map(|items| items.join(", "))
                .unwrap_or_else(|_| "<unavailable>".to_string()),
            "set" => redis::cmd("SMEMBERS")
                .arg(key)
                .query::<Vec<String>>(&mut conn)
                .map(|items| items.into_iter().take(5).collect::<Vec<_>>().join(", "))
                .unwrap_or_else(|_| "<unavailable>".to_string()),
            "hash" => redis::cmd("HGETALL")
                .arg(key)
                .query::<Vec<String>>(&mut conn)
                .map(|items| items.into_iter().take(10).collect::<Vec<_>>().join(", "))
                .unwrap_or_else(|_| "<unavailable>".to_string()),
            _ => format!("<{kind}>"),
        };
        rows.push(vec![key.clone(), kind, value]);
    }

    Ok(DataPreview {
        source_label: format!("Redis / db {}", if connection.database.is_empty() { "0" } else { &connection.database }),
        status_label: format!("{} keys", keys.len()),
        query_lines: vec![
            "SCAN 0".to_string(),
            "TYPE <key>".to_string(),
            "GET <key>".to_string(),
        ],
        schema_items: vec![SchemaEntry {
            name: "Keys".to_string(),
            count: keys.len(),
            active: true,
        }],
        columns: vec!["Key".to_string(), "Type".to_string(), "Value Preview".to_string()],
        rows,
    })
}

fn load_mongodb_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    let url = format!(
        "mongodb://{}:{}@{}:{}/",
        connection.username, connection.password, connection.host, connection.port
    );
    let client = if connection.username.is_empty() {
        MongoClient::with_uri_str(format!("mongodb://{}:{}/", connection.host, connection.port))
            .map_err(|err| PreviewError(err.to_string()))?
    } else {
        let options = ClientOptions::parse(&url).run().map_err(|err| PreviewError(err.to_string()))?;
        MongoClient::with_options(options).map_err(|err| PreviewError(err.to_string()))?
    };

    let database = client.database(&connection.database);
    let collections = database
        .list_collection_names()
        .run()
        .map_err(|err| PreviewError(err.to_string()))?;
    let first_collection = collections.first().cloned();
    let mut preview = DataPreview {
        source_label: format!("MongoDB / {}", connection.database),
        status_label: format!("{} collections", collections.len()),
        query_lines: first_collection
            .as_ref()
            .map(|name| vec![format!("db.{name}.find({{}}).limit(50)")])
            .unwrap_or_else(|| vec!["// No collection found in current database".to_string()]),
        schema_items: vec![SchemaEntry {
            name: "Collections".to_string(),
            count: collections.len(),
            active: true,
        }],
        columns: vec!["_id".to_string(), "document".to_string()],
        rows: Vec::new(),
    };

    if let Some(collection_name) = first_collection {
        let collection = database.collection::<Document>(&collection_name);
        let docs = collection
            .find(Document::new())
            .limit(50)
            .run()
            .map_err(|err| PreviewError(err.to_string()))?;
        preview.rows = docs
            .filter_map(Result::ok)
            .map(|doc| {
                let id = doc.get("_id").map(|value| value.to_string()).unwrap_or_default();
                vec![id, doc.to_string()]
            })
            .collect();
    }

    Ok(preview)
}
