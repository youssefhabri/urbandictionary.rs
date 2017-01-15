/// A definition and example for a word, including metadata such as the author's
/// name and the definition's rating.
#[derive(Clone, Debug, Deserialize)]
pub struct Definition {
    /// The name of the user who authored the definition.
    pub author: String,
    /// The text of the definition.
    pub definition: String,
    /// An example to provide context for a definition, such as a conversation
    /// that the word can be used in.
    pub example: String,
    /// The Id of the defintion.
    #[serde(rename="defid")]
    pub id: u64,
    /// A permanent link to the definition.
    pub permalink: String,
    /// The number of thumbs down that the definition has received.
    pub thumbs_down: u64,
    /// The number of thumbs up that the definition has received.
    pub thumbs_up: u64,
    /// The word that was defined.
    pub word: String,
}

/// A full response for a word, including the related tags and a list of
/// [`Definition`]s.
///
/// [`Definition`]: struct.Definition.html
#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    /// A list of definitions for a request, made via a call to [`definitions`].
    ///
    /// [`definitions`]: fn.definitions.html
    #[serde(rename="list")]
    pub definitions: Vec<Definition>,
    /// A list of tags that the word has been tagged with.
    pub tags: Vec<String>,
}
