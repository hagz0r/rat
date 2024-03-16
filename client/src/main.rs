// #![windows_subsystem = "windows"]

use std::thread::sleep;
use std::time::Duration;

use network::*;

mod network;
mod sys;

#[tokio::main]
async fn main() {
	println!("Client");
	loop {
		sleep(Duration::from_secs(1));
		NetworkInterface::start().await;
	}
}


