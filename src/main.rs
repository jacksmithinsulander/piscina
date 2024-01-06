// use provider_factory::test_import;
mod ethereum;
use ethereum::provider_factory;

fn main() {
    let network: &str = "boba";
    provider_factory::test_import(network);
}