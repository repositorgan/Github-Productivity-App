use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("usage.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_sessions (
            id INTEGER PRIMARY KEY,
            provider TEXT,
            tokens_in INTEGER,
            tokens_out INTEGER,
            cost REAL,
            timestamp TEXT
        )",
        [],
    )?;

    Ok(conn)
}

use chrono::Utc;

#[tauri::command]
pub fn log_session(
    provider: String,
    tokens_in: i32,
    tokens_out: i32,
    cost: f64,
) -> Result<(), String> {

    let conn = Connection::open("usage.db")
        .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO ai_sessions (provider, tokens_in, tokens_out, cost, timestamp)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            provider,
            tokens_in,
            tokens_out,
            cost,
            Utc::now().to_rfc3339(),
        ),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

