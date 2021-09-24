use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(
    Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
    )]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}
