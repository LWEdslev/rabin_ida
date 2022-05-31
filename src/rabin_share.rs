/// A share part
#[derive(Debug, Clone)]
pub struct RabinShare {
    pub id: u8,
    pub length: usize,
    pub body: Vec<u8>,
}
