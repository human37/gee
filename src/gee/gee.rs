use super::{utils, Config, Gee, Organization, Repo};
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
                config: Config {
                    queue_size: 5,
                    github_token: "".to_string(),
                },
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
            let mut token_found = false;
            let mut queue_size_found = false;
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
                    queue_size_found = true;
                } else if key == "github_token" {
                    let value: String = value
                        .trim()
                        .parse()
                        .expect("failed to convert &str to String");
                    self.config.github_token = value;
                    token_found = true;
                } else {
                    let mut output = "unknown key: ".to_owned();
                    output.push_str(key);
                    output.push_str(" found in file '.geerc', now using default configurations.");
                    println!("{}", output);
                }
            }
            if !token_found {
                self.config.github_token = "".to_string();
            }
            if !queue_size_found {
                self.config.queue_size = 5;
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
                utils::remove_repo(utils::prettify_url(&repo.url))
                    .expect("failed to remove repository");
                let mut output: String = "just popped ".to_owned();
                output.push_str(&utils::prettify_url(&repo.url));
                output.push_str(" off the queue.");
                utils::log_info(&output).expect("failed to log info");
            }
        }
        return true;
    }

    pub fn repo_on_queue(&mut self, name: &str) -> bool {
        if utils::file_exists(&utils::prefix_home(".gee/metadata.json")) {
            let repo = Repo {
                url: name.to_string(),
                is_mass: false,
                org: Organization {
                    name: "".to_string(),
                    repositories: vec![],
                },
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
                        is_mass: false,
                        org: Organization {
                            name: "".to_string(),
                            repositories: vec![],
                        },
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

    /// PARAMS: name = the url to the repository,
    /// org = the name of the organization.
    /// runs a 'git clone' command.
    /// clones the repo into .gee/tmp/org/, and
    /// it also logs the output to .gee/log.txt
    pub fn clone_repo_within_org(&mut self, name: &str, org: &str) -> Result<()> {
        let mut dir = utils::prefix_home(".gee/tmp/");
        dir.push_str(org);
        dir.push_str("/");
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
            let organization = Repo {
                url: org.to_string(),
                is_mass: true,
                org: Organization {
                    name: org.to_string(),
                    repositories: vec![],
                },
            };
            let index = self.repositories.iter().position(|x| x == &organization);
            if index == None {
                if self.manage_repo_installations() {
                    self.repositories.push_back(organization);
                }
            } else {
                self.repositories[index.unwrap()]
                    .org
                    .repositories
                    .push(organization);
            }
            self.write_data().expect("recording git clone failed.");
        } else {
            utils::log_process_error(process).expect("logging process error failed");
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
        if self.repositories.len() != 0 {
            println!("index   repository");
            for i in 0..self.repositories.len() {
                let url = utils::prettify_url(&self.repositories[i].url);
                println!("[ {} ] | {} ", index, url);
                index += 1;
            }
        } else {
            println!("you do not have any repositories currently installed with gee.")
        }
        Ok(())
    }
}
