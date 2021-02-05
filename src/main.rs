use spinners::{Spinner, Spinners};
use std::env::set_current_dir;
use std::path::Path;
mod commands;

fn test_stuff(all: bool) {
    if all {
        set_home_dir();
        commands::clone_repo("hello");
    }
}

fn set_home_dir() {
    let root = Path::new("/Users/ammont");
    assert!(set_current_dir(&root).is_ok());
}

fn run_command(loading_msg: &str) {
    let sp = Spinner::new(Spinners::Dots, loading_msg.into());
    loop{};
    commands::clone_repo("hello");
    sp.stop();
}

fn main() {
    test_stuff(false);
    set_home_dir();
    run_command("loading");
    print!(" ");
}
