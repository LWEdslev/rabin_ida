
#[derive(Debug, Clone)]
pub struct RabinShare {
    pub id: u8,
    pub length: usize,
    pub body: Vec<u8>,
}

impl RabinShare {
    fn size(&self) -> usize {
        self.length
    }
    fn with_size(size: usize) -> Self {
        Self {
            id: 0,
            length: 0,
            body: vec![0u8; size],
        }
    }
}