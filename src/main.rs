use crate::db::database::{create_lp, delete_lp, get_lp_count, init_lp_database, read_lp, update_lp, LiquidityPool};

mod api;
mod db;
mod ethereum;

use api::server::{
    get_latest_pairs
};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello World!!!");
    init_lp_database();

    let liquidity_pool_1 = LiquidityPool {
        uid: 1,
        chain: String::from("Ethereum"),
        time_of_creation: 123456789,
        token_a_name: String::from("TokenA"),
        token_a_symbol: String::from("TOKA"),
        token_a_amount: 100,
        token_a_price: 50,
        token_b_name: String::from("TokenB"),
        token_b_symbol: String::from("TOKB"),
        token_b_amount: 200,
        token_b_price: 25,
    };

    create_lp(&liquidity_pool_1);

    println!("LP count: {:?}", get_lp_count());

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .service(get_latest_pairs)
            //.service(start_scanner)
            //.service(stop_scannner)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}