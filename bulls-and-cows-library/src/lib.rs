#[cfg(test)]
mod count_cattle_should {
    use crate::{count_cattle, CattleCount};

    #[test]
    fn return_4_bulls_and_0_cows_when_exactly_correct() {
        let secret_code = "1234";
        let guess = "1234";

        let _actual = count_cattle(secret_code, guess);
        let _expected = CattleCount::new(4, 0);
    }
}

pub struct CattleCount {}

impl CattleCount {
    pub fn new(_bulls: u8, _cows: u8) -> CattleCount {
        CattleCount {}
    }
}

pub fn count_cattle(_secret_code: &str, _guess: &str) {
}
