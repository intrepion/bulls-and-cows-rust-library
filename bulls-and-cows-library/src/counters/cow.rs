#[derive(Debug, PartialEq)]
pub struct Cows {
    count: u8,
}

impl Cows {
    pub fn new(count: u8) -> Cows {
        Cows { count }
    }
}
