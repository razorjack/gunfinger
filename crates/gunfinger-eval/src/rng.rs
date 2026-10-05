//! A small seeded random number generator, so every evaluation draw is
//! repeatable from its seed.

/// SplitMix64 (Steele, Lea & Flood 2014): a 64-bit counter passed through a
/// strong mixing function. Plenty for choosing excerpts.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, 1)`, from the top 53 bits.
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1_u64 << 53) as f64
    }

    /// Uniform in `0..bound`. The modulo bias is below 2^-40 for any bound
    /// used here.
    pub fn below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }

    /// Fisher-Yates shuffle.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for last in (1..items.len()).rev() {
            items.swap(last, self.below(last + 1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_seed_always_gives_the_same_sequence() {
        let mut first = Rng::new(7);
        let mut second = Rng::new(7);

        for _ in 0..10 {
            assert_eq!(first.next_u64(), second.next_u64());
        }
    }

    #[test]
    fn shuffling_keeps_every_item() {
        let mut items: Vec<u32> = (0..50).collect();

        Rng::new(1).shuffle(&mut items);

        let mut sorted = items.clone();
        sorted.sort_unstable();
        assert_ne!(items, sorted);
        assert_eq!(sorted, (0..50).collect::<Vec<u32>>());
    }

    #[test]
    fn unit_values_stay_in_range() {
        let mut rng = Rng::new(3);

        assert!(
            (0..1000)
                .map(|_| rng.unit())
                .all(|value| (0.0..1.0).contains(&value))
        );
    }
}
