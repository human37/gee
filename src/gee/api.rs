use std::vec::Vec;
use github_rs::client::{Executor, Github};
use serde_json::Value;

pub fn get_repos(org: &str, token: &str) -> Vec<String> {
    let mut repositories = vec![];
    let client = Github::new(token).unwrap();
    let me = client.get()
                   .orgs()
                   .org(org)
                   .repos()
                   .execute::<Value>();
    match me {
        Ok((_headers, _status, json)) => {
            if let Some(json) = json {
                let mut index: usize = 0;
                while json.get(index) != None {
                    let metadata = json.get(index).unwrap();
                    let mut ssh_url = metadata["ssh_url"].to_string();
                    ssh_url.pop();
                    ssh_url.remove(0);
                    repositories.push(ssh_url);
                    index += 1;
                }
            }
        },
        Err(e) => println!("{}", e)
    }
    return repositories;
}
