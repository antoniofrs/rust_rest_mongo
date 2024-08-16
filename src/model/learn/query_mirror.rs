use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QueryParams {
    pub(crate) id: String,
    pub(crate) message: String
}