use crate::utils::{get_github_client, get_github_repo_name};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLocalizationData {
    pub enable: bool,
    pub version_name: String,
    pub update_at: String,
    pub info: String,
    #[serde(rename = "game_channel")]
    pub game_channel: String,
    pub note: String,
}

pub struct AutoApi {
    pub updated_releases: HashMap<String, Vec<crate::auto_release::_Localization>>,
    pub update_time: String,
    pub repo: String,
    pub repo_branch: String,
}

impl AutoApi {
    pub(crate) fn new() -> Self {
        Self {
            updated_releases: HashMap::new(),
            update_time: "".to_string(),
            repo: "".to_string(),
            repo_branch: "".to_string(),
        }
    }

    pub(crate) async fn push_change(&self) {
        println!("push change to repo: {:?}", &self.repo);
        let (owner, repo_s) = get_github_repo_name(Some(&self.repo));
        let o = get_github_client(None);
        let repo = o.repos(owner, repo_s);

        for (lang, releases) in &self.updated_releases {
            let file = format!("localizations/{}.json", lang);
            let _ref = format!("refs/heads/{}", self.repo_branch);
            println!(
                "reading files: {}/{}/{}",
                &self.repo, self.repo_branch, file
            );
            let contents = repo
                .get_content()
                .r#ref(_ref)
                .path(file.clone())
                .send()
                .await
                .unwrap();
            let content = contents.items.first().unwrap();
            let data = content.decoded_content().unwrap();
            let json_data: Vec<ApiLocalizationData> = serde_json::from_str(&data).unwrap();
            let mut new_data = json_data.clone();
            for release in releases {
                let found = new_data
                    .iter_mut()
                    .find(|d| d.version_name == release.version)
                    .is_some();

                if !found {
                    let game_channel = if release.version == "PU" { "PU" } else { "PTU" };
                    new_data.push(ApiLocalizationData {
                        enable: true,
                        version_name: release.version.clone(),
                        update_at: self.update_time.clone(),
                        info: release.info.clone(),
                        game_channel: game_channel.to_string(),
                        note: release.note.clone(),
                    });
                }
            }
            // remove outdated data (not in releases)
            new_data.retain(|d| {
                releases
                    .iter()
                    .find(|r| r.version == d.version_name)
                    .is_some()
            });
            let new_data = serde_json::to_string_pretty(&new_data).unwrap();
            
            // diff data
            if data == new_data {
                println!("Skip commit： no change in localization api data: {}", lang);
                continue;
            }
            
            println!("updating localization api data: {}", lang);
            repo.update_file(
                file.clone(),
                &format!("Auto update localization api data: {}", lang),
                &new_data,
                &content.sha,
            ).branch(&self.repo_branch)
            .send()
            .await
            .unwrap();
            println!("Updated localization api data: {}", lang);
        }
    }
}
