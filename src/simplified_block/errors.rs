use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SolProbeError {
    UnsupportedEncoding,
    UnsupportedMessageType,
}
