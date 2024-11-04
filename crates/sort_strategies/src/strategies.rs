use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Deserialize, Clone, Copy, PartialEq, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SortStrategy {
    Alphabetical = 0,
    AlphabeticalReversed,
    Lexicographical,
    LexicographicalReversed,
    // Add more strategies here as needed
    // Natural = 2,
    // CaseInsensitive = 3,
    // etc.
}