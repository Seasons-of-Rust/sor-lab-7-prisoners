use super::{Bot, Dilemma, Turn};
use rand::Rng;

#[derive(Debug, Default)]
pub struct ZakuArbor {}

impl Bot for ZakuArbor {
    fn new() -> Self {
        ZakuArbor {}
    }

    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        const silence_baseline: u32 = history.len() as u32 / 2;
        let mut silence_acc: u32 = 0;
        let mut rng = rand::thread_rng();
        match history
            .iter()
            // If the other bot every betrayed, always betray
            .any(|turn| matches!(turn.other_bot, Dilemma::Betray))
        {
            true => silence_acc += 1,
            false => silence_acc *= rng.gen_range(1..9) / 10,
        }
        if silence_acc >= silence_baseline {
            return Dilemma::Silence;
        }
        Dilemma::Betray
    }
}
