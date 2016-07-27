/// TO DO: TAB, builtins
/// example toolz, install toollz

#[macro_use]
extern crate lazy_static;


extern crate ansi_term;

use zhell::*;
pub mod zhell;


/// <<Signal handling
extern crate libc;
use libc::sighandler_t;
use libc::{c_int, c_void, SIGINT};
use libc::signal;

extern fn handler(_: c_int) {
	unsafe {
		if child_pid > 0 {
			std::process::Command::new("kill").arg(child_pid.to_string()).spawn().expect("lol");
		}
	}
}

fn get_handler() -> sighandler_t {
    handler as extern fn(c_int) as *mut c_void as sighandler_t
}
/// Signal handling>>


fn main() {
	unsafe{ signal(SIGINT, get_handler()); }

	let zhell = Zhell::new();
	zhell.run();
}
