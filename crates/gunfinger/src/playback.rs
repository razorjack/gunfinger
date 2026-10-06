//! Which playbacks a search assumes: the turntable ladder, key-locked rungs
//! (tempo only, as on CDJs with master tempo) or both. Both doubles the
//! search time (experiment 0010).

use clap::ValueEnum;
use gunfinger_core::speed::{Rung, key_lock_ladder, ladder};

#[derive(Clone, Copy, ValueEnum)]
pub enum PlaybackChoice {
    /// Pitch and tempo together, as on vinyl.
    Turntable,
    /// Tempo only, pitch unchanged.
    KeyLock,
    /// Either.
    Both,
}

impl PlaybackChoice {
    pub fn rungs(self) -> Vec<Rung> {
        match self {
            PlaybackChoice::Turntable => ladder(),
            PlaybackChoice::KeyLock => key_lock_ladder(),
            PlaybackChoice::Both => ladder().into_iter().chain(key_lock_ladder()).collect(),
        }
    }
}
