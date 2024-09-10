// use data_store::db::DbConnection;
// use tauri::Manager;

mod error;
mod service;
mod module;

mod shared;

// use rusqlite::Connection;
// use std::sync::Mutex;
// use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_| {
            module::feed::model::FeedModel::setup();
            // app.manage(db_connection);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            module::feed::api::fetch_feed,
            module::feed::api::add_feed,
            module::feed::api::get_all_feeds,
            module::feed::api::get_articles_for_feed,
            module::feed::category::api::add_category,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// #[tauri::command]
// async fn display_feed(url: &str) -> Result<serde_json::Value, error::Error> {
//     match feed::fetch_feed(url).await {
//         Ok(content) => feed::bytes_to_json(content),
//         Err(e) => Err(e),
//     }

// }

// struct DbConnection {
//     db: Mutex<Connection>,
// }

// fn setup_database() -> DbConnection {
//     let path = "db.sqlite";
//     if !std::path::Path::exists(std::path::Path::new(&path)) {
//         let connection = Connection::open(&path).expect("error with connection open");
//         connection
//             .execute(
//                 "CREATE TABLE IF NOT EXISTS rss
//                 (
//                     id INTEGER PRIMARY KEY,
//                     url TEXT NOT NULL
//                 )",
//                 (),
//             )
//             .expect("error with create table rss");

//         DbConnection {
//             db: Mutex::new(connection),
//         }
//     } else {
//         let connection = Connection::open(path).expect("error with connection open");
//         // check_database_integrity(&connection).expect("database integrity check failed");
//         DbConnection {
//             db: Mutex::new(connection),
//         }
//     }
// }

// #[tauri::command]
// fn add_feed(state: tauri::State<DbConnection>, url: &str) -> Result<(), ()> {
//     let connection = state
//         .db
//         .lock()
//         .map_err(|e| e.to_string())
//         .expect("error with mutex in add_feed");

//     match connection.execute("INSERT INTO rss (url) VALUES (?1)", [url]) {
//         Ok(_) => println!("Successfully inserted URL: {}", url),
//         Err(e) => {
//             eprintln!("Error inserting URL: {}: {:?}", url, e);
//             panic!("Error with insert into rss: {:?}", e);
//         }
//     }
//     drop(connection);
//     Ok(())
// }

// #[tauri::command]
// fn get_feed(state: tauri::State<DbConnection>) -> Result<Vec<String>, String> {
//     let connection = state.db.lock().map_err(|e| e.to_string())?;
//     let mut stmt = connection
//         .prepare("SELECT url FROM rss")
//         .map_err(|e| e.to_string())?;
//     let rss_iter = stmt
//         .query_map([], |row| row.get(0))
//         .map_err(|e| e.to_string())?;

//     let mut feeds = Vec::new();
//     for feed in rss_iter {
//         feeds.push(feed.map_err(|e| e.to_string())?);
//     }
//     Ok(feeds)
// }
