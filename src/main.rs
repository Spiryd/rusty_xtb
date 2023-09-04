mod models;

use rusty_xtb::XApi;
use dotenv::dotenv;

fn main() {
    //loading the environment variables
    dotenv().ok();
    //connecting to API socket
    let mut api = XApi::new();
    //logging in
    api.login();
    //get all symbol data
    let data = api.get_all_symbols();
    println!("Received: {:?}", data);
    //logging out
    api.logout();
}
