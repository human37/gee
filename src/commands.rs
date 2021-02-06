use json;
use std::process::{Command, Stdio};
use std::io::prelude::*;

pub fn clone_repo(name: &str) {
	let mut dir = String::from(".gee/tmp/");
	dir.push_str(name);
	let mut output = String::new();
	let process = match Command::new("git")
		.args(&["clone", "--progress", name, &dir])
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn() {
			Err(why) => panic!("error with output: {}", why),
			Ok(process) => process 
	};

	match process.stderr.unwrap().read_to_string(&mut output) {
		Err(why) => panic!("error: {}", why),
        Ok(_) => print!("\noutput: {}", output),
	}

	write_to_json(name);
}

fn write_to_json(name: &str) {
	let mut data = json::JsonValue::new_object();
	data["repositories"] = name.into();
	println!("{}", data);
}
