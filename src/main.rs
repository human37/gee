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
        match index.value_of("index").unwrap().parse::<usize>() {
            Ok(value) => {
                g.open_repo(value).expect("could not open repository.");
            }
            Err(_) => {
                println!("could not parse the index, use 'gee list' to find the index of the repository you would like to open.")
            }
        };
    }
    // "done" subcommand
    if let Some(_) = matches.subcommand_matches("done") {
        let mut g = gee::Gee::new();
        g.parse_conf();
        g.close_repo().expect("could not close repository.");
    }
    // "keep" subcommand
    if let Some(args) = matches.subcommand_matches("keep") {
        let mut g = gee::Gee::new();
        g.parse_conf();
        match args.value_of("index").unwrap().parse::<usize>() {
            Ok(value) => {
                let repo = &g.repositories[value - 1].url;
                let path = args.value_of("path").unwrap_or(".");
                gee::utils::copy_repo(&repo, path).expect("could not copy the repository.");
            }
            Err(_) => {
                println!("could not parse the index, use 'gee list' to find the index of the repository you would like to open.")
            }
        };
    }
    // "mass" subcommand
    if let Some(args) = matches.subcommand_matches("mass") {
        match args.value_of("organization").unwrap().parse::<String>() {
            Ok(org) => {
                let mut g = gee::Gee::new();
                g.parse_conf();
                if g.config.github_token == "" {
                    println!("you do not have a github personal access token in your '.geerc' file.");
                    return;
                }
                let wildcard = args.value_of("wildcard").unwrap_or("");
                let repositories =
                    gee::api::get_repos(&org, &g.config.github_token);
                let repos_contain =
                    gee::utils::contains_substring(repositories, wildcard.to_string());
                let num_repos = repos_contain.len();
                let full_org_name = String::from(&org) + " [wildcard: '" + wildcard + "']";
                if !g.repo_on_queue(&full_org_name) {
                    if wildcard != "" {
                        println!(
                            "found {} repositories within {} that match the wildcard '{}'",
                            num_repos, org, wildcard
                        );
                    } else {
                        println!("found {} repositories within {}", num_repos, org);
                    }
                    if num_repos > 0 {
                        let mut num_cloned = 1;
                        let sp = Spinner::new(Spinners::Dots9, "cloning...".into());
                        for repo in repos_contain {
                            let status = String::new()
                                + "cloning... ("
                                + &num_cloned.to_string()
                                + "/"
                                + &num_repos.to_string()
                                + ")";
                            sp.message(status);
                            g.clone_repo_within_org(&repo, &full_org_name)
                                .expect("could not clone repository");
                            num_cloned += 1;
                        }
                        sp.stop();
                        println!("");
                    }
                }
            }
            Err(_) => {
                println!("could not parse the organization arguments correctly.");
            }
        };
    }
    gee::utils::show_logs();
}
