/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* password_generator.rs                                                                   :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/21 22:39:06 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/21 22:57:38 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

#[derive(Debug, PartialEq)]
pub struct PasswordGenerator {
    pub pool: PasswordGeneratorPool,
    pub method: PasswordGeneratorMethod,
    pub disable_str: String,
    pub length: u8,
}

#[derive(Debug, PartialEq)]
pub enum PasswordGeneratorMethod  {
    SHA2512,
    SHA3512,
    BLAKE3,
    Whirlpool
}

#[derive(Debug, PartialEq)]
pub enum PasswordGeneratorPool {
    a_Z(String),
    a_Z0_9(String),
    a_Z0_9symbol(String)
}

impl PasswordGenerator {
    fn new() -> Self {
        Self {
            pool: PasswordGeneratorPool::a_Z("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()),
            method: PasswordGeneratorMethod::SHA2512,
            disable_str: "".to_string(),
            length: 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::password_generator::{PasswordGenerator, PasswordGeneratorPool, PasswordGeneratorMethod};

    #[test]
    fn pwgen_new_test() {
        let pwgen_new: PasswordGenerator = PasswordGenerator::new();
        assert_eq!(pwgen_new, PasswordGenerator {
            pool: PasswordGeneratorPool::a_Z("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()),
            method: PasswordGeneratorMethod::SHA2512,
            disable_str: "".to_string(),
            length: 8,
        })
    }
}

