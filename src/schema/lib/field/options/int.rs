#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Cardinality {
    /// The document must have exactly one value associated to the document.
    #[serde(rename = "single")]
    SingleValue,
    /// The document can have any number of values associated to the document.
    /// This is more memory and CPU expensive than the SingleValue solution.
    #[serde(rename = "multi")]
    MultiValues,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntOptions {
    indexed: bool,
    fast: Option<Cardinality>,
    stored: bool
}

impl IntOptions {
    pub fn is_indexed(&self) -> bool {
        self.indexed
    }
}

