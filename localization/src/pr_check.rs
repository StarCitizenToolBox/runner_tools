use glob::glob;

pub fn do_check() {
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
