use crate::counters::bull::Bulls;
use crate::counters::cow::Cows;

#[derive(Clone, Debug, PartialEq)]
pub struct Cattle {
    bulls: Bulls,
    cows: Cows,
}

impl Cattle {
    pub fn new(bulls: u8, cows: u8) -> Cattle {
        Cattle {
            bulls: Bulls::new(bulls),
            cows: Cows::new(cows),
        }
    }
}
