use json;
use std::fs::File;
use std::process::{Command, Stdio};
use std::io::prelude::*;

pub fn clone_repo(name: &str) -> std::io::Result<()> {
	let mut dir = String::from(".gee/tmp/");
	dir.push_str(name);
	let mut output = String::new();
	let process = match Command::new("git")
		.args(&["clone", "--progress", name, &dir])
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn() {
			Err(why) => panic!("error executing process: {}", why),
			Ok(process) => process 
	};
	match process.stderr.unwrap().read_to_string(&mut output) {
		Err(why) => panic!("error reading output: {}", why),
        Ok(_) => (print!("\n")),
	}
	write_to_json(name);
	let mut file = File::create(".gee/logs.txt")?;
	file.write_all(output.as_bytes())?;
	show_logs();
    Ok(())
}

fn write_to_json(name: &str) {
	let mut data = json::JsonValue::new_object();
	data["repositories"] = name.into();
	println!("{}", data);
}

fn show_logs() {
	println!("\nshowing logs below: \n");
	let mut output = String::new();
	let process = match Command::new("cat")
		.arg(".gee/logs.txt")
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn() {
			Err(why) => panic!("error executing process: {}", why),
			Ok(process) => process
	};
	match process.stdout.unwrap().read_to_string(&mut output) {
		Err(why) => panic!("error reading output: {}", why),
        Ok(_) => print!("{}", output),
	}
}