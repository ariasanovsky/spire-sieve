use libgdx_xs128::{rng::Random, RandomXS128};

use crate::seed::Seed;

impl Seed {
    pub fn juzuless_path(&self, path: [usize; 3]) -> bool {
        let mut rng = Random::new(self.seed as u64);
        const COMBAT_INCREMENT: f32 = 0.1f32;
        for length in path {
            let mut combat_threshold = COMBAT_INCREMENT;
            for _ in 0..length {
                // todo!("implement f32 for libgdx-xs128, c.f. https://github.com/gamerpuppy/sts_seed_search/blob/d475a098ba65faf66dc7a64fcd30d6937f69e10e/sts_impl.h#L66")
                let random = rng.next_u64() >> 40;
                let random = random as f64;
                let random = random * 5.9604644775390625E-8;
                let random = random as f32;
                if random < combat_threshold {
                    return false;
                }
                combat_threshold += COMBAT_INCREMENT;
            }
        }
        true
    }
}

#[cfg(test)]
mod test_juzuless_path {
    use crate::seed::Seed;

    #[test]
    fn long_juzuless_path() {
        let path = [9, 0, 0];
        let seed: Seed = b"           CY".into();
        assert!(seed.juzuless_path(path));
    }
}