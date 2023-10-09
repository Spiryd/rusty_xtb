//pub mod models;

use std::{thread, time};

use rusty_xtb::{XApi, models::TradeTransInfo};
use dotenv::dotenv;


fn main() {
    //loading the environment variables
    dotenv().ok();
    //connecting to API socket
    let mut api = XApi::new();
    //logging in
    api.login();
    //get symbol data
    let data = api.get_symbol("PLTR.US_4");
    //let data = api.get_all_symbols();
    println!("Received: {:?}", data);
    let buy = TradeTransInfo::simple_buy("EURUSD".to_string(), 0.01, 1.05);
    let order_id = api.make_trade(buy);
    api.get_trade_status(order_id);
    //logging out
    api.logout();
}
