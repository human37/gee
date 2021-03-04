use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
pub mod gee;
pub mod utils;

#[derive(Clone)]
struct Config {
    queue_size: usize,
}

#[derive(Clone, Deserialize, Serialize)]
struct Repo {
    url: String,
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        return &self.url == &other.url;
    }
}

#[derive(Clone)]
pub struct Gee {
    repositories: VecDeque<Repo>,
    config: Config,
}
