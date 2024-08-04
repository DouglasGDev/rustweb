use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use crate::SessionStore;

/// Exibe a página principal
pub async fn main_page(req: HttpRequest, sessions: web::Data<SessionStore>) -> impl Responder {
    // Loga o IP de quem acessa a página principal
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        println!("Acesso à página principal por IP: {}", peer_addr);
    }

    // Verifica se a sessão está ativa
    if let Some(cookie) = req.cookie("session_id") {
        let session_id = cookie.value().to_string();
        let sessions = sessions.lock().unwrap();
        if sessions.contains_key(&session_id) {
            // Conta o número de usuários online
            let online_users = sessions.len();

            // Carrega o HTML da página principal
            let html = format!(
                r#"
                <!DOCTYPE html>
                <html lang="pt-br">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Página Principal</title>
                    <style>
                        body {{
                            font-family: Arial, sans-serif;
                            text-align: center;
                            background-color: #f0f0f0;
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            height: 100vh;
                        }}
                        .container {{
                            background-color: white;
                            padding: 20px;
                            border-radius: 10px;
                            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
                        }}
                        h1 {{
                            color: #333;
                        }}
                        p {{
                            font-size: 18px;
                        }}
                    </style>
                </head>
                <body>
                    <div class="container">
                        <h1>Bem-vindo à Página Principal!</h1>
                        <p>Usuários online: {}</p>
                    </div>
                </body>
                </html>
                "#,
                online_users
            );

            return HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html);
        }
    }

    // Redireciona para a página de login se a sessão não estiver ativa
    HttpResponse::Found()
        .header("Location", "/login")
        .finish()
}
