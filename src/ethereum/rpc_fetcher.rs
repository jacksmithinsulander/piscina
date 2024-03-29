use serde::{Deserialize};

/// Structure for url list by network
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

/// Structure that holds list of RPC urls and current index 
pub struct Rpc {
    /// URLs of the network.
    pub net_urls: Vec<String>,
    /// Index to track the current URL in use.
    pub url_index: usize,
}

/// Implementation for url list
impl Rpc {
    /// Function for creating a new instance of RPC url list
    /// 
    /// # Arguments
    ///
    /// * `network` - A string slice representing the desired network.
    ///
    /// # Returns
    ///
    /// A new `Rpc` instance configured for the specified network.
    pub fn new(network: &str) -> Rpc { 
        // load json data from external file
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

        // Assign the RPC instance that gets returned
        Rpc { 
            net_urls: urls,
            url_index: 0
        }
    }

    /// Retrieves the next URL from the network URLs list.
    ///
    /// # Returns
    ///
    /// An optional reference to the next URL as a string, 
    /// or `None` if there are no URLs available.
    pub fn get_url(&mut self) -> Option<&String> {
        // Pick the url from the list
        // Doing modulus operation using self.net_urls.len() so that it rotates
        // through the list
        let url = &self.net_urls[self.url_index % self.net_urls.len()];
        
        // Increment index
        self.url_index += 1;
        
        // Returning url
        Some(url)
    }
}

///////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////TESTS/////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::Rpc;

    // Test to fetch one url
    #[test]
    fn test_eth_instance() {
        let mut eth = Rpc::new("eth");
        let expected_url = "https://endpoints.omniatech.io/v1/eth/mainnet/public";
        assert_eq!(Some(&expected_url.to_string()), eth.get_url());
    }

    // Test to try iterating through urls on the same network    
    #[test]
    fn test_multiple_instances() {    
        let mut eth = Rpc::new("eth");
        
        let expected_url_1 = "https://endpoints.omniatech.io/v1/eth/mainnet/public";
        let expected_url_2 = "https://eth.llamarpc.com";
        let expected_url_3 = "https://rpc.ankr.com/eth";

        assert_eq!(Some(&expected_url_1.to_string()), eth.get_url());
        assert_eq!(Some(&expected_url_2.to_string()), eth.get_url());
        assert_eq!(Some(&expected_url_3.to_string()), eth.get_url());
    }

    // Test to try multiple instances at the same time from different networks
    #[test]
    fn test_different_instances() {
        let mut eth = Rpc::new("eth");
        let mut matic = Rpc::new("matic");
        let mut ftm = Rpc::new("ftm");

        let eth_url_1 = "https://endpoints.omniatech.io/v1/eth/mainnet/public";
        let eth_url_2 = "https://eth.llamarpc.com";
        
        let matic_url_1 = "https://polygon-bor.publicnode.com";
        let matic_url_2 = "https://polygon.blockpi.network/v1/rpc/public";
        
        let ftm_url_1 = "https://rpcapi.fantom.network";
        let ftm_url_2 = "https://endpoints.omniatech.io/v1/fantom/mainnet/public";

        assert_eq!(Some(&eth_url_1.to_string()), eth.get_url());
        assert_eq!(Some(&eth_url_2.to_string()), eth.get_url());
        
        assert_eq!(Some(&matic_url_1.to_string()), matic.get_url());
        assert_eq!(Some(&matic_url_2.to_string()), matic.get_url());
        
        assert_eq!(Some(&ftm_url_1.to_string()), ftm.get_url());
        assert_eq!(Some(&ftm_url_2.to_string()), ftm.get_url());
    }

    // Test to see if instance fails as expected
    #[test]
    fn test_failed_instance() {
        let mut eth = Rpc::new("eth");
        let mut matic = Rpc::new("matic");
        let mut ftm = Rpc::new("ftm");

        let eth_url_1 = "https://endpoints.omniatech.io/v1/eth/mainnet/public";
        let eth_url_2 = "https://eth.llamarpc.com";
        
        let matic_url_1 = "https://polygon-bor.publicnode.com";
        let matic_url_2 = "https://polygon.blockpi.network/v1/rpc/public";
        
        let ftm_url_1 = "https://rpcapi.fantom.network";

        assert_ne!(Some(&eth_url_2.to_string()), eth.get_url());
        assert_ne!(Some(&matic_url_2.to_string()), eth.get_url());
        
        assert_ne!(Some(&matic_url_2.to_string()), matic.get_url());
        assert_ne!(Some(&matic_url_1.to_string()), matic.get_url());
        
        assert_ne!(Some(&eth_url_1.to_string()), ftm.get_url());
        assert_ne!(Some(&ftm_url_1.to_string()), ftm.get_url());
    }
}
