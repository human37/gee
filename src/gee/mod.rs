use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
pub mod gee;
pub mod utils;

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Config {
    queue_size: usize,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Repo {
    url: String,
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        return &self.url == &other.url;
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Gee {
    config: Config,
    repositories: VecDeque<Repo>,
    current_dir: String,
    open_link: String,
}
