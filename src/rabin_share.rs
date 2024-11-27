use serde::{Deserialize, Serialize};

/// A share part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabinShare {
    pub id: u8,
    pub length: usize,
    pub body: Vec<u8>,
}
