mod auto_release;
mod pr_check;

use std::process::exit;
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
    // always do self check
    self_check(opt.clone());
    // switch mode
    match mode.as_str() {
        "pr_check" => pr_check::do_check(),
        "auto_release" => {
            // wait pr check
            pr_check::do_check();
            // do release
            auto_release::do_release(opt);
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
    println!("gh_repo: {:?}", opt.gh_repo.unwrap_or("".to_string()));
    // gh_pr_number
    println!("gh_pr_number: {:?}", opt.gh_pr_number.unwrap_or("".to_string()));
    // gh_pr_title
    println!("gh_pr_title: {:?}", opt.gh_pr_title.unwrap_or("".to_string()));
    // get GH_TOKEN form env
    let gh_token = std::env::var("GH_TOKEN").unwrap_or("".to_string());
    println!("GH_TOKEN: {:?}", gh_token.len());
}