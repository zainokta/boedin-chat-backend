use std::{collections::HashMap, sync::Arc};

use actix_web::{web, App, HttpServer};
use tokio::sync::{broadcast, RwLock};

mod handlers;
mod routes;

#[derive(Clone)]
pub struct AppState {
    tx: broadcast::Sender<String>,
    history: Arc<RwLock<Vec<handlers::chat::Chat>>>,
    clients: Arc<RwLock<HashMap<String, actix_ws::Session>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, _) = broadcast::channel::<String>(100);

    let state = web::Data::new(AppState {
        tx: tx.clone(),
        history: Arc::new(RwLock::new(Vec::new())),
        clients: Arc::new(RwLock::new(HashMap::new())),
    });

    println!("Server started at http://0.0.0.0:8080");

    HttpServer::new(move || App::new().app_data(state.clone()).configure(routes::init))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
