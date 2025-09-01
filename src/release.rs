use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub release_name: String,
    pub namespace: String,
    pub values_file: String,
    pub chart_name: String,
    pub version: String,
    pub repo: String,
    pub repo_url: String,
}
