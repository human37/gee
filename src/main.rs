use spinners::{Spinner, Spinners};
use std::{env::set_current_dir, fs::read_to_string, path::Path, thread::sleep, time::Duration};
mod commands;

fn set_home_dir() {
    let root = Path::new("/Users/ammont");
    assert!(set_current_dir(&root).is_ok());
}

fn run_command(loading_msg: &str) {
    let sp = Spinner::new(Spinners::Dots, loading_msg.into());
    sleep(Duration::from_secs(1));
    commands::clone_repo("git@github.com:human37/stockbot.git");
    sp.stop();
}

fn read_configurations() {
    println!("in file {}", ".geerc");
    let contents = read_to_string(".geerc").expect("something went wrong reading the file");
    println!("with text:\n{}", contents);
}

fn main() {
    set_home_dir();
    read_configurations();
    run_command("cloning repostiory...");
}
