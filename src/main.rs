// use provider_factory::test_import;
mod ethereum;
mod tests;
mod database;

fn main() {
    println!("Hello World!!!");
    database::db::main();
}