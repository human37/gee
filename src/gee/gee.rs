use super::{utils, Config, Gee, Repo};
use std::{
    collections::VecDeque, convert::TryInto, fs::File, io::prelude::*, io::BufRead, io::BufReader,
    io::Result, process::Command, process::Stdio,
};

impl Gee {
    pub fn new() -> Self {
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
    pub fn init(&mut self) {
        self.parse_conf();
        if utils::file_exists(".gee/metadata.json") {
            let file =
                File::open(".gee/metadata.json").expect("could not open the file metadata.json");
            self.repositories =
                serde_json::from_reader(file).expect("could not deserialize metadata.json");
        }
    }

    /// PARAMS: none.
    /// tries to read in configurations from the
    /// .geerc file if it exists. if not it will
    /// load in default configurations, and will
    /// assign the configurations to the config datamember.  
    fn parse_conf(&mut self) {
        if utils::file_exists(".geerc") {
            let file = File::open(".geerc").unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();
                let mut split_line = line.split_whitespace();
                let key = split_line.next().expect("failed to parse the .geerc file");
                let value = split_line.next().expect("failed to parse the .geerc file");
                if key == "queue_size" {
                    let value: usize = value
                        .trim()
                        .parse()
                        .expect("failed to convert &str value to i32");
                    self.config.queue_size = value;
                } else {
                    let mut output = "[ err ] unknown key: ".to_owned();
                    output.push_str(key);
                    output.push_str(" found in file .geerc, now using default configurations");
                }
            }
        }
    }

    /// PARAMS: url = the url of the repository being recorded.
    /// records meta data about the repository, and
    /// writes it to .gee/metadata.json
    fn write_data(&mut self) -> Result<()> {
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
        if utils::file_exists(".gee/metadata.json") {
            let repo = Repo {
                url: name.to_string(),
            };
            if self.repositories.contains(&repo) {
                utils::log_info("[ info ] this repository is already on the queue").expect("failed to log info");
                return false;
            }
            while self.repositories.len() >= self.config.queue_size.try_into().unwrap() {
                let repo = self
                    .repositories
                    .pop_back()
                    .expect("could not pop last repo off queue");
                utils::remove_repo(&repo.url).expect("failed to remove repository");
                let mut output: String = "[ info ] just popped ".to_owned();
                output.push_str(&repo.url);
                output.push_str(" off the queue");
                utils::log_info(&output).expect("failed to log info");
            }
        }
        return true;
    }

    /// PARAMS: name = the url to the repository.
    /// runs a 'git clone' command.
    /// clones the repo into .gee/tmp/, and
    /// it also logs the output to .gee/log.txt
    pub fn clone_repo(&mut self, name: &str) -> Result<()> {
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
                utils::log_info("[ info ] cloning repository was successful")
                    .expect("logging info failed.");
            } else {
                utils::log_process_error(process).expect("logging process error failed");
            }
        }
        Ok(())
    }
}