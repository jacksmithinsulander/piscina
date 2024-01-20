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

async fn add_token()