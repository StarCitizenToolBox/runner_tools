mod auto_release;
mod pr_check;
mod auto_api;
mod utils;

use std::process::exit;
use structopt::StructOpt;

// ANSI color codes
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "basic")]
struct CliOpt {
    #[structopt(short, long)]
    mode: String,
}

#[tokio::main]
async fn main() {
    let opt = CliOpt::from_args();
    let mode = opt.mode.clone();
    // always do self check
    self_check(opt.clone());
    // switch mode
    match mode.as_str() {
        "pr_check" => {
            let error_count = pr_check::do_check();
            if error_count > 0 {
                println!("{}Found {} total errors{}", RED, error_count, RESET);
                exit(1);
            }
        },
        "auto_release" => {
            // wait pr check
            let error_count = pr_check::do_check();
            if error_count > 0 {
                println!("{}Found {} total errors, aborting release{}", RED, error_count, RESET);
                exit(1);
            }
            // do release
            auto_release::do_release().await;
        },
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
    let gh_repo = std::env::var("GH_REPO").unwrap_or("".to_string());
    println!("gh_repo: {:?}", gh_repo);
    // gh_pr_number
    let gh_pr_number = std::env::var("GH_PR_NUMBER").unwrap_or("".to_string());
    println!("gh_pr_number: {:?}", gh_pr_number);
    // gh_pr_title
    let gh_pr_title = std::env::var("GH_PR_TITLE").unwrap_or("".to_string());
    println!("gh_pr_title: {:?}", gh_pr_title);
    // get GH_TOKEN form env
    let gh_token = std::env::var("GH_TOKEN").unwrap_or("".to_string());
    println!("GH_TOKEN: {:?}", gh_token.len());
}