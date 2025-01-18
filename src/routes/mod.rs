use actix_web::web;

mod ws;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/ws", web::get().to(ws::chat));
}
