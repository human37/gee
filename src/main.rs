use spinners::{Spinner, Spinners};
use std::{env::set_current_dir, path::Path};
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
    commands::clone_repo("git@github.com:human37/gee.git").expect("error");
    commands::clone_repo("git@github.com:human37/stockbot.git").expect("error");
    commands::clone_repo("git@github.com:human37/ppm_editor.git").expect("error");
    commands::clone_repo("git@github.com:human37/lyrics_microservice.git").expect("error");
    sp.stop();
}

/// entrypoint to gee
fn main() {
    set_home_dir();
    run_command("loading...  ");
}
