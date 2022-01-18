use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Titles {
    #[serde(rename = "titles")]
    pub titles: Vec<TitleElement>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TitleElement {
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "possibleTitles")]
    pub possible_titles: Option<Vec<String>>,

    #[serde(rename = "properName")]
    pub proper_name: String,

    #[serde(rename = "value")]
    pub value: String,
}

impl Display for TitleElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
