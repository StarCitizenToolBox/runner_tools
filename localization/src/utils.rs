use octocrab::Octocrab;

pub fn get_github_client(token_str: Option<&str>) -> Octocrab {
    let gh_token = if let Some(token_str) = token_str {
        token_str.to_string()
    } else {
        std::env::var("GH_TOKEN").unwrap_or("".to_string())
    };
    Octocrab::builder()
        .personal_token(gh_token)
        .build()
        .unwrap()
}

pub fn get_github_repo_name(repo_str: Option<&str>) -> (String, String) {
    // repo_str or env

    let gh_repo = if let Some(repo_str) = repo_str {
        repo_str.to_string()
    } else {
        std::env::var("GH_REPO").unwrap_or("".to_string())
    };

    let repo_split: Vec<&str> = gh_repo.split("/").collect();
    (
        repo_split[0].parse().unwrap(),
        repo_split[1].parse().unwrap(),
    )
}
