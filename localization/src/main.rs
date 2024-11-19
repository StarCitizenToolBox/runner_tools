use std::process::exit;
use glob::glob;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "basic")]
struct CliOpt {
    #[structopt(short, long)]
    mode: String,
    #[structopt(long)]
    gh_repo: Option<String>,
    #[structopt(long)]
    gh_pr_number: Option<String>,
    #[structopt(long)]
    gh_pr_title: Option<String>,
}

fn main() {
    let opt = CliOpt::from_args();
    let mode = opt.mode.clone();
    self_check(opt.clone());
    // switch mode
    match mode.as_str() {
        "pr_check" => pr_check(),
        "auto_release" => auto_release(),
        "self_check" => exit(0),
        _ => {
            panic!("unknown mode: {}", mode);
        }
    }
}

fn self_check(opt: CliOpt) {
    // mode
    println!("mode: {}", opt.mode);
    // working dir
    let working_dir = std::env::current_dir().unwrap();
    println!("working dir: {:?}", working_dir);
    // gh_repo
    println!("gh_repo: {:?}", opt.gh_repo.unwrap_or("".to_string()));
    // gh_pr_number
    println!("gh_pr_number: {:?}", opt.gh_pr_number.unwrap_or("".to_string()));
    // gh_pr_title
    println!("gh_pr_title: {:?}", opt.gh_pr_title.unwrap_or("".to_string()));
}

fn pr_check() {
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

fn auto_release() {

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
