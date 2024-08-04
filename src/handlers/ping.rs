use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use crate::SessionStore;

/// Handler para a rota /ping
pub async fn ping(req: HttpRequest, sessions: web::Data<SessionStore>) -> impl Responder {
    // Loga o IP de quem faz a requisição de ping
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        println!("Ping recebido de IP: {}", peer_addr);
    }

    // Verifica se a sessão está ativa
    if let Some(cookie) = req.cookie("session_id") {
        let session_id = cookie.value().to_string();
        let sessions = sessions.lock().unwrap();

        // Se a sessão está no HashMap, o usuário está online
        if sessions.contains_key(&session_id) {
            return HttpResponse::Ok().body("pong");
        }
    }

    // Se não encontrar a sessão, o usuário não está online
    HttpResponse::Unauthorized().body("Não autorizado.")
}
