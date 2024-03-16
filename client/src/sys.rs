use std::process::Command;

use sysinfo::System;

pub async fn run_command(command: String) {
	let mut parts = command.split_whitespace();
	let cmd = match parts.next() {
		Some(cmd) => cmd,
		None => return,
	};
	let child = match Command::new(cmd).args(parts).spawn() {
		Ok(child) => child,
		Err(e) => {
			return;
		}
	};

	// if let Some(mut stdout) = child.stdout {
	// 	let mut output = Vec::new();
	// 	if stdout.read_to_end(&mut output).is_ok() {
	// 		let _ = send_stdout(String::from_utf8_lossy(&output).to_string(), client).await;
	// 	}
	// }
}

pub fn close_processes(names: &Vec<String>) {
	let mut sys = System::new();
	sys.refresh_all();
	for (_, proc) in sys.processes() {
		if names.contains(&proc.name().to_string()) { proc.kill(); }
	}
}