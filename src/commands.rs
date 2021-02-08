use json;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};

/// PARAMS: name = the url to the repository.
/// runs a 'git clone' command.
/// clones the repo into .gee/tmp/, and
/// it also logs the output to .gee/log.txt
pub fn clone_repo(name: &str) -> std::io::Result<()> {
	let mut dir = String::from(".gee/tmp/");
	dir.push_str(name);
	let process = match Command::new("git")
		.args(&["clone", "--progress", name, &dir])
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.output()
	{
		Err(why) => panic!("error executing process: {}", why),
		Ok(process) => process,
	};
	// records this clone in .gee/config.json
	write_to_json(name);
	// checks the exit status code for success
	if process.status.success() {
		println!("success.")
	} else {
		println!("error.");
		// assigns the process's output to string, logs and prints the error
		let output = String::from_utf8_lossy(&process.stderr);
		let mut file = File::create(".gee/logs.txt")?;
		file.write_all(output.as_bytes())?;
		show_logs();
	}
	Ok(())
}

/// PARAMS: name = the name of the repository being recorded.
/// records meta data about the repository, and
/// writes it to .gee/config.json
fn write_to_json(name: &str) {
	let mut data = json::JsonValue::new_object();
	data["repositories"] = name.into();
}

/// PARAMS: none.
/// runs a 'cat' command, and
/// prints the output of .gee/log.txt
fn show_logs() {
	println!("\nshowing logs below: ");
	let mut output = String::new();
	let process = match Command::new("cat")
		.arg(".gee/logs.txt")
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
	{
		Err(why) => panic!("error executing process: {}", why),
		Ok(process) => process,
	};
	match process.stdout.unwrap().read_to_string(&mut output) {
		Err(why) => panic!("error reading output: {}", why),
		Ok(_) => print!("{}", output),
	}
}
