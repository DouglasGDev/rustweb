use actix_web::HttpRequest;
use chrono::Local;

pub async fn log_access(req: &HttpRequest, action: &str) {
    // Obtém o endereço IP do cliente
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Ação: {} | IP: {} | Horário: {}", action, peer_addr, timestamp);
    }
}

pub async fn log_not_found(req: &HttpRequest) {
    // Obtém o endereço IP do cliente e a rota que resultou em erro 404
    if let Some(peer_addr) = req.connection_info().realip_remote_addr() {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let path = req.uri().path();
        println!(
            "Erro 404: Rota não encontrada | IP: {} | Rota: {} | Horário: {}",
            peer_addr, path, timestamp
        );
    }
}
