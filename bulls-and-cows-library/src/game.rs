#[cfg(test)]
mod game_should {
    use super::Game;

    #[test]
    fn have_initial_state_when_creating_new() {
        let game = Game::new();

        let actual = game.get_move_history();
        let expected = vec![];

        assert_eq!(actual, expected);
    }
}

use crate::counters::cattle::Cattle;
use crate::guess::Guess;
pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn get_move_history(&self) -> Vec<(Guess, Cattle)> {
        vec![]
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
