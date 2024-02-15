mod api;

use api::server::{
    get_latest_pairs
};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .service(get_latest_pairs)
            //.service(start_scanner)
            //.service(stop_scannner)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}