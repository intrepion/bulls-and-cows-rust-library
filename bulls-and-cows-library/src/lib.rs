#[cfg(test)]
mod count_cattle_should {
    use crate::{count_cattle, Cattle};

    #[test]
    fn return_4_bulls_and_0_cows_when_exactly_correct() {
        let secret_code = "1234";
        let guess = "1234";

        let actual = count_cattle(secret_code, guess);
        let expected = Cattle::new(4, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bulls_and_0_cows_when_completely_wrong() {
        let secret_code = "1234";
        let guess = "5678";

        let actual = count_cattle(secret_code, guess);
        let expected = Cattle::new(0, 0);

        assert_eq!(actual, expected);
    }
}

#[derive(Debug, PartialEq)]
pub struct Cattle {
    bulls: u8,
    cows: u8,
}

impl Cattle {
    pub fn new(bulls: u8, cows: u8) -> Cattle {
        Cattle {
            bulls,
            cows,
        }
    }
}

pub fn count_cattle(_secret_code: &str, _guess: &str) -> Cattle {
    Cattle::new(4, 0)
}
