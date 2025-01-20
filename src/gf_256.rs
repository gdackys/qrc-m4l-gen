pub struct GF256 {
    exp: [u8; 512],
    log: [u8; 256],
}

impl GF256 {
    pub fn new() -> Self {
        let mut gf_256 = GF256 {
            exp: [0; 512],
            log: [0; 256],
        };
        gf_256.init();
        gf_256
    }

    fn init(&mut self) {
        let mut x: u16 = 1;

        // Initialize exp and log tables for i < 256
        for i in 0..256 {
            self.exp[i] = x as u8;
            self.log[x as usize] = i as u8;

            x <<= 1;

            if (x & 0x100) != 0 {
                x ^= 0x11d; // The primitive polynomial x^8 + x^4 + x^3 + x^2 + 1
            }
        }

        // Fill in remaining exp values for i >= 256
        for i in 255..512 {
            self.exp[i] = self.exp[i - 255];
        }
    }

    pub fn multiply(&self, a: usize, b: usize) -> u8 {
        if a == 0 || b == 0 {
            return 0;
        }

        let exp_idx = (self.log[a] as usize + self.log[b] as usize) % 255;

        self.exp[exp_idx]
    }
}
