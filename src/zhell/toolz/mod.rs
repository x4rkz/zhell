pub trait ToolzTrait {
	fn exec_cmd(&self, String) -> ();
}

use self::local_netz::*;
pub mod local_netz;

pub enum Toolz {
	LocalNetz(LocalNetz),
}

impl ToolzTrait for Toolz {
	fn exec_cmd(&self, cmd: String) {
		match self {
			&Toolz::LocalNetz(ref x) => x.exec_cmd(cmd),
		}
	}
}

use std::collections::HashMap;

pub type VecToolz = HashMap<String, Toolz>;

lazy_static! {
	pub static ref VEC_TOOLZ: HashMap<String, Toolz> = {
		let mut m = HashMap::new();
		m.insert(String::from("local_netz"),
				Toolz::LocalNetz(LocalNetz::new()));
		m
	};
}