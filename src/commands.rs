use serde::{Deserialize, Serialize};
use std::{
	convert::TryInto, fs::File, io::prelude::*, io::BufRead, io::BufReader, process::Command,
	process::Stdio, path::Path
};

#[derive(Deserialize, Serialize)]
struct Repo {
	url: String,
	index: usize,
}

impl PartialEq for Repo {
	fn eq(&self, other: &Self) -> bool {
		return &self.url == &other.url;
	}
}

struct Config {
	qsize: usize,
}

/// PARAMS: url = the url of the repository being recorded.
/// records meta data about the repository, and
/// writes it to .gee/metadata.json
pub fn write_data(url: &str) -> std::io::Result<()> {
	let mut repositories = read_in_data();
	repositories.push(Repo {
		url: url.to_string(),
		index: repositories.len() + 1,
	});
	let metadata = serde_json::to_string(&repositories)?;
	let mut file = File::create(".gee/metadata.json")?;
	file.write_all(metadata.as_bytes())?;
	Ok(())
}

/// PARAMS: none.
/// reads in the data from metadata.json,
/// and deserializes the data into a vector of 'Repo' structs,
/// and returns the vector.
fn read_in_data() -> Vec<Repo> {
	if file_exists(".gee/metadata.json") {
		let file = File::open(".gee/metadata.json").expect("could not open the file metadata.json");
		let repositories: Vec<Repo> =
			serde_json::from_reader(file).expect("could not deserialize metadata.json");
		return repositories
	} else {
		File::create(".gee/metadata.json").expect("could not open the file metadata.json");
		let repositories: Vec<Repo> = Vec::new();
		return repositories
	}
}

fn manage_repo_installations(name: &str) {
	let config: Config = parse_conf();
	if file_exists(".gee/metadata.json") {
		let repositories: Vec<Repo> = read_in_data();
		let repo = Repo {
			url: name.to_string(),
			index: 0,
		};
		if repositories.contains(&repo) {
			println!("already have this repository cloned!\n");
			return
		}
		if repositories.len() >= config.qsize.try_into().unwrap() {
			remove_repo(&repositories[0].url).expect("failed to remove repository");
			println!("just removed {}\n", repositories[0].url)
		}
	}
}

/// PARAMS: name = the url to the repository.
/// runs a 'git clone' command.
/// clones the repo into .gee/tmp/, and
/// it also logs the output to .gee/log.txt
pub fn clone_repo(name: &str) -> std::io::Result<()> {
	manage_repo_installations(name);
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
	if process.status.success() {
		write_data(name).expect("recording git clone failed.");
		println!("success.")
	} else {
		println!("error.");
		log_error(process).expect("logging error failed.");
	}
	Ok(())
}

/// PARAMS: name = the name of the repo you would like to delete.
/// runs a 'rm -rf' command
fn remove_repo(name: &str) -> std::io::Result<()> {
	let path = String::new() + ".gee/tmp/" + &name.to_string();
	let process = match Command::new("rm")
		.args(&["-rf", &path])
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.output()
	{
		Err(why) => panic!("error executing process: {}", why),
		Ok(process) => process,
	};
	if !process.status.success() {
		log_error(process).expect("logging error failed.");
	}
	Ok(())
}

/// PARAMS: none.
/// runs a 'cat' command, and
/// prints the output of .gee/log.txt
fn show_logs() {
	println!("\nshowing logs below: ");
	let process = match Command::new("cat")
		.arg(".gee/logs.txt")
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.output()
	{
		Err(why) => panic!("error executing process: {}", why),
		Ok(process) => process,
	};
	if process.status.success() {
		let output = String::from_utf8_lossy(&process.stdout);
		println!("{}", output)
	} else {
		println!("error.");
		log_error(process).expect("logging error failed.")
	}
}

/// PARAMS: process = a pointer to the process
/// that had a non-zero exit status code.
/// it writes the stderr to .gee/logs.txt, and then prints the log.
fn log_error(process: std::process::Output) -> std::io::Result<()> {
	let output = String::from_utf8_lossy(&process.stderr);
	let mut file = File::create(".gee/logs.txt")?;
	file.write_all(output.as_bytes())?;
	show_logs();
	Ok(())
}

/// PARAMS: path = a path to the file.
/// it will return true if the file exists,
/// and false if the file does not exists.
fn file_exists(path: &str) -> bool {
	let file = Path::new(path);
	if file.is_file() {
		return true;
	} else {
		return false;
	}
}

/// PARAMS: none.
/// tries to read in configurations from the .geerc file if it exists.
/// if not it will load in default configurations,
/// and return a struct of the current configurations.
fn parse_conf() -> Config {
	let mut config = Config { qsize: 5 };
	if file_exists(".geerc") {
		let file = File::open(".geerc").unwrap();
		let reader = BufReader::new(file);
		for line in reader.lines() {
			let line = line.unwrap();
			let mut split_line = line.split_whitespace();
			let key = split_line.next().expect("failed to parse the .geerc file");
			let value = split_line.next().expect("failed to parse the .geerc file");
			if key == "qsize" {
				let value: usize = value
					.trim()
					.parse()
					.expect("failed to convert &str value to i32");
				config.qsize = value;
			} else {
				println!(
					"\nunknown key: '{}' found in .geerc, now using default configurations.",
					key
				);
			}
		}
	}
	return config;
}
