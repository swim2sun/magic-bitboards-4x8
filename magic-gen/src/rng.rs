// Simple Pcg64Mcg implementation
pub struct Rng(u128);

impl Default for Rng {
    fn default() -> Self {
        // Use a new, full 128-bit odd seed
        Self(0xF0E1_D2C3_B4A5_9687_7869_5A4B_3C2D_1E0F_u128)
    }
}

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645_u128);
        let rot = (self.0 >> 122) as u32;
        let xsl = (self.0 >> 64) as u64 ^ self.0 as u64;
        xsl.rotate_right(rot)
    }

    pub fn next_u32(&mut self) -> u32 {
        // Advance the 128-bit state
        self.0 = self.0.wrapping_mul(0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645_u128);

        // Determine rotation amount from the top 5 bits of the state (for 0-31 rotation for u32)
        // self.0 is u128. (self.0 >> 123) gives the top (128-123) = 5 bits.
        let rot = (self.0 >> 123) as u32;

        // XOR-fold the 128-bit state down to 32 bits.
        // 1. Fold 128-bit to 64-bit (similar to next_u64's xsl part):
        let xsl64 = (self.0 >> 64) as u64 ^ self.0 as u64;
        // 2. Fold that 64-bit value to 32-bit:
        let xsl32 = (xsl64 >> 32) as u32 ^ xsl64 as u32;

        // Rotate and return
        xsl32.rotate_right(rot)
    }
}
