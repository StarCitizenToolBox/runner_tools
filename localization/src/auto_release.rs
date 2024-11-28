use octocrab::models::repos::Release;
use octocrab::{Octocrab, Page};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::json;

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

    // check and create release
    for lang in &manifest.languages {
        for loc in &lang.localizations {
            let release_name = loc.version.clone();
            if releases
                .items
                .iter()
                .find(|r| r.name == Some(release_name.clone()))
                .is_none()
            {
                let gh_repo = gh_repo.clone();
                let gh_token = gh_token.clone();
                let branch = loc.branch.clone();
                let release_name = release_name.clone();
                tokio::spawn(async move {
                    _create_github_repo_release(&gh_repo, &gh_token, &branch, &release_name).await;
                })
                .await
                .unwrap();
            }else {
                println!("SKIP: release already exist: {:?}", release_name);
            }
        }
    }
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

async fn _create_github_repo_release(
    gh_repo: &str,
    gh_token: &str,
    branch: &str,
    version_name: &str,
) {
    println!("Auto creating release: {:?}", version_name);
    let c = _get_github_client(Some(gh_token));
    let (o, r) = _get_github_repo(Some(gh_repo));
    let repo = c.repos(o, r);

    let r = repo
        .releases()
        .create(version_name)
        .target_commitish(branch)
        .body("auto release")
        .name(version_name)
        .send()
        .await;

    if r.is_err() {
        println!("Failed to create release: {:?}", r.err());
        return;
    }
    let gen_notes = repo
        .releases()
        .generate_release_notes(version_name)
        .send()
        .await;

    if gen_notes.is_err() {
        println!("Failed to generate release notes: {:?}", gen_notes.err());
        return;
    }

    let r = repo
        .releases()
        .update(*r.unwrap().id)
        .body(&gen_notes.unwrap().body)
        .name(version_name)
        .send()
        .await;

    if r.is_err() {
        println!("Failed to update release: {:?}", r.err());
        return;
    }

    println!("release created: {:?}", r.unwrap().tag_name);
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
