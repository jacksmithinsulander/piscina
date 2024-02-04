//use crate::db::database::test_persy;

use crate::db::database::{create_lp, init_lp_database, read_lp, LiquidityPool};

// use provider_factory::test_import;
mod ethereum;
mod tests;
mod db;

fn main() {
    println!("Hello World!!!");
    init_lp_database();

    let liquidity_pool = LiquidityPool {
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

    create_lp(&liquidity_pool);

    read_lp(1);
}