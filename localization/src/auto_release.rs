use octocrab::models::repos::Release;
use octocrab::{Octocrab, Page};

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalizationManifestData {
    #[serde(rename = "update_time")]
    pub update_time: String,
    pub languages: Vec<_Language>,
    #[serde(rename = "target_api_repo")]
    pub target_api_repo: String,
    #[serde(rename = "target_api_branch")]
    pub target_api_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct _Language {
    pub name: String,
    pub localizations: Vec<_Localization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct _Localization {
    pub branch: String,
    pub version: String,
}

pub async fn do_release() {
    let manifest = _read_repo_manifest().await;
    println!("repo manifest: {:?}", &manifest);
    let gh_repo = std::env::var("GH_REPO").unwrap_or("".to_string()); // xkeyC/StarCitizenToolBox_LocalizationData
    let gh_token = std::env::var("GH_TOKEN").unwrap_or("".to_string()); // token
    if gh_repo.is_empty() || gh_token.is_empty() {
        println!("ENV is empty , skip auto Release ...");
        return;
    }
    let releases = _get_github_repo_release(&gh_repo, &gh_token).await.unwrap();
    println!("releases: {:?}", releases);
}

async fn _read_repo_manifest() -> LocalizationManifestData {
    let file_path = "manifest.json";
    // check exist
    if !std::path::Path::new(file_path).exists() {
        panic!("manifest.json not found");
    }
    let file_content = tokio::fs::read_to_string(file_path).await.unwrap();
    let manifest: LocalizationManifestData = serde_json::from_str(&file_content).unwrap();
    manifest
}

async fn _get_github_repo_release(
    gh_repo: &str,
    gh_token: &str,
) -> octocrab::Result<Page<Release>> {
    let c = _get_github_client(Some(gh_token));
    let (o, r) = _get_github_repo(Some(gh_repo));
    let repo = c.repos(o, r);
    repo.releases().list().per_page(255).send().await
}

fn _get_github_client(token_str: Option<&str>) -> Octocrab {
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

fn _get_github_repo(repo_str: Option<&str>) -> (String, String) {
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
