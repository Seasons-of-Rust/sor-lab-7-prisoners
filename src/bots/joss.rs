use rand::Rng;

use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct Joss {}

impl Bot for Joss {
    fn new() -> Self {
        Joss {}
    }

    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..10) {
            // 10% of the time, betray
            0i32 => Dilemma::Betray,
            // 90% of the time, use tft
            _ => {
                history
                    .last()
                    .unwrap_or(&Turn {
                        you: Dilemma::Silence,
                        other_bot: Dilemma::Silence,
                    })
                    .other_bot
            }
        }
    }
}
