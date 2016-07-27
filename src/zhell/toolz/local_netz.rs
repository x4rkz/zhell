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
	fn exec_cmd(&self, cmd: &str) {
		if cmd.starts_with("ping ") {
			LocalNetz::ping(&cmd[5..cmd.len()]);
		}	
		else if  cmd.starts_with("scan ") {
			self.scan(&cmd[5..cmd.len()]);
		}
		else if cmd == "pingall" {
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

