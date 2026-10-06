//! Synthetic music for end-to-end tests.
//!
//! A track is a list of procedural notes, so it can be played at any speed
//! exactly as a turntable plays a record: at speed `s`, note times divide by
//! `s` and frequencies multiply by it. Under key lock only the times divide.
//! No resampling is involved, and every track is reproducible from its
//! seed.

use std::fs;
use std::path::Path;

pub const RATE: u32 = 22_050;
/// 1/16 notes at 170 BPM.
const STEP_SECONDS: f64 = 60.0 / 170.0 / 4.0;

/// SplitMix64, as in `gunfinger-eval`.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in `[low, high)`.
    fn between(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * (self.next() >> 11) as f64 / (1_u64 << 53) as f64
    }
}

struct Note {
    start: f64,
    length: f64,
    /// Frequency in Hz and amplitude of each partial.
    partials: Vec<(f64, f64)>,
}

pub struct Track {
    notes: Vec<Note>,
    pub seconds: f64,
}

impl Track {
    /// A kick with the track's own pitch and one-bar pattern and, on most
    /// 1/16 steps, a decaying chord of two to four partials between 150 Hz
    /// and 3.5 kHz. Different seeds give unrelated tracks.
    pub fn random(seed: u64, seconds: f64) -> Track {
        let mut rng = Rng(seed);
        let kick_hz = rng.between(45.0, 90.0);
        let kick_pattern: Vec<bool> = (0..16).map(|_| rng.between(0.0, 1.0) < 0.3).collect();
        let mut notes = Vec::new();
        let steps = (seconds / STEP_SECONDS) as usize;
        for step in 0..steps {
            let start = step as f64 * STEP_SECONDS;
            if kick_pattern[step % 16] {
                notes.push(Note {
                    start,
                    length: 0.25,
                    partials: vec![(kick_hz, 0.6)],
                });
            }
            if rng.between(0.0, 1.0) < 0.7 {
                let count = rng.between(2.0, 5.0) as usize;
                let partials = (0..count)
                    .map(|_| {
                        let hz = (rng.between(150_f64.ln(), 3500_f64.ln())).exp();
                        (hz, rng.between(0.05, 0.25))
                    })
                    .collect();
                notes.push(Note {
                    start,
                    length: rng.between(0.08, 0.4),
                    partials,
                });
            }
        }
        Track { notes, seconds }
    }

    /// `seconds` of output starting at track time `from`, played at `speed`
    /// on a turntable.
    pub fn play(&self, from: f64, seconds: f64, speed: f64) -> Vec<f32> {
        self.render(from, seconds, speed, speed)
    }

    /// Like `play`, with key lock: the tempo changes, the pitch does not.
    pub fn play_key_locked(&self, from: f64, seconds: f64, tempo: f64) -> Vec<f32> {
        self.render(from, seconds, tempo, 1.0)
    }

    fn render(&self, from: f64, seconds: f64, speed: f64, pitch: f64) -> Vec<f32> {
        let rate = f64::from(RATE);
        let length = (seconds * rate) as usize;
        let mut out = vec![0.0_f64; length];
        for note in &self.notes {
            let onset = (note.start - from) / speed;
            let duration = note.length / speed;
            let first = (onset * rate).ceil().max(0.0) as usize;
            let last = (((onset + duration) * rate) as usize).min(length);
            for (index, sample) in out.iter_mut().enumerate().take(last).skip(first) {
                let since = index as f64 / rate - onset;
                // A 5 ms attack, then an exponential decay over the note.
                let envelope = (since / 0.005).min(1.0) * (-4.0 * since / duration).exp();
                for &(hz, amplitude) in &note.partials {
                    *sample +=
                        amplitude * envelope * (std::f64::consts::TAU * hz * pitch * since).sin();
                }
            }
        }
        out.iter().map(|&sample| (0.3 * sample) as f32).collect()
    }
}

/// A mix being assembled: tracks placed at times, with linear fades.
pub struct Mix {
    samples: Vec<f32>,
}

impl Mix {
    pub fn new(seconds: f64) -> Mix {
        Mix {
            samples: vec![0.0; (seconds * f64::from(RATE)) as usize],
        }
    }

    /// Adds `audio` at `at` seconds, fading in and out over `fade` seconds.
    pub fn add(&mut self, at: f64, audio: &[f32], fade: f64) {
        let offset = (at * f64::from(RATE)) as usize;
        let fade_samples = (fade * f64::from(RATE)).max(1.0);
        for (index, &value) in audio.iter().enumerate() {
            let Some(slot) = self.samples.get_mut(offset + index) else {
                break;
            };
            let from_start = index as f64;
            let from_end = (audio.len() - index) as f64;
            let gain = (from_start / fade_samples)
                .min(from_end / fade_samples)
                .min(1.0);
            *slot += value * gain as f32;
        }
    }

    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
}

/// Writes 16-bit mono PCM WAV.
pub fn write_wav(path: &Path, samples: &[f32]) {
    let data_bytes = u32::try_from(samples.len() * 2).unwrap();
    let mut bytes = Vec::with_capacity(44 + samples.len() * 2);
    bytes.extend(b"RIFF");
    bytes.extend((36 + data_bytes).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes()); // PCM
    bytes.extend(1_u16.to_le_bytes()); // mono
    bytes.extend(RATE.to_le_bytes());
    bytes.extend((RATE * 2).to_le_bytes());
    bytes.extend(2_u16.to_le_bytes());
    bytes.extend(16_u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(data_bytes.to_le_bytes());
    for &sample in samples {
        let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16;
        bytes.extend(value.to_le_bytes());
    }
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).unwrap();
    }
    fs::write(path, bytes).unwrap();
}
