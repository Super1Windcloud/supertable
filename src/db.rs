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
    pub object_names: Vec<String>,
    pub active_object: Option<String>,
    pub query_lines: Vec<String>,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub binding: PreviewBinding,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct SchemaEntry {
    pub name: String,
    pub count: usize,
    pub active: bool,
}

#[derive(Clone, Default)]
pub enum PreviewBinding {
    #[default]
    None,
    SqlTable {
        dialect: SqlDialect,
        table_name: String,
        original_rows: Vec<Vec<String>>,
    },
    RedisKeys {
        key_types: Vec<String>,
    },
    MongoCollection {
        _collection_name: String,
    },
}

#[derive(Clone, Copy)]
pub enum SqlDialect {
    MySql,
    PostgreSql,
    Sqlite,
}

#[derive(Debug)]
pub struct PreviewError(pub String);

impl fmt::Display for PreviewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn load_preview(
    connection: &Connection,
    selected_object: Option<&str>,
) -> Result<DataPreview, PreviewError> {
    match connection.kind {
        ConnectionKind::MySql => run_async(load_mysql_preview(connection, selected_object)),
        ConnectionKind::PostgreSql => run_async(load_postgres_preview(connection, selected_object)),
        ConnectionKind::Sqlite => run_async(load_sqlite_preview(connection, selected_object)),
        ConnectionKind::Redis => load_redis_preview(connection, selected_object),
        ConnectionKind::MongoDb => load_mongodb_preview(connection, selected_object),
    }
}

pub fn save_cell_edit(
    connection: &Connection,
    preview: &DataPreview,
    row_index: usize,
    column_index: usize,
    new_value: &str,
) -> Result<(), PreviewError> {
    match &preview.binding {
        PreviewBinding::SqlTable {
            dialect,
            table_name,
            original_rows,
        } => {
            let Some(original_row) = original_rows.get(row_index) else {
                return Err(PreviewError("Row out of range".to_string()));
            };
            run_async(save_sql_cell_edit(
                connection,
                *dialect,
                table_name.clone(),
                preview.columns.clone(),
                original_row.clone(),
                column_index,
                new_value.to_string(),
            ))
        }
        PreviewBinding::RedisKeys { key_types } => {
            save_redis_cell_edit(connection, preview, row_index, column_index, key_types, new_value)
        }
        PreviewBinding::MongoCollection { .. } => Err(PreviewError(
            "MongoDB grid write-back is not available yet".to_string(),
        )),
        PreviewBinding::None => Err(PreviewError("No editable data source is active".to_string())),
    }
}

pub fn execute_query(connection: &Connection, query: &str) -> Result<DataPreview, PreviewError> {
    match connection.kind {
        ConnectionKind::MySql => run_async(execute_mysql_query(connection, query)),
        ConnectionKind::PostgreSql => run_async(execute_postgres_query(connection, query)),
        ConnectionKind::Sqlite => run_async(execute_sqlite_query(connection, query)),
        ConnectionKind::Redis => execute_redis_command(connection, query),
        ConnectionKind::MongoDb => Err(PreviewError(
            "MongoDB ad-hoc query execution is not available yet".to_string(),
        )),
    }
}

pub fn explain_query(connection: &Connection, query: &str) -> Result<DataPreview, PreviewError> {
    match connection.kind {
        ConnectionKind::MySql => run_async(execute_mysql_query(
            connection,
            &format!("EXPLAIN {query}"),
        )),
        ConnectionKind::PostgreSql => run_async(execute_postgres_query(
            connection,
            &format!("EXPLAIN {query}"),
        )),
        ConnectionKind::Sqlite => run_async(execute_sqlite_query(
            connection,
            &format!("EXPLAIN QUERY PLAN {query}"),
        )),
        ConnectionKind::Redis | ConnectionKind::MongoDb => Err(PreviewError(
            "Explain is currently available only for SQL sources".to_string(),
        )),
    }
}

fn run_async<F, T>(future: F) -> Result<T, PreviewError>
where
    F: std::future::Future<Output = Result<T, PreviewError>>,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|err| PreviewError(err.to_string()))?
        .block_on(future)
}

async fn load_mysql_preview(
    connection: &Connection,
    selected_object: Option<&str>,
) -> Result<DataPreview, PreviewError> {
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

    let current_table = selected_object
        .filter(|name| tables.iter().any(|table| table == name))
        .map(str::to_string)
        .or_else(|| tables.first().cloned());
    let mut preview = DataPreview {
        source_label: format!("MySQL / {}", connection.database),
        status_label: format!("{} tables", tables.len()),
        schema_items: vec![SchemaEntry {
            name: "Tables".to_string(),
            count: tables.len(),
            active: true,
        }],
        object_names: tables.clone(),
        active_object: current_table.clone(),
        query_lines: current_table
            .as_ref()
            .map(|table| {
                vec![
                    "SELECT DISTINCT *".to_string(),
                    format!("FROM `{table}`"),
                    "LIMIT 50;".to_string(),
                ]
            })
            .unwrap_or_else(|| vec!["-- No table found".to_string()]),
        columns: Vec::new(),
        rows: Vec::new(),
        binding: PreviewBinding::None,
    };

    if let Some(table) = current_table {
        let query = format!("SELECT * FROM `{table}` LIMIT 50");
        let rows = sqlx::query(&query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        preview.columns = sql_columns_from_mysql(&rows);
        preview.rows = rows.iter().map(mysql_row_to_strings).collect();
        preview.binding = PreviewBinding::SqlTable {
            dialect: SqlDialect::MySql,
            table_name: table,
            original_rows: preview.rows.clone(),
        };
    }

    Ok(preview)
}

async fn execute_mysql_query(
    connection: &Connection,
    query: &str,
) -> Result<DataPreview, PreviewError> {
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&format!(
            "mysql://{}:{}@{}:{}/{}",
            connection.username,
            connection.password,
            connection.host,
            connection.port,
            connection.database
        ))
        .await
        .map_err(|err| PreviewError(err.to_string()))?;

    if is_select_like(query) {
        let rows = sqlx::query(query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        Ok(DataPreview {
            source_label: format!("MySQL / {}", connection.database),
            status_label: format!("{} rows", rows.len()),
            columns: sql_columns_from_mysql(&rows),
            rows: rows.iter().map(mysql_row_to_strings).collect(),
            query_lines: query.lines().map(ToString::to_string).collect(),
            ..DataPreview::default()
        })
    } else {
        let result = sqlx::query(query)
            .execute(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        Ok(DataPreview {
            source_label: format!("MySQL / {}", connection.database),
            status_label: format!("{} rows affected", result.rows_affected()),
            query_lines: query.lines().map(ToString::to_string).collect(),
            ..DataPreview::default()
        })
    }
}

async fn load_postgres_preview(
    connection: &Connection,
    selected_object: Option<&str>,
) -> Result<DataPreview, PreviewError> {
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

    let current_table = selected_object
        .filter(|name| tables.iter().any(|table| table == name))
        .map(str::to_string)
        .or_else(|| tables.first().cloned());
    let mut preview = DataPreview {
        source_label: format!("PostgreSQL / {}", connection.database),
        status_label: format!("{} tables", tables.len()),
        schema_items: vec![SchemaEntry {
            name: "Tables".to_string(),
            count: tables.len(),
            active: true,
        }],
        object_names: tables.clone(),
        active_object: current_table.clone(),
        query_lines: current_table
            .as_ref()
            .map(|table| {
                vec![
                    "SELECT DISTINCT *".to_string(),
                    format!("FROM \"{table}\""),
                    "LIMIT 50;".to_string(),
                ]
            })
            .unwrap_or_else(|| vec!["-- No table found".to_string()]),
        columns: Vec::new(),
        rows: Vec::new(),
        binding: PreviewBinding::None,
    };

    if let Some(table) = current_table {
        let query = format!("SELECT * FROM \"{table}\" LIMIT 50");
        let rows = sqlx::query(&query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        preview.columns = sql_columns_from_postgres(&rows);
        preview.rows = rows.iter().map(postgres_row_to_strings).collect();
        preview.binding = PreviewBinding::SqlTable {
            dialect: SqlDialect::PostgreSql,
            table_name: table,
            original_rows: preview.rows.clone(),
        };
    }

    Ok(preview)
}

async fn execute_postgres_query(
    connection: &Connection,
    query: &str,
) -> Result<DataPreview, PreviewError> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&format!(
            "postgres://{}:{}@{}:{}/{}",
            connection.username,
            connection.password,
            connection.host,
            connection.port,
            connection.database
        ))
        .await
        .map_err(|err| PreviewError(err.to_string()))?;

    if is_select_like(query) {
        let rows = sqlx::query(query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        Ok(DataPreview {
            source_label: format!("PostgreSQL / {}", connection.database),
            status_label: format!("{} rows", rows.len()),
            columns: sql_columns_from_postgres(&rows),
            rows: rows.iter().map(postgres_row_to_strings).collect(),
            query_lines: query.lines().map(ToString::to_string).collect(),
            ..DataPreview::default()
        })
    } else {
        let result = sqlx::query(query)
            .execute(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        Ok(DataPreview {
            source_label: format!("PostgreSQL / {}", connection.database),
            status_label: format!("{} rows affected", result.rows_affected()),
            query_lines: query.lines().map(ToString::to_string).collect(),
            ..DataPreview::default()
        })
    }
}

async fn load_sqlite_preview(
    connection: &Connection,
    selected_object: Option<&str>,
) -> Result<DataPreview, PreviewError> {
    if connection.file_path.is_empty() {
        return Err(PreviewError("SQLite file path is empty".to_string()));
    }

    let options = SqliteConnectOptions::new()
        .filename(&connection.file_path)
        .create_if_missing(false);
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

    let current_table = selected_object
        .filter(|name| tables.iter().any(|table| table == name))
        .map(str::to_string)
        .or_else(|| tables.first().cloned());
    let mut preview = DataPreview {
        source_label: format!("SQLite / {}", connection.name),
        status_label: format!("{} tables", tables.len()),
        schema_items: vec![SchemaEntry {
            name: "Tables".to_string(),
            count: tables.len(),
            active: true,
        }],
        object_names: tables.clone(),
        active_object: current_table.clone(),
        query_lines: current_table
            .as_ref()
            .map(|table| {
                vec![
                    "SELECT *".to_string(),
                    format!("FROM \"{table}\""),
                    "LIMIT 50;".to_string(),
                ]
            })
            .unwrap_or_else(|| vec!["-- No table found".to_string()]),
        columns: Vec::new(),
        rows: Vec::new(),
        binding: PreviewBinding::None,
    };

    if let Some(table) = current_table {
        let query = format!("SELECT * FROM \"{table}\" LIMIT 50");
        let rows = sqlx::query(&query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        preview.columns = sql_columns_from_sqlite(&rows);
        preview.rows = rows.iter().map(sqlite_row_to_strings).collect();
        preview.binding = PreviewBinding::SqlTable {
            dialect: SqlDialect::Sqlite,
            table_name: table,
            original_rows: preview.rows.clone(),
        };
    }

    Ok(preview)
}

async fn execute_sqlite_query(
    connection: &Connection,
    query: &str,
) -> Result<DataPreview, PreviewError> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(
            SqliteConnectOptions::new()
                .filename(&connection.file_path)
                .create_if_missing(false),
        )
        .await
        .map_err(|err| PreviewError(err.to_string()))?;

    if is_select_like(query) {
        let rows = sqlx::query(query)
            .fetch_all(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        Ok(DataPreview {
            source_label: format!("SQLite / {}", connection.name),
            status_label: format!("{} rows", rows.len()),
            columns: sql_columns_from_sqlite(&rows),
            rows: rows.iter().map(sqlite_row_to_strings).collect(),
            query_lines: query.lines().map(ToString::to_string).collect(),
            ..DataPreview::default()
        })
    } else {
        let result = sqlx::query(query)
            .execute(&pool)
            .await
            .map_err(|err| PreviewError(err.to_string()))?;
        Ok(DataPreview {
            source_label: format!("SQLite / {}", connection.name),
            status_label: format!("{} rows affected", result.rows_affected()),
            query_lines: query.lines().map(ToString::to_string).collect(),
            ..DataPreview::default()
        })
    }
}

fn load_redis_preview(
    connection: &Connection,
    selected_object: Option<&str>,
) -> Result<DataPreview, PreviewError> {
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
    let current_key = selected_object
        .filter(|name| keys.iter().any(|key| key == name))
        .map(str::to_string);
    let mut rows = Vec::new();
    let mut key_types = Vec::new();

    for key in keys.iter().take(50) {
        if let Some(active_key) = &current_key {
            if key != active_key {
                continue;
            }
        }
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
        key_types.push(kind.clone());
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
        object_names: keys.iter().take(50).cloned().collect(),
        active_object: current_key,
        query_lines: preview_redis_query(selected_object),
        columns: vec!["Key".to_string(), "Type".to_string(), "Value Preview".to_string()],
        rows,
        binding: PreviewBinding::RedisKeys { key_types },
    })
}

fn load_mongodb_preview(
    connection: &Connection,
    selected_object: Option<&str>,
) -> Result<DataPreview, PreviewError> {
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
    let current_collection = selected_object
        .filter(|name| collections.iter().any(|collection| collection == name))
        .map(str::to_string)
        .or_else(|| collections.first().cloned());
    let mut preview = DataPreview {
        source_label: format!("MongoDB / {}", connection.database),
        status_label: format!("{} collections", collections.len()),
        schema_items: vec![SchemaEntry {
            name: "Collections".to_string(),
            count: collections.len(),
            active: true,
        }],
        object_names: collections.clone(),
        active_object: current_collection.clone(),
        query_lines: current_collection
            .as_ref()
            .map(|name| vec![format!("db.{name}.find({{}}).limit(50)")])
            .unwrap_or_else(|| vec!["// No collection found".to_string()]),
        columns: vec!["_id".to_string(), "document".to_string()],
        rows: Vec::new(),
        binding: PreviewBinding::MongoCollection {
            _collection_name: current_collection.clone().unwrap_or_default(),
        },
    };

    if let Some(collection_name) = current_collection {
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

fn execute_redis_command(connection: &Connection, query: &str) -> Result<DataPreview, PreviewError> {
    let parts = query.split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() {
        return Err(PreviewError("Redis command is empty".to_string()));
    }

    let url = format!("redis://{}:{}/", connection.host, connection.port);
    let client = redis::Client::open(url).map_err(|err| PreviewError(err.to_string()))?;
    let mut conn = client
        .get_connection()
        .map_err(|err| PreviewError(err.to_string()))?;

    if !connection.database.is_empty() {
        let _: () = redis::cmd("SELECT")
            .arg(connection.database.clone())
            .query(&mut conn)
            .map_err(|err| PreviewError(err.to_string()))?;
    }

    match parts[0].to_uppercase().as_str() {
        "GET" if parts.len() >= 2 => {
            let value = conn
                .get::<_, String>(parts[1])
                .unwrap_or_else(|_| "<binary>".to_string());
            Ok(DataPreview {
                source_label: format!("Redis / db {}", connection.database),
                status_label: "1 row".to_string(),
                columns: vec!["Key".to_string(), "Value".to_string()],
                rows: vec![vec![parts[1].to_string(), value]],
                query_lines: vec![query.to_string()],
                ..DataPreview::default()
            })
        }
        "TYPE" if parts.len() >= 2 => {
            let value = redis::cmd("TYPE")
                .arg(parts[1])
                .query::<String>(&mut conn)
                .map_err(|err| PreviewError(err.to_string()))?;
            Ok(DataPreview {
                source_label: format!("Redis / db {}", connection.database),
                status_label: "1 row".to_string(),
                columns: vec!["Key".to_string(), "Type".to_string()],
                rows: vec![vec![parts[1].to_string(), value]],
                query_lines: vec![query.to_string()],
                ..DataPreview::default()
            })
        }
        "SCAN" => {
            let keys: Vec<String> = conn.keys("*").map_err(|err| PreviewError(err.to_string()))?;
            Ok(DataPreview {
                source_label: format!("Redis / db {}", connection.database),
                status_label: format!("{} keys", keys.len()),
                columns: vec!["Key".to_string()],
                rows: keys.into_iter().take(50).map(|key| vec![key]).collect(),
                query_lines: vec![query.to_string()],
                ..DataPreview::default()
            })
        }
        _ => Err(PreviewError(
            "Supported Redis commands: GET, TYPE, SCAN".to_string(),
        )),
    }
}

fn is_select_like(query: &str) -> bool {
    let normalized = query.trim_start().to_uppercase();
    normalized.starts_with("SELECT")
        || normalized.starts_with("SHOW")
        || normalized.starts_with("WITH")
        || normalized.starts_with("EXPLAIN")
        || normalized.starts_with("PRAGMA")
}

fn preview_redis_query(selected_object: Option<&str>) -> Vec<String> {
    if let Some(key) = selected_object {
        vec![
            "TYPE".to_string(),
            key.to_string(),
            "GET / LRANGE / HGETALL".to_string(),
        ]
    } else {
        vec!["SCAN 0".to_string(), "TYPE <key>".to_string(), "GET <key>".to_string()]
    }
}

async fn save_sql_cell_edit(
    connection: &Connection,
    dialect: SqlDialect,
    table_name: String,
    columns: Vec<String>,
    original_row: Vec<String>,
    column_index: usize,
    new_value: String,
) -> Result<(), PreviewError> {
    let Some(column_name) = columns.get(column_index) else {
        return Err(PreviewError("Column out of range".to_string()));
    };

    match dialect {
        SqlDialect::MySql => {
            let pool = MySqlPoolOptions::new()
                .max_connections(1)
                .connect(&format!(
                    "mysql://{}:{}@{}:{}/{}",
                    connection.username,
                    connection.password,
                    connection.host,
                    connection.port,
                    connection.database
                ))
                .await
                .map_err(|err| PreviewError(err.to_string()))?;
            let sql = build_update_sql(dialect, &table_name, column_name, &columns, &original_row);
            let mut query = sqlx::query(&sql);
            if new_value == "NULL" {
                query = query.bind(Option::<String>::None);
            } else {
                query = query.bind(new_value.clone());
            }
            for value in &original_row {
                if value != "NULL" {
                    query = query.bind(value.clone());
                }
            }
            query.execute(&pool)
                .await
                .map_err(|err| PreviewError(err.to_string()))?;
        }
        SqlDialect::PostgreSql => {
            let pool = PgPoolOptions::new()
                .max_connections(1)
                .connect(&format!(
                    "postgres://{}:{}@{}:{}/{}",
                    connection.username,
                    connection.password,
                    connection.host,
                    connection.port,
                    connection.database
                ))
                .await
                .map_err(|err| PreviewError(err.to_string()))?;
            let sql = build_update_sql(dialect, &table_name, column_name, &columns, &original_row);
            let mut query = sqlx::query(&sql);
            if new_value == "NULL" {
                query = query.bind(Option::<String>::None);
            } else {
                query = query.bind(new_value.clone());
            }
            for value in &original_row {
                if value != "NULL" {
                    query = query.bind(value.clone());
                }
            }
            query.execute(&pool)
                .await
                .map_err(|err| PreviewError(err.to_string()))?;
        }
        SqlDialect::Sqlite => {
            let options = SqliteConnectOptions::new()
                .filename(&connection.file_path)
                .create_if_missing(false);
            let pool = SqlitePoolOptions::new()
                .max_connections(1)
                .connect_with(options)
                .await
                .map_err(|err| PreviewError(err.to_string()))?;
            let sql = build_update_sql(dialect, &table_name, column_name, &columns, &original_row);
            let mut query = sqlx::query(&sql);
            if new_value == "NULL" {
                query = query.bind(Option::<String>::None);
            } else {
                query = query.bind(new_value.clone());
            }
            for value in &original_row {
                if value != "NULL" {
                    query = query.bind(value.clone());
                }
            }
            query.execute(&pool)
                .await
                .map_err(|err| PreviewError(err.to_string()))?;
        }
    }

    Ok(())
}

fn save_redis_cell_edit(
    connection: &Connection,
    preview: &DataPreview,
    row_index: usize,
    column_index: usize,
    key_types: &[String],
    new_value: &str,
) -> Result<(), PreviewError> {
    if column_index != 2 {
        return Err(PreviewError(
            "Redis editing currently supports only the value column".to_string(),
        ));
    }

    let key = preview
        .rows
        .get(row_index)
        .and_then(|row| row.first())
        .cloned()
        .ok_or_else(|| PreviewError("Redis key row not found".to_string()))?;
    let key_type = key_types
        .get(row_index)
        .cloned()
        .ok_or_else(|| PreviewError("Redis key type not found".to_string()))?;

    if key_type != "string" {
        return Err(PreviewError(
            "Only Redis string values can be edited from the grid".to_string(),
        ));
    }

    let url = format!("redis://{}:{}/", connection.host, connection.port);
    let client = redis::Client::open(url).map_err(|err| PreviewError(err.to_string()))?;
    let mut conn = client
        .get_connection()
        .map_err(|err| PreviewError(err.to_string()))?;

    if !connection.database.is_empty() {
        let _: () = redis::cmd("SELECT")
            .arg(connection.database.clone())
            .query(&mut conn)
            .map_err(|err| PreviewError(err.to_string()))?;
    }

    let _: () = redis::cmd("SET")
        .arg(key)
        .arg(new_value)
        .query(&mut conn)
        .map_err(|err| PreviewError(err.to_string()))?;

    Ok(())
}

fn build_update_sql(
    dialect: SqlDialect,
    table_name: &str,
    target_column: &str,
    columns: &[String],
    original_row: &[String],
) -> String {
    let table = quoted_identifier(dialect, table_name);
    let target = quoted_identifier(dialect, target_column);
    let mut predicates = Vec::new();

    for (index, column) in columns.iter().enumerate() {
        let col = quoted_identifier(dialect, column);
        let value = original_row.get(index).map(String::as_str).unwrap_or("");
        if value == "NULL" {
            predicates.push(format!("{col} IS NULL"));
        } else {
            predicates.push(format!("{col} = ?"));
        }
    }

    let limit_clause = if matches!(dialect, SqlDialect::MySql | SqlDialect::Sqlite) {
        " LIMIT 1"
    } else {
        ""
    };

    format!(
        "UPDATE {table} SET {target} = ? WHERE {}{limit_clause}",
        predicates.join(" AND ")
    )
}

fn quoted_identifier(dialect: SqlDialect, ident: &str) -> String {
    match dialect {
        SqlDialect::MySql => format!("`{}`", ident.replace('`', "``")),
        SqlDialect::PostgreSql | SqlDialect::Sqlite => {
            format!("\"{}\"", ident.replace('"', "\"\""))
        }
    }
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
