use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Declaration {
    pub release_name: String,
    pub namespace: String,
    pub values_file: String,
    pub chart_name: String,
    pub repo: String,
    pub repo_url: String,
}
