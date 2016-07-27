pub trait ToolzTrait {
	fn exec_cmd(&self, &str) -> ();
}

use self::local_netz::*;
pub mod local_netz;

pub enum Toolz {
	LocalNetz(LocalNetz),
}
/*
macro_rules! exec_cmd {

}*/

impl ToolzTrait for Toolz {
	fn exec_cmd(&self, cmd: &str) {
		match self {
			&Toolz::LocalNetz(ref x) => x.exec_cmd(cmd),
		}
	}
}

use std::collections::HashMap;

pub type VecToolz = HashMap<&'static str, Toolz>;

lazy_static! {
	pub static ref VEC_TOOLZ: HashMap<&'static str, Toolz> = {
		let mut m = HashMap::new();
		m.insert("local_netz",
				Toolz::LocalNetz(LocalNetz::new()));
		m
	};
}