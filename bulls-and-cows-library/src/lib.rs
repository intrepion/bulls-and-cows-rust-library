#[cfg(test)]
mod count_cattle_should {
    use crate::{count_cattle, Cattle, Guess, Secret, Shape};

    #[test]
    fn return_4_bull_and_0_cow_when_4_correct() {
        let secret = Secret::new(vec![
            Shape::Circle,
            Shape::Triangle,
            Shape::Square,
            Shape::Star,
        ]);
        let guess = Guess::new(vec![
            Shape::Circle,
            Shape::Triangle,
            Shape::Square,
            Shape::Star,
        ]);

        let actual = count_cattle(guess, secret);
        let expected = Cattle::new(4, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bull_and_0_cow_when_completely_wrong() {
        let secret = Secret::new(vec![
            Shape::Circle,
            Shape::Triangle,
            Shape::Square,
            Shape::Star,
        ]);
        let guess = Guess::new(vec![
            Shape::Club,
            Shape::Diamond,
            Shape::Heart,
            Shape::Spade,
        ]);

        let actual = count_cattle(guess, secret);
        let expected = Cattle::new(0, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bull_and_1_cow_when_1_wrong_position() {
        let secret = Secret::new(vec![
            Shape::Circle,
            Shape::Triangle,
            Shape::Square,
            Shape::Star,
        ]);
        let guess = Guess::new(vec![
            Shape::Club,
            Shape::Diamond,
            Shape::Heart,
            Shape::Circle,
        ]);

        let actual = count_cattle(guess, secret);
        let expected = Cattle::new(0, 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_1_bull_and_0_cow_when_1_correct() {
        let secret = Secret::new(vec![
            Shape::Circle,
            Shape::Triangle,
            Shape::Square,
            Shape::Star,
        ]);
        let guess = Guess::new(vec![
            Shape::Circle,
            Shape::Diamond,
            Shape::Heart,
            Shape::Spade,
        ]);

        let actual = count_cattle(guess, secret);
        let expected = Cattle::new(1, 0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_1_bull_and_3_cow_when_4_correct_and_3_wrong_position() {
        let secret = Secret::new(vec![
            Shape::Circle,
            Shape::Triangle,
            Shape::Square,
            Shape::Star,
        ]);
        let guess = Guess::new(vec![
            Shape::Circle,
            Shape::Circle,
            Shape::Circle,
            Shape::Circle,
        ]);

        let actual = count_cattle(guess, secret);
        let expected = Cattle::new(1, 3);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bull_and_4_cow_when_4_correct_and_4_wrong_position() {
        let secret = Secret::new(vec![
            Shape::Circle,
            Shape::Triangle,
            Shape::Square,
            Shape::Star,
        ]);
        let guess = Guess::new(vec![
            Shape::Star,
            Shape::Square,
            Shape::Triangle,
            Shape::Circle,
        ]);

        let actual = count_cattle(guess, secret);
        let expected = Cattle::new(0, 4);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_0_bull_and_3_cow_when_3_correct_and_3_wrong_position() {
        let secret = Secret::new(vec![
            Shape::Circle,
            Shape::Circle,
            Shape::Square,
            Shape::Star,
        ]);
        let guess = Guess::new(vec![
            Shape::Star,
            Shape::Square,
            Shape::Triangle,
            Shape::Circle,
        ]);

        let actual = count_cattle(guess, secret);
        let expected = Cattle::new(0, 3);

        assert_eq!(actual, expected);
    }

    #[test]
    fn return_1_bull_and_2_cow_when_3_correct_and_2_wrong_position() {
        let secret = Secret::new(vec![
            Shape::Circle,
            Shape::Circle,
            Shape::Square,
            Shape::Star,
        ]);
        let guess = Guess::new(vec![
            Shape::Star,
            Shape::Circle,
            Shape::Triangle,
            Shape::Square,
        ]);

        let actual = count_cattle(guess, secret);
        let expected = Cattle::new(1, 2);

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
        Guess { code }
    }
}

pub struct Secret {
    code: Vec<Shape>,
}

impl Secret {
    pub fn new(code: Vec<Shape>) -> Secret {
        Secret { code }
    }
}

#[derive(Debug, PartialEq)]
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

pub fn count_cattle(guess: Guess, secret: Secret) -> Cattle {
    let mut bulls: u8 = 0;
    let mut cows: u8 = 0;

    for (i, guess_shape) in guess.code.iter().enumerate() {
        if *guess_shape == secret.code[i] {
            bulls += 1;
        } else {
            for (j, secret_shape) in secret.code.iter().enumerate() {
                if i != j && *secret_shape == *guess_shape {
                    cows += 1;
                }
            }
        }
    }

    Cattle::new(bulls, cows)
}
