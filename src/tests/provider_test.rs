
mod test {
    use ethers::prelude::*;
    use crate::ethereum::rpc_fetcher::Rpc;
    
    #[test]
    fn test_import() {
        let mut eth = Rpc::new("eth");
        let expected_url = "https://endpoints.omniatech.io/v1/eth/mainnet/public";
        assert_eq!(Some(&expected_url.to_string()), eth.get_url());
    }
    
    #[tokio::test]
    async fn test_get_eth_data() {    
        let mut eth = Rpc::new("bnb");
        let url: &str = eth.get_url().expect("Could not fetch url");
        let eth_provider = Provider::<Http>::try_from(url);
        let block = eth_provider.expect("Couldnt fetch block").get_block_number().await.unwrap();
        let expected_block_number = U64::from(35000000);
        assert!(block > expected_block_number);
    }
}
