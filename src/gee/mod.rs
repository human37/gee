use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
pub mod gee;
pub mod utils;

#[derive(Clone, Deserialize, Serialize)]
struct Config {
    queue_size: usize,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Repo {
    pub url: String,
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        return &self.url == &other.url;
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Gee {
    config: Config,
    pub repositories: VecDeque<Repo>,
    current_dir: String,
    open_link: String,
}
