use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Networks {
    eth: Vec<String>,
    matic: Vec<String>,
    arb: Vec<String>,
    op: Vec<String>,
    ftm: Vec<String>,
    avax: Vec<String>,
    metis: Vec<String>,
    harmony: Vec<String>,
    pulsechain: Vec<String>,
    bnb: Vec<String>,
    boba: Vec<String>
}

fn main() {
    // Read the file contents
    let json_data = include_str!("../data/rpc.json");

    // Deserialize JSON into Networks struct
    let networks: Networks = serde_json::from_str(json_data).expect("JSON parsing failed");

    println!("ETH networks: {:?}", networks.eth);
}
