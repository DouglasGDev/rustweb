use actix_web::{cookie::Cookie, web, HttpRequest, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::SessionStore;

/// Exibe a página de login
pub async fn login_page(req: HttpRequest, sessions: web::Data<SessionStore>) -> impl Responder {
    // Loga o IP de quem acessa a página de login
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        println!("Acesso à página de login por IP: {}", peer_addr);
    }

    // Verifica se a sessão já está ativa
    if let Some(cookie) = req.cookie("session_id") {
        let session_id = cookie.value().to_string();
        let sessions = sessions.lock().unwrap();
        if sessions.contains_key(&session_id) {
            // Redireciona para a página principal se a sessão estiver ativa
            return HttpResponse::Found()
                .header("Location", "/main")
                .finish();
        }
    }

    // Carrega o HTML do login
    let html = include_str!("../../static/login.html").to_string();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

/// Valida o login do usuário
pub async fn login(
    form: web::Form<HashMap<String, String>>,
    sessions: web::Data<SessionStore>,
) -> impl Responder {
    // Simula um login sem validação real de credenciais

    // Obtem o timestamp atual de forma segura
    let session_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());

    // Armazena a sessão
    {
        let mut sessions = sessions.lock().unwrap();
        sessions.insert(session_id.clone(), SystemTime::now());
    }

    // Define um cookie de sessão
    let cookie = Cookie::build("session_id", session_id)
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(300))
        .finish();

    // Redireciona para a página principal
    HttpResponse::Found()
        .header("Location", "/main")
        .cookie(cookie)
        .finish()
}
