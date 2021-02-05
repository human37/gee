use std::process::Command;


pub fn clone_repo(name: &str) {
	let mut dir = String::from(".gee/tmp/");
	dir.push_str(name);
	Command::new("git")
		.args(&["clone", name, &dir])
		.output()
		.expect("git clone failed");
}
