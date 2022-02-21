#[cfg(test)]
mod count_cattle_should {
    use crate::count_cattle;

    #[test]
    fn return_4_bulls_and_0_cows_when_exactly_correct() {
        let secret_code = "1234";
        let guess = "1234";

        let _actual = count_cattle(secret_code, guess);
    }
}

pub fn count_cattle(_secret_code: &str, _guess: &str) {
}
