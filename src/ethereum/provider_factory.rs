use ethers::providers::{Provider, Http};

use crate::ethereum::rpc_fetcher::Rpc;

pub fn test_import(network: &str) {
    let mut rpc = Rpc::new(network);
    
    ///if let Some(url) = rpc.get_url() {
    //    println!("URL: {}", url);
    //} else {
    //    println!("No URL available");
    //}
}