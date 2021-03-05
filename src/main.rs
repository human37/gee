mod gee;
use clap::{load_yaml, App};


fn main() {

    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();

    if let Some(clone) = m.value_of("clone") {
        match clone {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _ => unreachable!(),
        }
    } else {
        println!("--clone <CLONE> wasn't used...")
    }

    gee::utils::set_home_dir();
    gee::utils::test_run_command("loading...  ");
}
