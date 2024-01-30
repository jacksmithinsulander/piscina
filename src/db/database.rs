use actix_web::{ HttpServer,
    App,
    HttpResponse,
    web };
use serde::{ Serialize, Deserialize };
use sqlx::mysql::{ MySqlConnection, MySqlPool, MySqlPoolOptions, MySqlQueryResult, MySqlRow };
use sqlx::{FromRow, Connection};


#[derive(Serialize, Deserialize)]
struct LiquidityPool {
    uid: i32,
    chain: str,
    time_of_creation: i32,
    token_a_name: str,
    token_a_symbol: str,
    token_a_amount: i32,
    token_a_price: i32,
    token_b_name: str,
    token_b_symbol: str,
    token_b_amount: i32,
    token_b_price: i32,
}

#[derive(Serialize, Deserialize)]
struct DeletePairBody {
    uid: i32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    const DB_URL: &str = "mysql://user:password@127.0.0.1:3306/sqlx";
    
    let pool: MySqlPool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DB_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(root))
    }).bind(("127.0.0.1", 8000))?
        .run()
        .await
}

async fn root() -> String {
    "Server is up and running".to_string()
}

async fn get_pair() {}

async fn add_pair(body: web::Json<LiquidityPool>, app_state: web::Data<AppState>) -> HttpResponse {}

async fn delete_pair() {}