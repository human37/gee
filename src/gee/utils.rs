use std::{
    env::set_current_dir, fs::create_dir, fs::OpenOptions, io::Result, io::Write, path::Path,
    process::Command, process::Stdio, env::var, io::stdout,
};

const LOG_FILE: &'static str = ".gee/logs.txt";

/// PARAMS: path = a path to the file.
/// it will return true if the file exists,
/// and false if the file does not exist.
pub fn file_exists(path: &str) -> bool {
    let file = Path::new(path);
    file.is_file()
}

/// PARAMS: path = a path to the directory.
/// it will return true if the dir exists,
/// and false if the dir does not exist.
pub fn dir_exists(path: &str) -> bool {
    let dir = Path::new(path);
    dir.is_dir()
}

/// PARAMS: process = a pointer to the process
/// that had a non-zero exit status code.
/// it writes the stderr to .gee/logs.txt, and then prints the log.
pub fn log_process_error(process: std::process::Output) -> Result<()> {
    match OpenOptions::new().create(true).append(true).open(LOG_FILE) {
        Ok(ref mut file) => {
            let output = String::from_utf8_lossy(&process.stderr);
            writeln!(file, "{}", output).unwrap();
        }
        Err(err) => {
            panic!("failed to open log file: {}", err);
        }
    }
    Ok(())
}

/// PARAMS: info = a string of the message you would
/// liked logged. writes the output to the file
/// at the path .gee/logs.txt.
pub fn log_info(info: &str) -> Result<()> {
    match OpenOptions::new().create(true).append(true).open(LOG_FILE) {
        Ok(ref mut file) => {
            writeln!(file, "{}", info).unwrap();
        }
        Err(err) => {
            panic!("failed to open log file: {}", err);
        }
    }
    Ok(())
}

/// PARAMS: none.
/// runs a 'cat' command, and
/// prints the output of .gee/log.txt
pub fn show_logs() {
    let process = match Command::new("cat")
        .arg(".gee/logs.txt")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
    {
        Err(why) => panic!("error executing process: {}", why),
        Ok(process) => process,
    };
    if process.status.success() {
        let output = String::from_utf8_lossy(&process.stdout);
        stdout().flush().unwrap();
        print!("{}", output)
    } else {
        log_process_error(process).expect("logging error failed.")
    }
}

/// PARAMS: name = the name of the repo you would like to delete.
/// runs a 'rm -rf' command, and deletes the repository
/// with the parameters's name.
pub fn remove_repo(name: &str) -> Result<()> {
    let path = String::new() + ".gee/tmp/" + &name.to_string();
    remove_file(&path)?;
    Ok(())
}

/// PARAMS: path = the path to the file you would like deleted.
/// runs a 'rm -rf' command, and deletes the file with the
/// specified path.
pub fn remove_file(path: &str) -> Result<()> {
    let process = match Command::new("rm")
        .args(&["-rf", &path])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
    {
        Err(why) => panic!("error executing process: {}", why),
        Ok(process) => process,
    };
    if !process.status.success() {
        log_process_error(process).expect("logging error failed.");
    }
    Ok(())
}

/// PARAMS: none.
/// sets the working directory to the user's home directory.
pub fn set_home_dir() {
    let home_env = var("HOME").expect("could not fetch the user's HOME env variable.");
    let root = Path::new(&home_env);
    assert!(set_current_dir(&root).is_ok());
}

/// PARAMS: none.
/// if the necessary filesystem is not in place,
/// create the necessary files / directories. 
pub fn init_file_system() -> Result<()> {
    if !dir_exists(".gee") {
        create_dir(".gee")?;
    }
    if file_exists(".gee/logs.txt") {
        remove_file(".gee/logs.txt")?;
    }
    Ok(())
}

/// PARAMS: the url of the repository.
/// it will split the url based on the ':'
/// character, on order to tidy up the output
/// to the console.
pub fn prettify_url(url: &str) -> &str {
    let output: Vec<&str> = url.split(':').collect();
    return output[1];
}
