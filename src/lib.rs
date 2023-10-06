mod models;

use models::{SymbolRecord, SymbolResult, TradeTransInfo};
use tungstenite::{connect, Message};
use url::Url;
use std::env;
use indoc::formatdoc;

pub struct XApi{
    socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
}

impl Default for XApi {
    fn default() -> Self {
        Self::new()
    }
}

impl XApi {
    pub fn new() -> Self {
        let (socket, response) = connect(Url::parse("wss://ws.xtb.com/real").unwrap()).expect("Can't connect");
        println!("Response HTTP code: {}", response.status());
        XApi { socket }
    }
    
    pub fn login(&mut self) {
        let userid = env::var("USER_ID").expect("You've not set the USER_ID");
        let password = env::var("PASSWORD").expect("You've not set the PASSWORD");
        let req = formatdoc!{r#"
            {{
                "command" : "login",
                "arguments" : {{
                    "userId" : "{userid}",
                    "password": "{password}"
                }}
            }}"#, userid=userid, password=password,};
        self.socket.send(Message::Text(req)).unwrap();
        let msg = self.socket.read().expect("Error reading message").to_string();
        println!("Received: {:?}", &msg);
    }

    pub fn logout(&mut self) {
        let req = r#"{
            "command": "logout"
        }"#;
        self.socket.send(Message::Text(req.to_string())).unwrap();
        let msg = self.socket.read().expect("Error reading message").to_string();
        println!("{}", &msg);
    }

    pub fn get_all_symbols(&mut self) -> Vec<SymbolRecord> {
        let req = r#"{
            "command": "getAllSymbols"
        }"#;
        self.socket.send(Message::Text(req.to_string())).unwrap();
        let msg = self.socket.read().expect("Error reading message").to_string();
        let data: SymbolResult = serde_json::from_str(&msg).unwrap();
        data.return_data
    }

    pub fn get_symbol(&mut self, ticker: String) -> Vec<SymbolRecord> {
        let req = formatdoc!{r#"{{
            "command": "getAllSymbols"
            "arguments": {{
                "symbol": "{ticker}"
            }}
        }}"#, ticker=ticker};
        self.socket.send(Message::Text(req.to_string())).unwrap();
        let msg = self.socket.read().expect("Error reading message").to_string();
        let data: SymbolResult = serde_json::from_str(&msg).unwrap();
        data.return_data
    }

    pub fn make_trade(&mut self, trade_trans_info: TradeTransInfo) -> i32 {
        todo!()
    }

    pub fn get_trade_status(&mut self) {
        todo!()
    }
}
