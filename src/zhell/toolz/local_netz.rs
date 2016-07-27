use std::process::Command;

pub struct LocalNetz {

}

impl LocalNetz {
	pub fn new() -> LocalNetz {
		LocalNetz {

		}
	}

	pub fn scan(&self, addr: &str) {
		let (ip, mask) = {
		 	let mut ipmask = addr.split("/"); 
			(ipmask.next().unwrap(), ipmask.next().unwrap()) 
		};
		println!("{}", ip);
		println!("{}", mask);
	}

	pub fn ping(addr: &str) {
		Command::new("ping").arg(addr).spawn().unwrap()
			.wait().unwrap();
	}

	pub fn ping_all(&self) {

	}
}

use ansi_term::Colour::*;
impl super::ToolzTrait for LocalNetz {
	fn exec_cmd(&self, cmd: String) {
		if cmd.len() >= 5 && cmd[0..5] == "ping ".to_string() {
			LocalNetz::ping(&cmd[5..cmd.len()]);
		}	
		else if  cmd.len() >= 5 && cmd[0..5] == "scan ".to_string() {
			self.scan(&cmd[5..cmd.len()]);
		}
		else if cmd[0..cmd.len()] == "pingall".to_string() {
			self.ping_all();
		}
		else {
			println!("{}", 
				Red.paint("LocalNetz: '".to_string()
					+ &cmd
					+ "' command not found !"));
		}
	}
}

