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

pub struct Rpc {
    net_urls: Vec<String>,
    url_index: usize,
}

impl Rpc {
    pub fn new(network: &str) -> Rpc {
        let json_data = include_str!("../../data/rpc.json");
        
        // Deserialize JSON into Networks struct
        let networks: Networks = serde_json::from_str(json_data).expect("JSON parsing failed");

        // Create instance for url vec
        let urls: Vec<String> = match network {
            "eth" => networks.eth,
            "matic" => networks.matic,
            "arb" => networks.arb,
            "op" => networks.op,
            "ftm" => networks.ftm,
            "avax" => networks.avax,
            "metis" => networks.metis,
            "harmony" => networks.harmony,
            "pulsechain" => networks.pulsechain,
            "bnb" => networks.bnb,
            "boba" => networks.boba,
            _ => {
                println!("Chain not found");
                Vec::new() 
            }
        };

        Rpc { 
            net_urls: urls,
            url_index: 0
        }
    }

    pub fn get_url(&mut self) -> Option<&String> {
        let url = &self.net_urls[self.url_index % self.net_urls.len()];
        self.url_index += 1;
        Some(url)
    }
}

fn main() {
    let mut rpc_instance = Rpc::new("eth");

    // Get the URL twice
    if let Some(url) = rpc_instance.get_url() {
        println!("First URL: {}", url);
    }
    
    if let Some(url) = rpc_instance.get_url() {
        println!("Second URL: {}", url);
    }
}