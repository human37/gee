use std::{fs::File, io::prelude::*, path::Path, process::Command, process::Stdio};

/// PARAMS: path = a path to the file.
/// it will return true if the file exists,
/// and false if the file does not exists.
pub fn file_exists(path: &str) -> bool {
    let file = Path::new(path);
    file.is_file()
}

/// PARAMS: process = a pointer to the process
/// that had a non-zero exit status code.
/// it writes the stderr to .gee/logs.txt, and then prints the log.
pub fn log_error(process: std::process::Output) -> std::io::Result<()> {
    let output = String::from_utf8_lossy(&process.stderr);
    let mut file = File::create(".gee/logs.txt")?;
    file.write_all(output.as_bytes())?;
    show_logs();
    Ok(())
}

/// PARAMS: none.
/// runs a 'cat' command, and
/// prints the output of .gee/log.txt
pub fn show_logs() {
    println!("\nshowing logs below: ");
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
        println!("{}", output)
    } else {
        println!("error.");
        log_error(process).expect("logging error failed.")
    }
}

/// PARAMS: name = the name of the repo you would like to delete.
/// runs a 'rm -rf' command, and deletes the repository
/// with the parameters's name.
pub fn remove_repo(name: &str) -> std::io::Result<()> {
    let path = String::new() + ".gee/tmp/" + &name.to_string();
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
        log_error(process).expect("logging error failed.");
    }
    Ok(())
}
