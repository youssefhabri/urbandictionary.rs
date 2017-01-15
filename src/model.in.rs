/// A definition and example for a word, including metadata such as the author's
/// name and the definition's rating.
#[derive(Clone, Debug, Deserialize)]
pub struct Definition {
    pub author: String,
    pub definition: String,
    pub example: String,
    #[serde(rename="defid")]
    pub id: u64,
    pub permalink: String,
    pub thumbs_down: u64,
    pub thumbs_up: u64,
    pub word: String,
}

/// A full response for a word, including the related tags and a list of
/// `Definition`s.
#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    #[serde(rename="list")]
    pub definitions: Vec<Definition>,
    pub tags: Vec<String>,
}
