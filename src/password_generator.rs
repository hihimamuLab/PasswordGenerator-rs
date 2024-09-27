pub mod pool;

use pool::Pool;
use rdrand::RdRand;

pub struct PasswordGenerator {
    pool: Pool,
    length: u8,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            pool: Pool::default(),
            length: 8,
        }
    }
}

impl PasswordGenerator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn pool(mut self, pool: Pool) -> Self {
        self.pool = pool;
        self
    }
    pub fn length(mut self, length: u8) -> Self {
        self.length = length;
        self
    }
    pub fn finalize(self) -> String {
        let mut res: String = String::new();
        let str_pool: String = Pool::to_string(self.pool);
        loop {
            let rand: u32 = RdRand::new().unwrap().try_next_u32().unwrap();
            res.push(
                str_pool
                    .chars()
                    .nth(usize::try_from(rand).unwrap() % str_pool.len())
                    .unwrap(),
            );
            if self.length == u8::try_from(res.len()).unwrap() {
                break;
            }
        }
        res
    }
}
