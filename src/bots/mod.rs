use strum_macros::EnumIter;

use self::{
    alanreviews::AlanReviews, allie::Allie, always_betray::AlwaysBetray,
    always_silence::AlwaysSilence, angelonfira::AngelOnFira, detective::Detective,
    fifty_fifty::FiftyFifty, grim_trigger::GrimTrigger, joss::Joss, kjersey::KJersey,
    nguyen_hanh_nong::NguyenHanhNong, williamdual::Williamdual, zakuarbor::ZakuArbor,
};

mod alanreviews;
mod allie;
mod always_betray;
mod always_silence;
mod angelonfira;
mod detective;
mod fifty_fifty;
mod grim_trigger;
mod joss;
mod kjersey;
mod nguyen_hanh_nong;
mod williamdual;
mod zakuarbor;

pub trait Bot {
    fn new() -> Self
    where
        Self: Sized;

    fn turn(&mut self, history: &[Turn]) -> Dilemma;
}

pub struct Turn {
    pub you: Dilemma,
    pub other_bot: Dilemma,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Dilemma {
    Silence,
    Betray,
}

#[derive(Debug, EnumIter, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Bots {
    AngelOnFira,
    FiftyFifty,
    AlwaysSilence,
    AlwaysBetray,
    GrimTrigger,
    Detective,
    NguyenHanhNong,
    ZakuArbor,
    Williamdual,
    Allie,
    KJersey,
    KJerseyHelper1,
    KJerseyHelper2,
    Joss,
    AlanReviews,
}

impl Bots {
    pub fn objects(&self) -> Box<dyn Bot> {
        match self {
            Bots::AngelOnFira => Box::new(AngelOnFira::new()),
            Bots::FiftyFifty => Box::new(FiftyFifty::new()),
            Bots::AlwaysSilence => Box::new(AlwaysSilence::new()),
            Bots::AlwaysBetray => Box::new(AlwaysBetray::new()),
            Bots::GrimTrigger => Box::new(GrimTrigger::new()),
            Bots::Detective => Box::new(Detective::new()),
            Bots::NguyenHanhNong => Box::new(NguyenHanhNong::new()),
            Bots::ZakuArbor => Box::new(ZakuArbor::new()),
            Bots::Williamdual => Box::new(Williamdual::new()),
            Bots::Allie => Box::new(Allie::new()),
            Bots::KJersey => Box::new(KJersey::new()),
            Bots::KJerseyHelper1 => Box::new(kjersey::Helper::new()),
            Bots::KJerseyHelper2 => Box::new(kjersey::Helper::new()),
            Bots::Joss => Box::new(Joss::new()),
            Bots::AlanReviews => Box::new(AlanReviews::new()),
        }
    }
}
