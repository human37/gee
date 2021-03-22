use super::{utils, Config, Gee, Repo};
use std::{
    collections::VecDeque, convert::TryInto, fs::File, io::prelude::*, io::BufRead, io::BufReader,
    io::Result, os::unix, process::Command, process::Stdio,
};

impl Gee {
    /// PARAMS: none.
    /// initializes the Gee struct by attempting to
    /// read in data from metadata.json into the
    /// repositories struct, and creating if it
    /// does not exist.
    pub fn new() -> Self {
        if utils::file_exists(&utils::prefix_home(".gee/metadata.json")) {
            let file = File::open(&utils::prefix_home(".gee/metadata.json")).unwrap();
            let mut gee: Gee =
                serde_json::from_reader(file).expect("could not deserialize metadata.json");
            gee.current_dir = utils::return_curr_dir();
            return gee;
        } else {
            Gee {
                repositories: VecDeque::new(),
                config: Config { queue_size: 5 },
                current_dir: "".to_string(),
                open_link: "".to_string(),
            }
        }
    }

    /// PARAMS: none.
    /// tries to read in configurations from the
    /// .geerc file if it exists. if not it will
    /// load in default configurations, and will
    /// assign the configurations to the config datamember.  
    pub fn parse_conf(&mut self) {
        if utils::file_exists(&utils::prefix_home(".geerc")) {
            let file = File::open(utils::prefix_home(".geerc")).unwrap();
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
                    let mut output = "unknown key: ".to_owned();
                    output.push_str(key);
                    output.push_str(" found in file .geerc, now using default configurations.");
                    println!("{}", output);
                }
            }
        }
    }

    /// PARAMS: url = the url of the repository being recorded.
    /// records meta data about the repository, and
    /// writes it to .gee/metadata.json
    fn write_data(&mut self) -> Result<()> {
        let metadata = serde_json::to_string(&self)?;
        let mut file = File::create(utils::prefix_home(".gee/metadata.json"))?;
        file.write_all(metadata.as_bytes())?;
        Ok(())
    }

    /// PARAMS: name = the url of the repository you would like cloned.
    /// parses the user's configurations, and checks to see if this
    /// repository has already been cloned, and also if the total number
    /// of cloned repositories is greater then the user's configured number.
    fn manage_repo_installations(&mut self) -> bool {
        if utils::file_exists(&utils::prefix_home(".gee/metadata.json")) {
            while self.repositories.len() > self.config.queue_size.try_into().unwrap() {
                let repo = self
                    .repositories
                    .pop_back()
                    .expect("could not pop last repo off queue");
                utils::remove_repo(utils::prettify_url(&repo.url)).expect("failed to remove repository");
                let mut output: String = "just popped ".to_owned();
                output.push_str(&utils::prettify_url(&repo.url));
                output.push_str(" off the queue.");
                utils::log_info(&output).expect("failed to log info");
            }
        }
        return true;
    }

    fn repo_on_queue(&mut self, name: &str) -> bool {
        if utils::file_exists(&utils::prefix_home(".gee/metadata.json")) {
            let repo = Repo {
                url: name.to_string(),
            };
            if self.repositories.contains(&repo) {
                utils::log_info("this repository is already on the queue.")
                    .expect("failed to log info");
                return true;
            }
        }
        return false;
    }

    /// PARAMS: name = the url to the repository.
    /// runs a 'git clone' command.
    /// clones the repo into .gee/tmp/, and
    /// it also logs the output to .gee/log.txt
    pub fn clone_repo(&mut self, name: &str) -> Result<()> {
        if !self.repo_on_queue(name) {
            let mut dir = utils::prefix_home(".gee/tmp/");
            dir.push_str(utils::prettify_url(name));
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
                if self.manage_repo_installations() {
                    self.repositories.push_front(Repo {
                        url: name.to_string(),
                    });
                    self.write_data().expect("recording git clone failed.");
                    utils::log_info("cloning repository was successful.")
                        .expect("logging info failed.");
                }
            } else {
                utils::log_process_error(process).expect("logging process error failed");
            }
        }
        Ok(())
    }

    /// PARAMS: index = the index of the repository.
    /// creates a symlink of the desired repository in
    /// the user's current directory. Also records the
    /// path of the link in the self.open_link datamember.
    pub fn open_repo(&mut self, index: usize) -> Result<()> {
        if self.open_link == "" {
            let path = utils::prefix_home(".gee/tmp/").to_owned()
                + utils::prettify_url(&self.repositories[index - 1].url);
            let link = String::from("./") + utils::prettify_url(&self.repositories[index - 1].url);
            self.open_link = String::from(utils::return_curr_dir())
                + "/"
                + utils::prettify_url(&self.repositories[index - 1].url);
            if !utils::dir_exists(&link) {
                self.write_data()?;
                unix::fs::symlink(path, link)?;
                println!(
                    "successfully opened {} in your current directory.",
                    utils::prettify_url(&self.repositories[index - 1].url)
                );
            } else {
                println!("this repository is already open, type 'gee done' to close.");
            }
        } else {
            println!("you already have a repository open, type 'gee done' to close.");
        }
        Ok(())
    }

    /// PARAMS: none. closes the currently opened
    /// repository, by running an 'rm' command on the
    /// symbolic link. If no repository is opened,
    /// it prints a message and returns.
    pub fn close_repo(&mut self) -> Result<()> {
        if utils::dir_exists(&self.open_link) {
            utils::remove_file(&self.open_link)?;
            self.open_link = "".to_string();
            self.write_data()?;
            println!("successfully closed the opened repository.")
        } else {
            println!("there is no repository currently open.");
        }
        Ok(())
    }

    /// PARAMS: none.
    /// it will print every repository that is
    /// currently installed, along with it's current
    /// index value.
    pub fn print_status(self) -> Result<()> {
        let mut index = 1;
        println!("==================");
        println!("index   repository");
        println!("==================");
        for repo in self.repositories {
            let url = utils::prettify_url(&repo.url);
            println!("[ {} ]   {} ", index, url);
            index += 1;
        }
        Ok(())
    }
}
