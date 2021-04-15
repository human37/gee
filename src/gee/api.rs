use github_rs::client::{Executor, Github};
use serde_json::Value;
use std::convert::TryInto;
use std::vec::Vec;

pub fn get_repos(org: &str, token: &str, max: usize) -> Vec<String> {
    let mut total_repositories = vec![];
    let mut page_num = 1;
    let total_org_repos = get_num_repositories(org, token).try_into().unwrap();
    while total_repositories.len() < total_org_repos {
        if max == 0 || total_repositories.len() >= max {
            break;
        }
        total_repositories.append(&mut request(token, org, page_num));
        page_num += 1;
    }
    if total_repositories.len() > max {
        return total_repositories[..=max].to_vec();
    }
    return total_repositories;
}

fn get_num_repositories(org: &str, token: &str) -> i64 {
    let client = Github::new(token).unwrap();
    let endpoint = format!("orgs/{}", org);
    let me = client
        .get()
        .custom_endpoint(&endpoint)
        .execute::<Value>();
    match me {
        Ok((_headers, _status, json)) => {
            if let Some(json) = json {
                if _status == 200 {
                    let mut total_public = 0;
                    let mut total_private = 0;
                    if json.get("public_repos") != None {
                        total_public = json.get("public_repos").unwrap().as_i64().unwrap();
                    }
                    if json.get("private_repos") != None {
                        total_private = json.get("private_repos").unwrap().as_i64().unwrap();
                    }
                    return total_public + total_private;
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
    return 0;
}

fn request(token: &str, org: &str, page_num: usize) -> Vec<String> {
    let mut repositories = vec![];
    let client = Github::new(token).unwrap();
    let endpoint = format!("orgs/{}/repos?page={}", org, page_num);
    let me = client
        .get()
        .custom_endpoint(&endpoint)
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
