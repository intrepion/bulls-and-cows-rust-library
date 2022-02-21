#[cfg(test)]
mod count_cattle_should {
    use crate::{count_cattle, Cattle, Guess, SecretCode, Shape};

    #[test]
    fn return_4_bulls_and_0_cows_when_exactly_correct() {
        let secret_code = SecretCode::new(vec![Shape::Circle, Shape::Triangle, Shape::Square, Shape::Star]);
        let guess = Guess::new(vec![Shape::Circle, Shape::Triangle, Shape::Square, Shape::Star]);

        let actual = count_cattle(guess, secret_code);
        let expected = Cattle::new(4, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bulls_and_0_cows_when_completely_wrong() {
        let secret_code = SecretCode::new(vec![Shape::Circle, Shape::Triangle, Shape::Square, Shape::Star]);
        let guess = Guess::new(vec![Shape::Club, Shape::Diamond, Shape::Heart, Shape::Spade]);

        let actual = count_cattle(guess, secret_code);
        let expected = Cattle::new(0, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bulls_and_1_cow_when_one_digit_correct_and_wrong_position() {
        let secret_code = SecretCode::new(vec![Shape::Circle, Shape::Triangle, Shape::Square, Shape::Star]);
        let guess = Guess::new(vec![Shape::Club, Shape::Diamond, Shape::Heart, Shape::Circle]);

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
    code: Vec<Shape>,
}

impl Guess {
    pub fn new(code: Vec<Shape>) -> Guess {
        Guess {
            code,
        }
    }
}

pub struct SecretCode {
    code: Vec<Shape>,
}

impl SecretCode {
    pub fn new(code: Vec<Shape>) -> SecretCode {
        SecretCode {
            code,
        }
    }
}

#[derive(PartialEq)]
pub enum Shape {
    Circle,
    Club,
    Diamond,
    Heart,
    Star,
    Square,
    Spade,
    Triangle,
}

pub fn count_cattle(guess: Guess, secret_code: SecretCode) -> Cattle {
    let mut bulls: u8 = 0;
    let mut cows: u8 = 0;

    for (i, guess_shape) in guess.code.iter().enumerate() {
        for (j, secret_shape) in secret_code.code.iter().enumerate() {
            if i == j {
                if *guess_shape == *secret_shape {
                    bulls += 1;
                }
            } else if *guess_shape == *secret_shape {
                cows += 1;
            }
        }
    }

    Cattle::new(bulls, cows)
}
