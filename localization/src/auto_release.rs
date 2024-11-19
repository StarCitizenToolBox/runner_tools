use octocrab::models::repos::Release;
use octocrab::{Octocrab, Page};
use regex::Regex;

pub async fn do_release() {
    let gh_repo = std::env::var("GH_REPO").unwrap_or("".to_string()); // xkeyC/StarCitizenToolBox_LocalizationData
    let gh_pr_title = std::env::var("GH_PR_TITLE").unwrap_or("".to_string()); // 3.24.3(PTU)_CN_V6
    let gh_token = std::env::var("GH_TOKEN").unwrap_or("".to_string()); // token
    if gh_repo.is_empty() || gh_token.is_empty() || gh_pr_title.is_empty() {
        println!("ENV is empty , skip auto Release ...");
        return;
    }

    let re = Regex::new(r"\d\.\d\.\d\(PTU|PU\)_.*_V.*").unwrap();
    if !re.is_match(&gh_pr_title) {
        println!("PR title not match auto release pattern, skip auto Release ...");
        return;
    }

    let mut has_release = false;
    // get release
    let releases = _get_github_repo_release(&gh_repo, &gh_token).await.unwrap();
    releases.items.iter().for_each(
        |release| {
            if release.tag_name == gh_pr_title {
                has_release = true;
            }
        }
    );

    if has_release {
        println!("Release already exist, skip auto Release ...");
        return;
    }
    
    // TODO 
}


async fn _get_github_repo_release(gh_repo: &str, gh_token: &str) -> octocrab::Result<Page<Release>> {
    let o = Octocrab::builder().personal_token(gh_token).build()?;
    let repo_split: Vec<&str> = gh_repo.split("/").collect();
    let repo = o.repos(repo_split[0], repo_split[1]);
    repo.releases().list().per_page(50).send().await
}