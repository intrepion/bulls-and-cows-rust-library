#[cfg(test)]
mod game_test {
    mod get_guess_history_should {
        use crate::counters::cattle::Cattle;
        use crate::secret::Secret;
        use crate::shape::Shape;
        use crate::guess::Guess;

        use super::super::Game;

        #[test]
        fn be_empty_when_creating_new() {
            let game = Game::default();

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
            let expected = vec![(Guess::new(vec![
                Shape::Circle,
                Shape::Triangle,
                Shape::Square,
                Shape::Star,
            ]), Cattle::new(4, 0))];

            assert_eq!(actual, expected);
        }
    }
}

use crate::counters::cattle::Cattle;
use crate::counters::count_cattle::count_cattle;
use crate::guess::Guess;
use crate::secret::Secret;

pub struct Game {
    guess_history: Vec<(Guess, Cattle)>,
    secret: Secret,
}

impl Game {
    pub fn add_guess(&mut self, guess: Guess) {
        let cattle = count_cattle(guess.clone(), &self.secret);
        self.guess_history.push((guess, cattle));
    }

    pub fn get_guess_history(&self) -> Vec<(Guess, Cattle)> {
        self.guess_history.clone()
    }

    pub fn new() -> Game {
        Game::new_with_secret(Secret::new(vec![]))
    }

    pub fn new_with_secret(secret: Secret) -> Game {
        Game {
            guess_history: vec![],
            secret
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
