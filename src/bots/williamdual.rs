use rand::Rng;

use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct Williamdual {
    pub who_screams_for_ice_cream: String,
}
impl Bot for Williamdual {
    fn new() -> Self {
        Williamdual {
            who_screams_for_ice_cream: "weAllScreamForIceCream".to_string(),
        }
    }
    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) {
            1 | 2 => match history.len() % 2 {
                0 => Dilemma::Silence,
                1 => Dilemma::Betray,
                _ => Dilemma::Silence,
            },
            _ => Dilemma::Betray,
        }
    }
}
