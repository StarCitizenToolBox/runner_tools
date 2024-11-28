use octocrab::models::repos::Release;
use octocrab::Page;

use crate::auto_api::AutoApi;
use crate::utils::{get_github_client, get_github_repo_name};
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
    pub enabled: bool,
    pub branch: String,
    pub version: String,
    pub info: String,
    pub note: String,
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
    let mut auto_api = AutoApi::new();
    auto_api.update_time = manifest.update_time.clone();
    auto_api.repo = manifest.target_api_repo.clone();
    auto_api.repo_branch = manifest.target_api_branch.clone();
    let releases = _get_github_repo_release(&gh_repo, &gh_token).await.unwrap();
    // check and create release
    for lang in &manifest.languages {
        auto_api.updated_releases.insert(lang.name.clone(), vec![]);
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
                let ok =
                    _create_github_repo_release(&gh_repo, &gh_token, &branch, &release_name).await;
                if ok {
                    println!("release created: {:?}", release_name);
                }
            } else {
                println!("SKIP: release already exist: {:?}", release_name);
            }
            auto_api
                .updated_releases
                .entry(lang.name.clone())
                .or_insert(vec![])
                .push(loc.clone());
        }
    }
    auto_api.push_change().await;
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
) -> bool {
    println!("Auto creating release: {:?}", version_name);
    let c = get_github_client(Some(gh_token));
    let (o, r) = get_github_repo_name(Some(gh_repo));
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
        return false;
    }
    let gen_notes = repo
        .releases()
        .generate_release_notes(version_name)
        .send()
        .await;

    if gen_notes.is_err() {
        println!("Failed to generate release notes: {:?}", gen_notes.err());
        return true;
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
        return true;
    }

    true
}

async fn _get_github_repo_release(
    gh_repo: &str,
    gh_token: &str,
) -> octocrab::Result<Page<Release>> {
    let c = get_github_client(Some(gh_token));
    let (o, r) = get_github_repo_name(Some(gh_repo));
    let repo = c.repos(o, r);
    repo.releases().list().per_page(255).send().await
}
