/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* password_generator.rs                                                                   :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/21 22:39:06 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/22 14:44:23 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

use std::ops::BitOr;

#[derive(Debug, PartialEq)]
pub struct PasswordGenerator<'a> {
    pub pool: PasswordGeneratorPool,
    pub method: PasswordGeneratorMethod,
    pub disable_str: &'a str,
    pub length: u8,
}

#[derive(Debug, PartialEq, Default)]
pub enum PasswordGeneratorMethod {
    SHA2512,
    SHA3512,
    #[default]
    BLAKE3,
    Whirlpool
}

#[derive(Debug, PartialEq)]
pub struct PasswordGeneratorPool {
    value: i32
}

impl PasswordGeneratorPool {
    pub const UPPERCASE: Self = PasswordGeneratorPool { value: 1 };
    pub const LOWERCASE: Self = PasswordGeneratorPool { value: 2 };
    pub const SYMBOL: Self = PasswordGeneratorPool { value: 4  };
}

impl BitOr for PasswordGeneratorPool {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value
        }
    }
}

impl Default for PasswordGeneratorPool {
    fn default() -> Self {
        Self::UPPERCASE | Self::LOWERCASE
    }
}

impl PasswordGenerator<'_> {
    pub fn new() -> Self {
        Self {
            pool: Default::default(),
            method: Default::default(),
            disable_str: "",
            length: 8,
        }
    }
    pub fn pool(self, pool: PasswordGeneratorPool) -> Self {
        Self {
            pool,
            method: self.method,
            disable_str: self.disable_str,
            length: self.length,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::password_generator::{PasswordGenerator, PasswordGeneratorMethod, PasswordGeneratorPool};

    #[test]
    fn pwgen_new_test() {
        let pwgen_new: PasswordGenerator = PasswordGenerator::new();
        assert_eq!(pwgen_new, PasswordGenerator {
            pool: PasswordGeneratorPool::UPPERCASE | PasswordGeneratorPool::LOWERCASE,
            method: PasswordGeneratorMethod::BLAKE3,
            disable_str: "",
            length: 8,
        });
    }
    #[test]
    fn pwgen_pool_test() {
        let pwgen_pool: PasswordGenerator = PasswordGenerator::new()
            .pool(PasswordGeneratorPool::UPPERCASE);
        assert_eq!(pwgen_pool, PasswordGenerator {
            pool: PasswordGeneratorPool::UPPERCASE,
            method: PasswordGeneratorMethod::BLAKE3,
            disable_str: "",
            length: 8,
        })
    }
}

