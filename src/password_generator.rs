/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* password_generator.rs                                                                   :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/27 21:15:13 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/27 21:15:13 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

pub mod pool;

use pool::Pool;
use rdrand::RdRand;
use rand::{rngs::StdRng, RngCore, SeedableRng};

#[derive(Debug)]
pub struct PasswordGenerator {
    pool: Pool,
    length: u8,
    no_similar: bool
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            pool: Pool::default(),
            length: 8,
            no_similar: false
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
    #[allow(dead_code)]
    pub fn no_similar(mut self) -> Self {
        self.no_similar = true;
        self
    }
    pub fn finalize(self) -> String {
        let mut res: String = String::new();
        let mut str_pool: String = Pool::to_string(self.pool);
        if usize::try_from(self.length).unwrap() > str_pool.len() && self.no_similar == true {
                println!("[ FAILED ]: The password length is longer than String pool lenght.");
                return "Password generate failed".to_string();
        }
        loop {
            let mut rand: u32 = RdRand::new().unwrap().try_next_u32().unwrap();
            rand = StdRng::seed_from_u64(u64::from(rand)).next_u32();
            let rand_char_idx: usize = usize::try_from(rand).unwrap() % str_pool.len();
            res.push(str_pool.chars().nth(rand_char_idx).unwrap());
            if self.no_similar == true {
                str_pool.remove(rand_char_idx);
            }
            if self.length == u8::try_from(res.len()).unwrap() {
                break;
            }
        }
        res
    }
}
