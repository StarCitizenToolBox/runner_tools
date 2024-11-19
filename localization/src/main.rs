use std::process::exit;
use glob::glob;

fn main() {
    // cli arguments
    let mode = std::env::args().nth(1).unwrap_or("self_check".to_string());
    _self_check();
    // switch mode
    match mode.as_str() {
        "pr_check" => _pr_check(),
        "self_check" => exit(0),
        _ => {
            panic!("unknown mode: {}", mode);
        }
    }
}

fn _self_check() {
    // working dir
    let working_dir = std::env::current_dir().unwrap();
    println!("working dir: {:?}", working_dir);
}

fn _pr_check() {
    for entry in glob("./**/global.ini").unwrap() {
        match entry {
            Ok(path) => {
                println!("global.ini: {:?}", path.display());
                _ini_check(path)
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

fn _ini_check(path: std::path::PathBuf) {
    let content = std::fs::read_to_string(path.clone()).unwrap();
    let mut line_number = 0;
    let mut passed_lines = 0;
    for line in content.lines() {
        line_number +=1;
        if line.trim().is_empty() || line.trim().starts_with("#") {
            continue
        }
        // check has '='
        if !line.contains("=") {
            panic!("missing '=' on line: {}", line_number);
        }
        // check kv , split on first `=`
        let kv: Vec<&str> = line.splitn(2, "=").collect();
        if kv.len() != 2 {
            panic!("invalid kv format: {}", line_number);
        }

        passed_lines += 1;
    }
    println!("{}: passed lines: {}", path.display(), passed_lines);
}
