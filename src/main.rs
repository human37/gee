mod gee;
use clap::{load_yaml, App};
use spinners::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};

fn main() {
    gee::utils::init_file_system().expect("failed to initialize filesystem.");
    let cli_conf = load_yaml!("cli.yaml");
    let matches = App::from(cli_conf).get_matches();
    // "clone" subcommand
    if let Some(url) = matches.subcommand_matches("clone") {
        let mut g = gee::Gee::new();
        g.parse_conf();
        let sp = Spinner::new(Spinners::Dots9, "cloning...".into());
        g.clone_repo(url.value_of("url").unwrap())
            .expect("could not clone repository");
        sp.message("done. ".into());
        sleep(Duration::from_secs(1));
        sp.stop();
    }
    // "list" subcommand
    if let Some(_) = matches.subcommand_matches("list") {
        let mut g = gee::Gee::new();
        g.parse_conf();
        g.print_status().expect("could not print repository info.");
    }
    // "open" subcommand
    if let Some(index) = matches.subcommand_matches("open") {
        let mut g = gee::Gee::new();
        g.parse_conf();
        g.open_repo(index.value_of("index").unwrap().parse::<usize>().unwrap())
            .expect("could not open repository.");
    }
    // "done" subcommand
    if let Some(_) = matches.subcommand_matches("done") {
        let mut g = gee::Gee::new();
        g.parse_conf();
        g.close_repo().expect("could not close repository.");
    }
    gee::utils::show_logs();
}