use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]

pub struct Allie {}

impl Bot for Allie {
    fn new() -> Self {
        Allie {}
    }

    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        let mut betray_count = 0;
        let mut silence_count = 0;
        match history
            .iter()
            .any(|turn| matches!(turn.other_bot, Dilemma::Betray))
        {
            true => betray_count += 1,
            false => silence_count += 1,
        }
        match betray_count > silence_count {
            true => Dilemma::Betray,
            false => Dilemma::Silence,
        }
    }
}
