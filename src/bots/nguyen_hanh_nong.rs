use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct NguyenHanhNong {
    pub name: String,
    pub age: u8,
    pub epicness: u8,
}

impl Bot for NguyenHanhNong {
    fn new() -> Self {
        NguyenHanhNong {
            name: "Nguyen-Hanh Nong".to_string(),
            age: 18,
            epicness: 100,
        }
    }

    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        match history.len() > 100 {
            true => Dilemma::Betray,
            false => Dilemma::Silence,
        }
    }
}
