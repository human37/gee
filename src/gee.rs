use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque, convert::TryInto, fs::File, io::prelude::*, io::BufRead, io::BufReader,
    path::Path, process::Command, process::Stdio,
};

struct Config {
    queue_size: usize,
}

#[derive(Deserialize, Serialize)]
struct Repo {
    url: String,
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        return &self.url == &other.url;
    }
}

pub struct Gee {
    repositories: VecDeque<Repo>,
    config: Config,
}

pub trait Commands {
    fn new() -> Self;
    fn init(&mut self);
    fn parse_conf(&mut self);
    fn file_exists(self, path: &str) -> &bool;
    fn write_data(self) -> std::io::Result<()>;
    fn manage_repo_installations(&mut self, name: &str) -> bool;
    fn clone_repo(&mut self, name: &str) -> std::io::Result<()>;
    fn remove_repo(self, name: &str) -> std::io::Result<()>;
    fn show_logs(self);
    fn log_error(self, process: std::process::Output) -> std::io::Result<()>;
}

impl Commands for Gee {
    fn new() -> Self {
        Gee {
            repositories: VecDeque::new(),
            config: Config { queue_size: 5 },
        }
    }
    /// PARAMS: none.
    /// initializes the Gee struct by attempting to
    /// read in data from metadata.json into the
    /// repositories struct, and creating if it
    /// does not exist. It will also attempt to
    /// read in configurations from a potential
    /// '.geerc' file in the user's home directory.
    fn init(&mut self) {
        self.parse_conf();
        if *self.file_exists(".gee/metadata.json") {
            let file =
                File::open(".gee/metadata.json").expect("could not open the file metadata.json");
            self.repositories =
                serde_json::from_reader(file).expect("could not deserialize metadata.json");
        } else {
            File::create(".gee/metadata.json").expect("could not open the file metadata.json");
        }
    }

    /// PARAMS: none.
    /// tries to read in configurations from the
    /// .geerc file if it exists. if not it will
    /// load in default configurations, and will
    /// assign the configurations to the config datamember.  
    fn parse_conf(&mut self) {
        if *self.file_exists(".geerc") {
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
                    self.config.queue_size = value;
                } else {
                    println!(
                        "\nunknown key: '{}' found in .geerc, now using default configurations.",
                        key
                    );
                }
            }
        }
    }

    /// PARAMS: path = a path to the file.
    /// it will return true if the file exists,
    /// and false if the file does not exists.
    fn file_exists(self, path: &str) -> &bool {
        let file = Path::new(path);
        &file.is_file()
    }

    /// PARAMS: url = the url of the repository being recorded.
    /// records meta data about the repository, and
    /// writes it to .gee/metadata.json
    fn write_data(self) -> std::io::Result<()> {
        let metadata = serde_json::to_string(&self.repositories)?;
        let mut file = File::create(".gee/metadata.json")?;
        file.write_all(metadata.as_bytes())?;
        Ok(())
    }

    /// PARAMS: name = the url of the repository you would like cloned.
    /// parses the user's configurations, and checks to see if this
    /// repository has already been cloned, and also if the total number
    /// of cloned repositories is greater then the user's configured number.
    fn manage_repo_installations(&mut self, name: &str) -> bool {
        if *self.file_exists(".gee/metadata.json") {
            let repo = Repo {
                url: name.to_string(),
            };
            if self.repositories.contains(&repo) {
                println!("already have this repository cloned!\n");
                return false;
            }
            while self.repositories.len() > self.config.queue_size.try_into().unwrap() {
                let repo = self
                    .repositories
                    .pop_back()
                    .expect("could not pop last repo off queue");
                self.remove_repo(&repo.url)
                    .expect("failed to remove repository");
                println!("just removed {}\n", repo.url);
            }
        }
        return true;
    }

    /// PARAMS: name = the url to the repository.
    /// runs a 'git clone' command.
    /// clones the repo into .gee/tmp/, and
    /// it also logs the output to .gee/log.txt
    fn clone_repo(&mut self, name: &str) -> std::io::Result<()> {
        if self.manage_repo_installations(name) {
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
                self.repositories.push_front(Repo {
                    url: name.to_string(),
                });
                self.write_data().expect("recording git clone failed.");
                println!("success.")
            } else {
                println!("error.");
                self.log_error(process).expect("logging error failed.");
            }
        }
        Ok(())
    }

    /// PARAMS: name = the name of the repo you would like to delete.
    /// runs a 'rm -rf' command, and deletes the repository
    /// with the parameters's name. 
    fn remove_repo(self, name: &str) -> std::io::Result<()> {
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
            self.log_error(process).expect("logging error failed.");
        }
        Ok(())
    }

    /// PARAMS: none.
    /// runs a 'cat' command, and
    /// prints the output of .gee/log.txt
    fn show_logs(self) {
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
            self.log_error(process).expect("logging error failed.")
        }
    }

    /// PARAMS: process = a pointer to the process
    /// that had a non-zero exit status code.
    /// it writes the stderr to .gee/logs.txt, and then prints the log.
    fn log_error(self, process: std::process::Output) -> std::io::Result<()> {
        let output = String::from_utf8_lossy(&process.stderr);
        let mut file = File::create(".gee/logs.txt")?;
        file.write_all(output.as_bytes())?;
        self.show_logs();
        Ok(())
    }
}
