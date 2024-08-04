use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use crate::SessionStore;

/// Verifica o status do usuário
pub async fn status(req: HttpRequest, sessions: web::Data<SessionStore>) -> impl Responder {
    // Loga o IP de quem verifica o status
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        println!("Verificação de status por IP: {}", peer_addr);
    }

    // Verifica se a sessão está ativa
    if let Some(cookie) = req.cookie("session_id") {
        let session_id = cookie.value().to_string();
        let sessions = sessions.lock().unwrap();
        if sessions.contains_key(&session_id) {
            return HttpResponse::Ok()
                .content_type("text/plain; charset=utf-8")
                .body("Usuário está online.");
        }
    }

    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("Usuário está offline.")
}
