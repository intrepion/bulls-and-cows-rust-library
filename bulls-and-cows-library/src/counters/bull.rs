#[derive(Clone, Debug, PartialEq)]
pub struct Bulls {
    count: u8,
}

impl Bulls {
    pub fn new(count: u8) -> Bulls {
        Bulls { count }
    }
}
