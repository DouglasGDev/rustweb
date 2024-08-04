use actix_web::{web, HttpRequest, HttpResponse, Responder};

/// Exibe a página de erro 404
pub async fn not_found_page(req: HttpRequest) -> impl Responder {
    // Loga o IP de quem tenta acessar uma rota inválida
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        println!("Tentativa de acesso a rota inválida por IP: {}", peer_addr);
    }

    // Carrega o HTML da página 404
    let html = include_str!("../../static/404.html").to_string();

    HttpResponse::NotFound()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
