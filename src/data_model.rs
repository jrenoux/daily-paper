use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub paper_folder: String,
    pub note_folder: String,
    #[serde(default = "default_regex")]
    pub regex: String
}

fn default_regex() -> String {
    ".".to_string()
}