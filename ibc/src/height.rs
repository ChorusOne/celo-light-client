use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(
    Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
    )]
pub struct Height {
    #[serde(default)]
    pub revision_number: u64,
    #[serde(default)]
    pub revision_height: u64,
}
