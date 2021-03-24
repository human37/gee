use github_rs::client::{Executor, Github};
use serde_json::Value;
use std::vec::Vec;

pub fn get_repos(org: &str, token: &str) -> Vec<String> {
    let mut repositories = vec![];
    let client = Github::new(token).unwrap();
    let issues_endpoint = format!("orgs/{}/repos?per_page=100", org);
    let me = client
        .get()
        .custom_endpoint(&issues_endpoint)
        .execute::<Value>();
    match me {
        Ok((_headers, _status, json)) => {
            if let Some(json) = json {
                if _status == 200 {
                    let mut index: usize = 0;
                    while json.get(index) != None {
                        let metadata = json.get(index).unwrap();
                        let mut ssh_url = metadata["ssh_url"].to_string();
                        ssh_url.pop();
                        ssh_url.remove(0);
                        repositories.push(ssh_url);
                        index += 1;
                    }
                } else {
                    println!(
                        "bad request, the github api responded with status code: {}",
                        _status
                    );
                }
            }
        }
        Err(e) => println!("{}", e),
    }
    return repositories;
}
