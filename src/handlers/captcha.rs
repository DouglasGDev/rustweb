use actix_web::{cookie::Cookie, web, HttpRequest, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use crate::SessionStore;
use time::Duration;

/// Exibe a página de CAPTCHA
pub async fn captcha_page(req: HttpRequest, sessions: web::Data<SessionStore>) -> impl Responder {
    // Loga o IP de quem acessa a página CAPTCHA
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        println!("Acesso à página CAPTCHA por IP: {}", peer_addr);
    }

    // Verifica se a sessão já está ativa
    if let Some(cookie) = req.cookie("session_id") {
        let session_id = cookie.value().to_string();
        let sessions = sessions.lock().unwrap();
        if sessions.contains_key(&session_id) {
            // Redireciona para a página de login se a sessão estiver ativa
            return HttpResponse::Found()
                .header("Location", "/login")
                .finish();
        }
    }

    // Gera números aleatórios para o CAPTCHA
    let mut rng = rand::thread_rng();
    let a: u32 = rng.gen_range(1..10);
    let b: u32 = rng.gen_range(1..10);
    let sum = a + b;

    // Carrega o HTML do CAPTCHA e substitui os placeholders
    let mut html = include_str!("../../static/captcha.html").to_string();
    html = html.replace("{{ a }}", &a.to_string());
    html = html.replace("{{ b }}", &b.to_string());
    html = html.replace("{{ sum }}", &sum.to_string());

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

/// Valida a resposta do CAPTCHA
pub async fn validate_captcha(
    form: web::Form<HashMap<String, String>>,
    req: HttpRequest,  // Adiciona HttpRequest como argumento
    sessions: web::Data<SessionStore>,
) -> impl Responder {
    // Loga o IP de quem tenta validar o CAPTCHA
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        println!("Tentativa de validação de CAPTCHA por IP: {}", peer_addr);
    }

    // Obtém os valores do CAPTCHA do formulário
    let captcha_value = form.get("captcha_value").unwrap_or(&"0".to_string()).clone();
    let user_input = form.get("captcha").unwrap_or(&"".to_string()).clone();

    // Verifica se o CAPTCHA está correto
    if user_input == captcha_value {
        // Cria uma nova sessão com um ID único baseado no tempo
        let session_id = format!(
            "{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        );

        // Armazena a sessão
        {
            let mut sessions = sessions.lock().unwrap();
            sessions.insert(session_id.clone(), SystemTime::now());
        }

        // Define um cookie de sessão com duração de 5 minutos
        let cookie = Cookie::build("session_id", session_id)
            .path("/")
            .http_only(true)
            .max_age(Duration::seconds(300))  // Usando time::Duration
            .finish();

        // Redireciona para a página de login
        HttpResponse::Found()
            .header("Location", "/login")
            .cookie(cookie)
            .finish()
    } else {
        // Redireciona de volta para o CAPTCHA se a resposta estiver errada
        HttpResponse::Found()
            .header("Location", "/")
            .finish()
    }
}
