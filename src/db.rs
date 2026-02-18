use rusqlite::Connection;

pub fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            pictures TEXT,
            video TEXT,
            concept TEXT,
            collaborators TEXT,
            medium TEXT,
            duration TEXT,
            release TEXT,
            dir TEXT
        );

        CREATE TABLE IF NOT EXISTS about (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            location TEXT,
            link TEXT,
            big_row INTEGER,
            date TEXT
        );

        CREATE TABLE IF NOT EXISTS exhibitions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            till TEXT,
            start_date TEXT,
            link TEXT,
            location TEXT,
            type INTEGER
        );
        ",
    )
}
