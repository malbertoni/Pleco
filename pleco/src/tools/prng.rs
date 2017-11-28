//! Contains the Pseduo-random number generator.

/// Object for generating pseudo-random numbers.
pub struct PRNG {
    seed: u64,
}

impl PRNG {
    /// Creates PRNG from a seed.
    ///
    /// # Panics
    ///
    /// Panics if the seed is zero.
    pub fn init(s: u64) -> PRNG {
        assert_ne!(s, 0);
        PRNG { seed: s }
    }

    /// Returns a pseudo-random number.
    #[allow(dead_code)]
    pub fn rand(&mut self) -> u64 {
        self.rand_change()
    }

    /// Returns a pseudo-random number with on average 8 bits being set.
    pub fn sparse_rand(&mut self) -> u64 {
        let mut s = self.rand_change();
        s &= self.rand_change();
        s &= self.rand_change();
        s
    }

    /// Returns a u64 with exactly one bit set in a random location.
    pub fn singular_bit(&mut self) -> u64 {
        let num: u64 = 0;
        num.wrapping_shl(self.rand().count_ones())
    }

    /// Randomizes the current seed and returns a random value.
    fn rand_change(&mut self) -> u64 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;
        self.seed.wrapping_mul(2685_8216_5773_6338_717)
    }
}