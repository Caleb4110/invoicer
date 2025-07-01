// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use inv_tools::{args::NewClientArgs, database, models::Client};
use rusqlite::Connection;
use tauri::State;

// Wrap connection in a struct to share via State
struct DbConn(std::sync::Mutex<Connection>);

#[tauri::command]
fn all_clients(db: State<DbConn>) -> Vec<Client> {
    let conn = db.0.lock().unwrap();
    database::list_clients(&conn).unwrap()
}

fn main() {
    // Set up DB connection and initialize database
    let conn = Connection::open("./clinv.db").expect("Could not open DB");
    database::init_db(&conn).expect("Could not initialize DB");
    let db_conn = DbConn(std::sync::Mutex::new(conn));

    tauri::Builder::default()
        .manage(db_conn)
        .invoke_handler(tauri::generate_handler![all_clients])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
