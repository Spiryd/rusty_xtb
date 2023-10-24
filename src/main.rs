use rusty_xtb::{XApi, models::TradeTransInfo};
use dotenv::dotenv;
use std::env;

fn main() {
    //loading the environment variables
    dotenv().ok();
    //connecting to API socket
    let mut api = XApi::new(false);
    //logging in
    let userid = env::var("USER_ID").expect("You've not set the USER_ID");
    let password = env::var("PASSWORD").expect("You've not set the PASSWORD");
    api.login(&userid, &password);
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
