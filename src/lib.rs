pub mod models;

use indoc::formatdoc;
use models::*;
use tungstenite::{connect, Message};
use url::Url;

pub struct XApi {
    socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
}

impl Default for XApi {
    /// Defaults to Demo for safety
    fn default() -> Self {
        Self::new(false)
    }
}

impl XApi {
    /// Creates a new socket and connects it to the API.
    /// Takes in a Boolean value corresponding to the type of the account `true` for a Real account, `false` for a Demo
    pub fn new(real: bool) -> Self {
        let connection_type = if real { "real" } else { "demo" };
        let (socket, response) =
            connect(Url::parse(&format!("wss://ws.xtb.com/{connection_type}")).unwrap())
                .expect("Can't connect");
        println!("Response HTTP code: {}", response.status());
        XApi { socket }
    }

    /// Logs In to an Account using provided `userid` and `password`
    pub fn login(&mut self, userid: &str, password: &str) {
        let req = formatdoc! {r#"
            {{
                "command" : "login",
                "arguments" : {{
                    "userId" : "{userid}",
                    "password": "{password}"
                }}
            }}"#, userid=userid, password=password,};
        self.socket.send(Message::Text(req)).unwrap();
        let msg = self
            .socket
            .read()
            .expect("Error reading message")
            .to_string();
        println!("Received: {:?}", &msg);
    }

    /// Logs Out of the Account
    pub fn logout(&mut self) {
        let req = r#"{
            "command": "logout"
        }"#;
        self.socket.send(Message::Text(req.to_string())).unwrap();
        let msg = self
            .socket
            .read()
            .expect("Error reading message")
            .to_string();
        println!("{}", &msg);
    }

    /// Sends a request to get Symbol information for all the symbols to the API and if succesful returns [`SymbolRecord`]
    pub fn get_all_symbols(&mut self) -> Vec<SymbolRecord> {
        let req = r#"{
            "command": "getAllSymbols"
        }"#;
        self.socket.send(Message::Text(req.to_string())).unwrap();
        let msg = self
            .socket
            .read()
            .expect("Error reading message")
            .to_string();
        let data: SymbolResults = serde_json::from_str(&msg).unwrap();
        data.return_data
    }

    /// Sends a request to get Symbol information about the provided `ticker` to the API and if succesful returns [`SymbolRecord`]
    pub fn get_symbol(&mut self, ticker: &str) -> SymbolRecord {
        let req = formatdoc! {r#"{{
            "command": "getSymbol",
            "arguments": {{
                "symbol": "{ticker}"
            }}
        }}"#, ticker=ticker};
        self.socket.send(Message::Text(req.to_string())).unwrap();
        let msg = self
            .socket
            .read()
            .expect("Error reading message")
            .to_string();
        dbg!(&msg);
        let data: SymbolResult = serde_json::from_str(&msg).unwrap();
        data.return_data
    }

    /// Sends a request to make a transaction according to the provided [`TradeTransInfo`] to the API and if succesful returns the order id
    pub fn make_trade(&mut self, trade_trans_info: TradeTransInfo) -> i32 {
        let req = formatdoc! {r#"{{
            "command": "tradeTransaction",
            "arguments": {{
                "tradeTransInfo": {tti}
            }}
        }}"#, tti=serde_json::to_string(&trade_trans_info).unwrap()};
        self.socket.send(Message::Text(req.to_string())).unwrap();
        let msg = self
            .socket
            .read()
            .expect("Error reading message")
            .to_string();
        dbg!(&msg);
        let data: TradeTransResult = serde_json::from_str(&msg).unwrap();
        data.return_data.order
    }

    /// Sends a request to get the status of your transaction
    pub fn get_trade_status(&mut self, order_id: i32) {
        let req = formatdoc! {r#"{{
            "command": "tradeTransactionStatus",
            "arguments": {{
                "order": "{order_id}"
            }}
        }}"#, order_id=order_id};
        self.socket.send(Message::Text(req.to_string())).unwrap();
        let msg = self
            .socket
            .read()
            .expect("Error reading message")
            .to_string();
        dbg!(&msg);
        println!(
            "{:?}",
            serde_json::from_str::<TradeTransStatusResoult>(&msg).unwrap()
        );
    }
}
