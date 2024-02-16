//use crate::db::database::test_persy;

use crate::db::database::{create_lp, delete_lp, get_lp_count, init_lp_database, read_lp, update_lp, LiquidityPool};

// use provider_factory::test_import;
mod ethereum;
mod tests;
mod db;

fn main() {
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
    
    let liquidity_pool_2 = LiquidityPool {
        uid: 2,
        chain: String::from("BSC"),
        time_of_creation: 123456739,
        token_a_name: String::from("SHIBA INU"),
        token_a_symbol: String::from("SHIB"),
        token_a_amount: 1000000,
        token_a_price: 50,
        token_b_name: String::from("DOGECOIN"),
        token_b_symbol: String::from("DOGE"),
        token_b_amount: 200000,
        token_b_price: 25,
    };

    let liquidity_pool_3 = LiquidityPool {
        uid: 2,
        chain: String::from("BSC"),
        time_of_creation: 123456739,
        token_a_name: String::from("MONERO"),
        token_a_symbol: String::from("XMR"),
        token_a_amount: 1000000,
        token_a_price: 50,
        token_b_name: String::from("Frogchain"),
        token_b_symbol: String::from("ribbit"),
        token_b_amount: 200000,
        token_b_price: 25,
    };


    create_lp(&liquidity_pool_1);

    println!("LP count: {:?}", get_lp_count());

    create_lp(&liquidity_pool_2);

    println!("LP count: {:?}", get_lp_count());

    println!("LP index 1: {:?}",read_lp(1));

    println!("LP index 2: {:?}", read_lp(2));

    update_lp(2, &liquidity_pool_3);

    println!("LP index 2: {:?}", read_lp(2));

    delete_lp(2);
    
    println!("LP count: {:?}", get_lp_count());

}
