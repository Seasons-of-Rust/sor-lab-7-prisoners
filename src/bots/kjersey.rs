use std::cmp::Ordering;

use super::{Bot, Dilemma, Turn};

#[derive(Debug, Default)]
pub struct KJersey {
    helped: Option<bool>,
}

const KJERSEY_PATTERN: [Dilemma; 5] = [
    Dilemma::Silence,
    Dilemma::Silence,
    Dilemma::Silence,
    Dilemma::Betray,
    Dilemma::Silence,
];

impl KJersey {
    fn check_pattern(&mut self, history: &[Turn]) -> Dilemma {
        let mut found: bool = true;

        for (turn, pattern) in history
            .iter()
            .map(|t| &t.other_bot)
            .zip(HELPER_PATTERN.iter())
        {
            if turn != pattern {
                found = false;
                break;
            }
        }

        self.helped = Some(found);
        self.rest(history)
    }

    fn rest(&mut self, history: &[Turn]) -> Dilemma {
        match self.helped.unwrap() {
            true => Dilemma::Betray,
            false => history.last().unwrap().other_bot,
        }
    }
}

impl Bot for KJersey {
    fn new() -> Self {
        KJersey { helped: None }
    }

    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        let turn_number = history.len();
        match turn_number.cmp(&KJERSEY_PATTERN.len()) {
            Ordering::Less => KJERSEY_PATTERN[turn_number],
            Ordering::Equal => self.check_pattern(history),
            _ => self.rest(history),
        }
    }
}

// Helper

#[derive(Debug, Default)]
pub struct Helper {
    helping: Option<bool>,
}

const HELPER_PATTERN: [Dilemma; 5] = [
    Dilemma::Betray,
    Dilemma::Betray,
    Dilemma::Silence,
    Dilemma::Betray,
    Dilemma::Silence,
];

impl Helper {
    fn check_pattern(&mut self, history: &[Turn]) -> Dilemma {
        let mut found: bool = true;

        for (turn, pattern) in history
            .iter()
            .map(|t| &t.other_bot)
            .zip(KJERSEY_PATTERN.iter())
        {
            if turn != pattern {
                found = false;
                break;
            }
        }

        self.helping = Some(found);
        self.rest()
    }

    fn rest(&mut self) -> Dilemma {
        match self.helping.unwrap() {
            true => Dilemma::Silence,
            false => Dilemma::Betray,
        }
    }
}

impl Bot for Helper {
    fn new() -> Self {
        Helper { helping: None }
    }

    fn turn(&mut self, history: &[Turn]) -> Dilemma {
        let turn_number = history.len();
        match turn_number.cmp(&HELPER_PATTERN.len()) {
            Ordering::Less => HELPER_PATTERN[turn_number],
            Ordering::Equal => self.check_pattern(history),
            _ => self.rest(),
        }
    }
}
