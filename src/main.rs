fn main() {
	use std::io;
  	use std::io::Write;

	loop {	
		let mut cmd = String::new();
		print!("zhell> ");
		io::stdout().flush().ok().expect("ERROR FLUSHING!");
		match io::stdin().read_line(&mut cmd) {
		    Ok(0) => break,
		    Ok(_) => {cmd.pop(); exec_cmd(cmd)}
		    Err(error) => println!("error: {}", error),
		}
	}
}

fn exec_cmd(cmd: String) {
	use std::process::Command;

	println!("a{}a", cmd);
	let output = Command::new(cmd).output()
								  .expect("Failed to execute process")
								  .stdout;
	let output = "Lolilol";
	println!("{}", output);
}
