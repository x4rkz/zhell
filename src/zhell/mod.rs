use std;
use std::env;
use std::path::Path;
pub static mut child_pid: u32 = 0;

use ansi_term::Colour::*;

use self::toolz::*;
pub mod toolz;

pub struct Zhell {
//	working_directory: String,
}

impl Zhell {
	pub fn new() -> Zhell {
		Zhell {
		/*	working_directory: String::from(
					env::current_dir().unwrap()
					.to_str().unwrap()
				),*/
		}
	}

	pub fn cwd(&self) -> String {
		String::from(env::current_dir().unwrap().to_str().unwrap())
	}

	pub fn run(&self) {
		loop {
			let mut cmd = String::new();
			self.prompt();
			
			match std::io::stdin().read_line(&mut cmd) {
			    Ok(0) => break,
			    Ok(_) => { 
			    	cmd.pop(); // pop the '\n'
			    	self.exec_cmd(&cmd); 
			    }
			    Err(error) => println!("error: {}", error),
			}
		}
	}

	pub fn prompt(&self) {
		use std::io;
	  	use std::io::Write;

		print!("{}:{}> ", Fixed(221).paint("zhell"), 
					Fixed(134).paint(self.cwd()));
		io::stdout().flush().ok().expect("Error flushing!");
	}

	pub fn cmd_match(cmd: &str, name: &str) -> bool {
		cmd.starts_with(&*format!("{} ", name)) 
			|| cmd == name
	}

	pub fn exec_cmd(&self, cmd: &str) {
		for (name, toolz) in &VEC_TOOLZ as &VecToolz {
			if Zhell::cmd_match(cmd, name) {
				toolz.exec_cmd(&cmd[name.len()..cmd.len()].trim());
				return;
			}
		}
		self.bash(cmd);
	}
	

	pub fn bash(&self, cmd: &str) {
	  	let mut c = std::process::Command::new("bash");
	  	c.args(&["-c", cmd]);
		let mut child = c.spawn().expect("Could not run the command");
		unsafe { child_pid = child.id(); }
		child.wait().expect("Command issue");
		unsafe { child_pid  = 0; }
	}
}

//static 

/// builtins
impl Zhell {
	fn cd<'a>(&self, path: &'a str) {
		env::set_current_dir(Path::new(path)).expect("Bug cd");
	}
}



/*
fn split_cmd(cmd: &String) -> Vec<String>{
	let bytes = cmd.as_bytes();
	let mut argv: Vec<String> = Vec::new();

	//let &[space, quote] = " \"".as_bytes();
	let mut begin = 0;
	println!("{}", begin);
	let mut quoted = false;
	for i in 0..bytes.len() {
		let b = bytes[i];
		match b as char {
			'"' => quoted = true,
			' ' => {
				if !quoted {
					argv.push(cmd[begin..i].to_string());
					begin = i+1;
				}
			}
			_ => {}
		}
	}
	argv.push(cmd[begin..cmd.len()].to_string());

	argv
}*/
