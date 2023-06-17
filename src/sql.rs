use rusqlite::{Connection, Result};

pub fn connect() -> Option<Connection> {
    if let Ok(conn) = Connection::open("./data/notes.db"){
        return Some(conn);
    }   else {
        return None;
    }
}

    // conn.execute(
    //     "create table if not exists cat_colors (
    //          id integer primary key,
    //          name text not null unique
    //      )",
    //     NO_PARAMS,
    // )?;