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

struct Indexes {
    eth: i32,
    matic: i32,
    arb: i32,
    op: i32,
    ftm: i32,
    avax: i32,
    metis: i32,
    harmony: i32,
    pulsechain: i32,
    bnb: i32,
    boba: i32
}

fn main() {
    // Testprinting
    rpc("eth");
    rpc("avax");
    rpc("pulsechain");
}

fn rpc(chain: &str) {
    // Read the file contents
    let json_data = include_str!("../data/rpc.json");

    // Deserialize JSON into Networks struct
    let networks: Networks = serde_json::from_str(json_data).expect("JSON parsing failed");

    // Init empty Vec for holding rpc urls
    let mut rpc_urls: Vec<String> = Vec::new();

    // Access the rpc urls based on input argument
    match chain {
        "eth" => {
            rpc_urls = networks.eth.clone();
        },
        "matic" => { 
            rpc_urls = networks.matic.clone();
        },
        "arb" => { 
            rpc_urls = networks.arb.clone();
        },
        "op" => {
            rpc_urls = networks.op.clone();
        },
        "ftm" => { 
            rpc_urls = networks.ftm.clone();
        },
        "avax" => { 
            rpc_urls = networks.avax.clone();
        },
        "metis" => { 
            rpc_urls = networks.metis.clone();
        },
        "harmony" => { 
            rpc_urls = networks.harmony.clone();
        },
        "pulsechain" => { 
            rpc_urls = networks.pulsechain.clone();
        },
        "bnb" => { 
            rpc_urls = networks.bnb.clone();
        },
        "boba" => {
            rpc_urls = networks.boba.clone();
        },
        _ => println!("Chain not found"),
    }

    // Test print
    println!("Network urls found: {:?}", rpc_urls)
}
