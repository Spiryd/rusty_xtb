use tungstenite::{connect, Message};
use url::Url;
use std::env;
use dotenv::dotenv;
use indoc::formatdoc;

fn main() {
    //loading the environment variables
    dotenv().ok();
    //connecting to API socket
    let (mut socket, response) 
        = connect(Url::parse("wss://ws.xtb.com/real").unwrap()).expect("Can't connect");
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    //logging in
    let userid = env::var("USER_ID").expect("You've not set the USER_ID");
    let password = env::var("PASSWORD").expect("You've not set the PASSWORD");
    let login = formatdoc!{r#"
        {{
            "command" : "login",
            "arguments" : {{
                "userId" : "{userid}",
                "password": "{password}"
            }}
        }}"#, userid=userid, password=password,};
    socket.send(Message::Text(login.into())).unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
