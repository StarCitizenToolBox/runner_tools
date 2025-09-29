use glob::glob;

// ANSI color codes
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub fn do_check() -> i32 {
    let mut total_errors = 0;
    for entry in glob("./**/global.ini").unwrap() {
        match entry {
            Ok(path) => {
                println!("global.ini: {:?}", path.display());
                total_errors += _ini_check(path)
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    total_errors
}

fn _ini_check(path: std::path::PathBuf) -> i32 {
    let content = std::fs::read_to_string(path.clone()).unwrap();
    let mut line_number = 0;
    let mut passed_lines = 0;
    let mut errors = 0;
    
    for line in content.lines() {
        line_number +=1;
        if line.trim().is_empty() || line.trim().starts_with("#") {
            continue
        }
        // check has '='
        if !line.contains("=") {
            println!("{}ERROR{}: missing '=' on line {} in file {}: {}", RED, RESET, line_number, path.display(), line);
            errors += 1;
            continue;
        }
        // check kv , split on first `=`
        let kv: Vec<&str> = line.splitn(2, "=").collect();
        if kv.len() != 2 {
            println!("{}ERROR{}: invalid kv format on line {} in file {}: {}", RED, RESET, line_number, path.display(), line);
            errors += 1;
            continue;
        }

        passed_lines += 1;
    }
    println!("{}{}:{} passed lines: {}", GREEN, path.display(), RESET, passed_lines);
    if errors > 0 {
        println!("{}{}:{} found {} errors", RED, path.display(), RESET, errors);
    }
    errors
}
