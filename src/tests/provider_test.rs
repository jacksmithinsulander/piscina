mod test {
    use ethers::providers::{Provider, Http};
    use crate::ethereum::rpc_fetcher::Rpc;
    
    #[test]
    fn test_import() {
        let mut eth = Rpc::new("eth");
        let expected_url = "https://endpoints.omniatech.io/v1/eth/mainnet/public";
        assert_eq!(Some(&expected_url.to_string()), eth.get_url());
    }
    
    // #[test]
    // async fn test_get_eth_data() {    
        // let mut eth = Rpc::new("eth");
        // let url: &str = eth.get_url();
        // let eth_provider = Provider::<Http>::try_from(url);
        // let ens = "vitalik.eth";
        // let address_from_ens = eth_provider.resolve_name(ens).await;
        // let address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
        // assert_eq!(address_from_ens, address);
    // }
}
