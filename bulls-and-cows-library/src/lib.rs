#[cfg(test)]
mod count_cattle_should {
    use crate::{count_cattle, Cattle, Guess, SecretCode};

    #[test]
    fn return_4_bulls_and_0_cows_when_exactly_correct() {
        let secret_code = SecretCode::new("1234");
        let guess = Guess::new("1234");

        let actual = count_cattle(guess, secret_code);
        let expected = Cattle::new(4, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bulls_and_0_cows_when_completely_wrong() {
        let secret_code = SecretCode::new("1234");
        let guess = Guess::new("5678");

        let actual = count_cattle(guess, secret_code);
        let expected = Cattle::new(0, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bulls_and_1_cow_when_one_digit_correct_and_wrong_position() {
        let secret_code = SecretCode::new("1234");
        let guess = Guess::new("5671");

        let actual = count_cattle(guess, secret_code);
        let expected = Cattle::new(0, 1);

        assert_eq!(actual, expected);
    }
}

#[derive(Debug, PartialEq)]
pub struct Bulls {
    count: u8,
}

impl Bulls {
    pub fn new(count: u8) -> Bulls {
        Bulls { count }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Cows {
    count: u8,
}

impl Cows {
    pub fn new(count: u8) -> Cows {
        Cows { count }
    }
}

pub struct Guess {
    code: String,
}

impl Guess {
    pub fn new(code: &str) -> Guess {
        Guess {
            code: code.to_string(),
        }
    }
}

pub struct SecretCode {
    code: String,
}

impl SecretCode {
    pub fn new(code: &str) -> SecretCode {
        SecretCode {
            code: code.to_string(),
        }
    }
}

pub fn count_cattle(guess: Guess, secret_code: SecretCode) -> Cattle {
    if guess.code == secret_code.code {
        return Cattle::new(4, 0);
    }

    Cattle::new(0, 0)
}
