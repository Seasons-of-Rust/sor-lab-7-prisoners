use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct Nguyen_HanhNong {
    pub name: String,
    pub age: u8,
    pub epicness: u8,
}

impl Bot for Nguyen_HanhNong {
    fn new() -> Self {
      Nguyen_HanhNong {
        name: "Nguyen-Hanh Nong".to_string(), 
        age: 18, 
        epicness: 100, }
    }

    fn turn(&mut self, history: &[Turn]) -> Dilemma {
      match history
            .iter()
            .count() > 100
        {
            true => Dilemma::Betray,
            false => Dilemma::Silence,
        }
    }
}