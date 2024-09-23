/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* password_generator.rs                                                                   :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/21 22:39:06 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/23 10:45:01 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

mod pool;

#[derive(Debug, PartialEq)]
pub struct PasswordGenerator {
    pub pool: pool::Pool,
    pub method: PasswordGeneratorMethod,
    pub disable_str: String,
    pub length: u8,
}

#[derive(Debug, PartialEq, Default)]
pub enum PasswordGeneratorMethod {
    SHA2512,
    SHA3512,
    #[default]
    BLAKE3,
    Whirlpool,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        Self {
            pool: Default::default(),
            method: Default::default(),
            disable_str: "".to_string(),
            length: 8,
        }
    }
    pub fn pool(mut self, pool: pool::Pool) -> Self{
        self.pool = pool;
        self
    }
    pub fn method(mut self, method: PasswordGeneratorMethod) -> Self {
        self.method = method;
        self
    }
    pub fn disable_str(mut self, disable_str: String) -> Self {
        self.disable_str = disable_str;
        self
    }
    pub fn length(mut self, length: u8) -> Self {
        self.length = length;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::password_generator::{
        PasswordGenerator, PasswordGeneratorMethod,
    };
    use crate::password_generator::pool::Pool;

    #[test]
    fn pwgen_new_test() {
        let pwgen_new: PasswordGenerator = PasswordGenerator::new();
        assert_eq!(
            pwgen_new,
            PasswordGenerator {
                pool: Pool::UPPERCASE | Pool::LOWERCASE,
                 method: PasswordGeneratorMethod::BLAKE3,
                disable_str: "".to_string(),
                length: 8,
            }
        );
    }
    #[test]
    fn pwgen_pool_test() {
        let pwgen_pool: PasswordGenerator =
            PasswordGenerator::new().pool(Pool::UPPERCASE);
        assert_eq!(
            pwgen_pool,
            PasswordGenerator {
                pool: Pool::UPPERCASE,
                method: PasswordGeneratorMethod::BLAKE3,
                disable_str: "".to_string(),
                length: 8,
            }
        )
    }
    #[test]
    fn pwgen_method_test() {
        let pwgen_method: PasswordGenerator = 
            PasswordGenerator::new().method(PasswordGeneratorMethod::SHA2512);
        assert_eq!(
            pwgen_method,
            PasswordGenerator {
                pool: Pool::UPPERCASE | Pool::LOWERCASE,
                method: PasswordGeneratorMethod::SHA2512,
                disable_str: "".to_string(),
                length: 8
            }
        )
    }
    #[test]
    fn pwgen_disable_str_test() {
        let pwgen_disable_str: PasswordGenerator =
            PasswordGenerator::new().disable_str("abc".to_string());
        assert_eq!(
            pwgen_disable_str,
            PasswordGenerator {
                pool: Pool::UPPERCASE | Pool::LOWERCASE,
                method: PasswordGeneratorMethod::BLAKE3,
                disable_str: "abc".to_string(),
                length: 8
            }
        )
    }
    #[test]
    fn pwgen_length_test() {
        let pwgen_length: PasswordGenerator = 
            PasswordGenerator::new().length(16);
        assert_eq!(
            pwgen_length,
            PasswordGenerator {
                pool: Pool::UPPERCASE | Pool::LOWERCASE,
                method: PasswordGeneratorMethod::BLAKE3,
                disable_str: "".to_string(),
                length: 16,
            }
        )
    }
}
