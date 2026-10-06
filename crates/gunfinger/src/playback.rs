//! Which playbacks a search assumes: the turntable ladder, key-locked rungs
//! (tempo only, as on CDJs with master tempo) or both. Both makes the
//! search about a third slower (experiment 0016).

use clap::ValueEnum;
use gunfinger_core::speed::{Rung, key_lock_ladder, ladder};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlaybackChoice {
    /// Pitch and tempo together, as on vinyl.
    Turntable,
    /// Tempo only, pitch unchanged.
    KeyLock,
    /// Either.
    Both,
}

impl PlaybackChoice {
    pub fn name(self) -> &'static str {
        match self {
            PlaybackChoice::Turntable => "turntable",
            PlaybackChoice::KeyLock => "key-lock",
            PlaybackChoice::Both => "both",
        }
    }

    pub fn rungs(self) -> Vec<Rung> {
        match self {
            PlaybackChoice::Turntable => ladder(),
            PlaybackChoice::KeyLock => key_lock_ladder(),
            PlaybackChoice::Both => ladder().into_iter().chain(key_lock_ladder()).collect(),
        }
    }
}
