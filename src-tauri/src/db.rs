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

