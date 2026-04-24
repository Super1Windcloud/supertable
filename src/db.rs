use std::fmt;

use mongodb::{
    bson::Document,
    options::ClientOptions,
    sync::Client as MongoClient,
};
use redis::Commands;
use sqlx::{
    Column, Row,
    mysql::{MySqlPoolOptions, MySqlRow},
    postgres::{PgPoolOptions, PgRow},
    sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteRow},
};

use crate::data::{Connection, ConnectionKind};

#[derive(Clone, Default)]
pub struct DataPreview {
    pub source_label: String,
    pub status_label: String,
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
        ConnectionKind::MySql => run_async(load_mysql_preview(connection)),
        ConnectionKind::PostgreSql => run_async(load_postgres_preview(connection)),
        ConnectionKind::Sqlite => run_async(load_sqlite_preview(connection)),
        ConnectionKind::Redis => load_redis_preview(connection),
        ConnectionKind::MongoDb => load_mongodb_preview(connection),
    }
}

fn run_async<F>(future: F) -> Result<DataPreview, PreviewError>
where
    F: std::future::Future<Output = Result<DataPreview, PreviewError>>,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|err| PreviewError(err.to_string()))?
        .block_on(future)
}

async fn load_mysql_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        connection.username,
        connection.password,
        connection.host,
        connection.port,
        connection.database
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .map_err(|err| PreviewError(err.to_string()))?;

    let tables = sqlx::query("SHOW TABLES")
        .fetch_all(&pool)
        .await
        .map_err(|err| PreviewError(err.to_string()))?
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>(0).ok())
        .collect::<Vec<_>>();

    let first_table = tables.first().cloned();
    let mut preview = DataPreview {
        source_label: format!("MySQL / {}", connection.database),
        status_label: format!("{} tables", tables.len()),
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
        let rows = sqlx::query(&query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        preview.columns = sql_columns_from_mysql(&rows);
        preview.rows = rows.iter().map(mysql_row_to_strings).collect();
    }

    Ok(preview)
}

async fn load_postgres_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        connection.username,
        connection.password,
        connection.host,
        connection.port,
        connection.database
    );
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .map_err(|err| PreviewError(err.to_string()))?;

    let tables = sqlx::query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' ORDER BY table_name",
    )
    .fetch_all(&pool)
    .await
    .map_err(|err| PreviewError(err.to_string()))?
    .into_iter()
    .filter_map(|row| row.try_get::<String, _>(0).ok())
    .collect::<Vec<_>>();

    let first_table = tables.first().cloned();
    let mut preview = DataPreview {
        source_label: format!("PostgreSQL / {}", connection.database),
        status_label: format!("{} tables", tables.len()),
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
        let rows = sqlx::query(&query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        preview.columns = sql_columns_from_postgres(&rows);
        preview.rows = rows.iter().map(postgres_row_to_strings).collect();
    }

    Ok(preview)
}

async fn load_sqlite_preview(connection: &Connection) -> Result<DataPreview, PreviewError> {
    if connection.file_path.is_empty() {
        return Err(PreviewError("SQLite file path is empty".to_string()));
    }

    let options = SqliteConnectOptions::new()
        .filename(&connection.file_path)
        .read_only(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .map_err(|err| PreviewError(err.to_string()))?;

    let tables = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name",
    )
    .fetch_all(&pool)
    .await
    .map_err(|err| PreviewError(err.to_string()))?
    .into_iter()
    .filter_map(|row| row.try_get::<String, _>(0).ok())
    .collect::<Vec<_>>();

    let first_table = tables.first().cloned();
    let mut preview = DataPreview {
        source_label: format!("SQLite / {}", connection.name),
        status_label: format!("{} tables", tables.len()),
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
        let rows = sqlx::query(&query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        preview.columns = sql_columns_from_sqlite(&rows);
        preview.rows = rows.iter().map(sqlite_row_to_strings).collect();
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
        source_label: format!(
            "Redis / db {}",
            if connection.database.is_empty() {
                "0"
            } else {
                &connection.database
            }
        ),
        status_label: format!("{} keys", keys.len()),
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
        let options = ClientOptions::parse(&url)
            .run()
            .map_err(|err| PreviewError(err.to_string()))?;
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

fn sql_columns_from_mysql(rows: &[MySqlRow]) -> Vec<String> {
    rows.first()
        .map(|row| row.columns().iter().map(|column| column.name().to_string()).collect())
        .unwrap_or_default()
}

fn sql_columns_from_postgres(rows: &[PgRow]) -> Vec<String> {
    rows.first()
        .map(|row| row.columns().iter().map(|column| column.name().to_string()).collect())
        .unwrap_or_default()
}

fn sql_columns_from_sqlite(rows: &[SqliteRow]) -> Vec<String> {
    rows.first()
        .map(|row| row.columns().iter().map(|column| column.name().to_string()).collect())
        .unwrap_or_default()
}

fn mysql_row_to_strings(row: &MySqlRow) -> Vec<String> {
    (0..row.len()).map(|index| mysql_cell_to_string(row, index)).collect()
}

fn postgres_row_to_strings(row: &PgRow) -> Vec<String> {
    (0..row.len())
        .map(|index| postgres_cell_to_string(row, index))
        .collect()
}

fn sqlite_row_to_strings(row: &SqliteRow) -> Vec<String> {
    (0..row.len()).map(|index| sqlite_cell_to_string(row, index)).collect()
}

fn mysql_cell_to_string(row: &MySqlRow, index: usize) -> String {
    decode_common_cell(
        row.try_get::<Option<String>, _>(index),
        row.try_get::<Option<i64>, _>(index),
        row.try_get::<Option<i32>, _>(index),
        row.try_get::<Option<i16>, _>(index),
        row.try_get::<Option<u64>, _>(index),
        row.try_get::<Option<f64>, _>(index),
        row.try_get::<Option<bool>, _>(index),
        row.try_get::<Option<Vec<u8>>, _>(index),
    )
}

fn postgres_cell_to_string(row: &PgRow, index: usize) -> String {
    decode_common_cell(
        row.try_get::<Option<String>, _>(index),
        row.try_get::<Option<i64>, _>(index),
        row.try_get::<Option<i32>, _>(index),
        row.try_get::<Option<i16>, _>(index),
        Err(sqlx::Error::ColumnNotFound(String::new())),
        row.try_get::<Option<f64>, _>(index),
        row.try_get::<Option<bool>, _>(index),
        row.try_get::<Option<Vec<u8>>, _>(index),
    )
}

fn sqlite_cell_to_string(row: &SqliteRow, index: usize) -> String {
    decode_common_cell(
        row.try_get::<Option<String>, _>(index),
        row.try_get::<Option<i64>, _>(index),
        row.try_get::<Option<i32>, _>(index),
        row.try_get::<Option<i16>, _>(index),
        Err(sqlx::Error::ColumnNotFound(String::new())),
        row.try_get::<Option<f64>, _>(index),
        row.try_get::<Option<bool>, _>(index),
        row.try_get::<Option<Vec<u8>>, _>(index),
    )
}

fn decode_common_cell(
    as_string: Result<Option<String>, sqlx::Error>,
    as_i64: Result<Option<i64>, sqlx::Error>,
    as_i32: Result<Option<i32>, sqlx::Error>,
    as_i16: Result<Option<i16>, sqlx::Error>,
    as_u64: Result<Option<u64>, sqlx::Error>,
    as_f64: Result<Option<f64>, sqlx::Error>,
    as_bool: Result<Option<bool>, sqlx::Error>,
    as_bytes: Result<Option<Vec<u8>>, sqlx::Error>,
) -> String {
    if let Ok(value) = as_string {
        return value.unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(value) = as_i64 {
        return value
            .map(|item| item.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(value) = as_i32 {
        return value
            .map(|item| item.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(value) = as_i16 {
        return value
            .map(|item| item.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(value) = as_u64 {
        return value
            .map(|item| item.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(value) = as_f64 {
        return value
            .map(|item| item.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(value) = as_bool {
        return value
            .map(|item| item.to_string())
            .unwrap_or_else(|| "NULL".to_string());
    }
    if let Ok(value) = as_bytes {
        return value
            .map(|item| format!("<{} bytes>", item.len()))
            .unwrap_or_else(|| "NULL".to_string());
    }

    "<value>".to_string()
}
