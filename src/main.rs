use spinners::{Spinner, Spinners};
use std::{env::set_current_dir, fs::read_to_string, path::Path};
mod commands;

/// sets the working directory to the user's home directory
fn set_home_dir() {
    let root = Path::new("/Users/ammont");
    assert!(set_current_dir(&root).is_ok());
}

/// calls the 'clone_repo' function, and also
/// shows a loading spinner while the command is running
fn run_command(loading_msg: &str) {
    let sp = Spinner::new(Spinners::Dots, loading_msg.into());
    commands::clone_repo("git@github.com:human37/stockbot.git").expect("error");
    sp.stop();
}

/// reads the contents of .geerc (if it exists)
/// in order to load in user specific configurations
fn read_configurations() {
    println!("in file {}", ".geerc");
    let contents = read_to_string(".geerc").expect("something went wrong reading the file");
    println!("with text:\n{}", contents);
}

/// entrypoint to gee
fn main() {
    set_home_dir();
    read_configurations();
    run_command("loading...  ");
}