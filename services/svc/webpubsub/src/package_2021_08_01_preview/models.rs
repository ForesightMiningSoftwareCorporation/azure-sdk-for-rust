#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientTokenResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}