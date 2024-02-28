use std::io::BufRead;

use reqwest::Client;
use serde_json::json;

const ADDRESS_SEND: &str = "http://127.0.0.1:8080/send";
const ADDRESS_RECV: &str = "http://127.0.0.1:8080/recvhost";

#[tokio::main]
async fn main() {
	let client = Client::new();
	println!("Host");
	println!("[+] Connection established!");

	for line in std::io::stdin().lock().lines() {
		let msg = line.expect("Failed to read line from stdin");
		if msg.is_empty() || msg == "\u{1A}" { break; }
		send_msg(msg, &client).await.unwrap();
		let _ = recv_msg(&client).await;
	}

	println!("[x] Socket closed!");
}

async fn send_msg(msg: String, client: &Client) -> Result<(), reqwest::Error> {
	let json = json!({
        "message": msg
    });
	let res = client.post(ADDRESS_SEND).json(&json).send().await?;
	println!("Sended, response status: {}", res.status());
	Ok(())
}

async fn recv_msg(client: &Client) -> Result<(), reqwest::Error> {
	let res = client.get(ADDRESS_RECV).send().await?;
	if res.status() == 200 {
		let data = res.text().await?;
		println!("Received: {}", data);
	}
	Ok(())
}
