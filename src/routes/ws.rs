use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::{handlers, AppState};

pub async fn chat(
    req: HttpRequest,
    body: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, body)?;

    let client_id = Uuid::new_v4().to_string();
    state
        .clients
        .write()
        .await
        .insert(client_id.clone(), session.clone());

    let history = state.history.read().await;
    for chat_msg in history.iter() {
        let json = serde_json::to_string(chat_msg).unwrap();
        session.text(json).await.unwrap();
    }

    let rx = state.tx.subscribe();
    let tx = state.tx.clone();
    let clients = state.clients.clone();
    let history = state.history.clone();

    actix_web::rt::spawn(handlers::chat::handle_incoming_messages(
        stream,
        client_id.clone(),
        tx,
        clients,
    ));
    actix_web::rt::spawn(handlers::chat::handle_outgoing_messages(
        rx, session, history,
    ));

    Ok(res)
}
