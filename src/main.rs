use actix_web::{
    cookie::{Cookie, time::Duration},
    web, App, HttpServer, Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::{LocalBoxFuture, Ready, ready}; // Usando futures_util::future::ready
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::task::{Context, Poll};
use handlers::{captcha::*, login::*, main_page::*, not_found::*, ping::*, status::*};

mod handlers;
mod utils;

// Armazena as sessões com o tempo de última atividade
type SessionStore = Arc<Mutex<HashMap<String, SystemTime>>>;

// Define o tempo de expiração da sessão em segundos
const SESSION_TIMEOUT: u64 = 300;

/// Middleware para atualizar sessões e remover sessões expiradas
struct SessionMiddleware {
    sessions: web::Data<SessionStore>,
}

impl<S, B> Transform<S, ServiceRequest> for SessionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = SessionMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionMiddlewareService {
            service,
            sessions: self.sessions.clone(),
        }))
    }
}

struct SessionMiddlewareService<S> {
    service: S,
    sessions: web::Data<SessionStore>,
}

impl<S, B> Service<ServiceRequest> for SessionMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let sessions = self.sessions.clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            if let Some(cookie) = res.request().cookie("session_id") {
                let session_id = cookie.value().to_string();
                let mut sessions = sessions.lock().unwrap();
                // Atualiza o tempo da última atividade para a sessão
                sessions.insert(session_id.clone(), SystemTime::now());

                // Remove sessões expiradas
                sessions.retain(|_, &mut last_access| {
                    SystemTime::now()
                        .duration_since(last_access)
                        .unwrap_or_default()
                        .as_secs()
                        < SESSION_TIMEOUT
                });
            }

            Ok(res)
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cria um HashMap para armazenar sessões
    let sessions: SessionStore = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sessions.clone()))
            .wrap(SessionMiddleware {
                sessions: web::Data::new(sessions.clone()),
            })
            .route("/", web::get().to(captcha_page)) // Rota para a página de CAPTCHA
            .route("/validate_captcha", web::post().to(validate_captcha)) // Rota para validação do CAPTCHA
            .route("/login", web::get().to(login_page)) // Rota para o login
            .route("/login", web::post().to(login)) // Rota para o login
            .route("/main", web::get().to(main_page)) // Rota para a página principal
            .route("/ping", web::get().to(ping)) // Rota para ping
            .route("/status", web::get().to(status)) // Rota para verificar status do usuário
            .default_service(web::route().to(not_found_page)) // Rota padrão para páginas não encontradas
    })
    .bind("192.168.3.43:80")?
    .run()
    .await
}
