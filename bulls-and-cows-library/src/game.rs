#[cfg(test)]
mod game_test {
    mod get_game_over_should {
        use super::super::Game;
        use crate::guess::Guess;
        use crate::secret::Secret;
        use crate::shape::Shape;

        #[test]
        fn return_false_when_game_is_not_over() {
            let game = Game::default();

            let actual = game.get_game_over();
            let expected = false;

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_true_when_guess_matches() {
            let secret = Secret::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);
            let mut game = Game::new_with_secret(secret);
            let guess = Guess::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);
            game.add_guess(guess);

            let actual = game.get_game_over();
            let expected = true;

            assert_eq!(actual, expected);
        }

        #[test]
        fn return_false_when_guesses_do_not_match() {
            let secret = Secret::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);
            let mut game = Game::new_with_secret(secret);
            let guess = Guess::new(vec![
                Shape::Heart,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);
            game.add_guess(guess);

            let actual = game.get_game_over();
            let expected = false;

            assert_eq!(actual, expected);
        }
    }

    mod get_guess_history_should {
        use super::super::Game;
        use crate::counters::cattle::Cattle;
        use crate::guess::Guess;
        use crate::secret::Secret;
        use crate::shape::Shape;

        #[test]
        fn be_empty_when_creating_new() {
            let mut game = Game::default();
            for _ in 0..1000 {
                game = Game::default();
            }

            let actual = game.get_guess_history();
            let expected = vec![];

            assert_eq!(actual, expected);
        }

        #[test]
        fn have_one_move_in_history() {
            let secret = Secret::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);
            let mut game = Game::new_with_secret(secret);
            let guess = Guess::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);
            game.add_guess(guess);

            let actual = game.get_guess_history();
            let expected = vec![(
                Guess::new(vec![
                    Shape::Circle,
                    Shape::Triangle,
                    Shape::Square,
                    Shape::Star,
                ]),
                Cattle::new(4, 0),
            )];

            assert_eq!(actual, expected);
        }

        #[test]
        fn not_include_any_guesses_after_game_over() {
            let secret = Secret::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);
            let mut game = Game::new_with_secret(secret);
            let guess = Guess::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]);
            game.add_guess(guess);
            let guess = Guess::new(vec![
                Shape::Club,
                Shape::Diamond,
                Shape::Heart,
                Shape::Spade,
            ]);
            game.add_guess(guess);

            let actual = game.get_guess_history();
            let expected = vec![(
                Guess::new(vec![
                    Shape::Circle,
                    Shape::Triangle,
                    Shape::Square,
                    Shape::Star,
                ]),
                Cattle::new(4, 0),
            )];

            assert_eq!(actual, expected);                
        }
    }
}

use crate::counters::cattle::Cattle;
use crate::counters::count_cattle::count_cattle;
use crate::guess::Guess;
use crate::secret::Secret;
use rand;

pub struct Game {
    game_over: bool,
    guess_history: Vec<(Guess, Cattle)>,
    secret: Secret,
}

impl Game {
    pub fn add_guess(&mut self, guess: Guess) {
        if self.game_over {
            return;
        }

        let cattle = count_cattle(guess.clone(), &self.secret);
        self.game_over = guess.get_code() == self.secret.get_code();
        self.guess_history.push((guess, cattle));
    }

    pub fn get_game_over(&self) -> bool {
        self.game_over.clone()
    }

    pub fn get_guess_history(&self) -> Vec<(Guess, Cattle)> {
        self.guess_history.clone()
    }

    pub fn new() -> Game {
        let secret = Secret::new(vec![
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
        ]);
        Game::new_with_secret(secret)
    }

    pub fn new_with_secret(secret: Secret) -> Game {
        Game {
            game_over: false,
            guess_history: vec![],
            secret,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
