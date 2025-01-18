use std::{collections::HashMap, sync::Arc};

use actix_ws::{Message as WsMessage, Session};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, RwLock};

#[derive(Deserialize, Serialize)]
pub struct Chat {
    name: String,
    message: String,
    time: DateTime<Utc>,
}

pub async fn handle_incoming_messages(
    mut stream: actix_ws::MessageStream,
    client_id: String,
    tx: broadcast::Sender<String>,
    clients: Arc<RwLock<HashMap<String, Session>>>,
) {
    while let Some(Ok(msg)) = stream.recv().await {
        match msg {
            WsMessage::Text(text) => {
                if let Err(err) = tx.send(text.to_string()) {
                    eprintln!("Failed to broadcast message: {:?}", err);
                }
            }
            WsMessage::Close(_reason) => {
                break;
            }
            WsMessage::Ping(bytes) => {
                let mut clients = clients.write().await;
                if let Some(client) = clients.get_mut(&client_id) {
                    if let Err(err) = client.pong(&bytes).await {
                        eprintln!("Failed to send response: {:?}", err);
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    clients.write().await.remove(&client_id);
}

pub async fn handle_outgoing_messages(
    mut rx: broadcast::Receiver<String>,
    mut session: Session,
    history: Arc<RwLock<Vec<Chat>>>,
) {
    while let Ok(msg) = rx.recv().await {
        if let Ok(chat) = serde_json::from_str::<Chat>(&msg) {
            let json = serde_json::to_string(&chat).unwrap();

            if session.text(json).await.is_err() {
                break;
            }

            history.write().await.push(chat);
        }
    }
}
