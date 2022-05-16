// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "titles")]
    titles: Vec<TitleElement>,
}

#[derive(Serialize, Deserialize)]
pub struct TitleElement {
    /// The RGB HTML Color
    #[serde(rename = "color")]
    color: Option<String>,

    #[serde(rename = "description")]
    description: Option<String>,

    #[serde(rename = "possibleTitles")]
    possible_titles: Option<Vec<String>>,

    #[serde(rename = "properName")]
    proper_name: String,

    #[serde(rename = "value")]
    value: String,
}
