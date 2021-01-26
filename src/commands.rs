use cmd_lib::run_cmd;

pub fn find_file(file: &str) {

    if run_cmd! {
        pwd
        cd ../
        pwd
    }.is_err() {
        println!("file: {} could not be found.", file);
    }
}

