mod gee;
use clap::{load_yaml, App};
use spinners::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};

fn main() {
    gee::utils::set_home_dir();
    gee::utils::init_file_system().expect("failed to initialize filesystem.");
    let mut g = gee::Gee::new();
    g.init();
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    if let Some(url) = matches.subcommand_matches("clone") {
        let sp = Spinner::new(Spinners::Dots9, "cloning...".into());
        g.clone_repo(url.value_of("url").unwrap())
            .expect("could not clone repository");
        sp.message("done. ".into());
        sleep(Duration::from_secs(1));
        sp.stop();
    }
    gee::utils::show_logs();
}
