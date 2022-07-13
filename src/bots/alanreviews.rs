use super::{Bot, Dilemma, Turn};
// use rand::Rng;

#[derive(Debug, Default)]
pub struct AlanReviews {}

impl Bot for AlanReviews {
    fn new() -> Self {
        AlanReviews {}
    }

    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        match history.iter().last() {
            Some(turn) => turn.other_bot,
            None => Dilemma::Silence,
        }
        // let iterator = history.iter().rev().take(2);

        // Randomly choose between silence and betrayal
        // let mut rng = rand::thread_rng();
        // let number: i32 = rng.gen_range(0..3);
        // if (0..4).contains(&number) {
        //     Dilemma::Silence
        // }
        // else {
        //     Dilemma::Betray
        // }
    }
}
