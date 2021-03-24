use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
pub mod gee;
pub mod utils;
pub mod api;

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    queue_size: usize,
    pub github_token: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Repo {
    pub url: String,
    pub is_mass: bool,
    pub org: Organization,
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        return &self.url == &other.url;
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Organization {
    pub name: String,
    pub repositories: Vec<Repo>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Gee {
    pub config: Config,
    pub repositories: VecDeque<Repo>,
    current_dir: String,
    open_link: String,
}
