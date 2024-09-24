/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* tests.rs                                                                                :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/23 17:29:27 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/23 17:29:28 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

#[cfg(test)]
mod tests {
    use crate::password_generator::{method::Method, pool::Pool, hash::Hash, PasswordGenerator};

    #[test]
    fn pwgen_new_test() {
        let pwgen_new: PasswordGenerator = PasswordGenerator::new();
        assert_eq!(
            pwgen_new,
            PasswordGenerator {
                pool: Pool::UPPERCASE | Pool::LOWERCASE,
                method: Method::BLAKE3,
                disable_str: "".to_string(),
                length: 8,
            }
        )
    }
    #[test]
    fn pwgen_pool_test() {
        let pwgen_pool: PasswordGenerator = PasswordGenerator::new().pool(Pool::UPPERCASE);
        assert_eq!(
            pwgen_pool,
            PasswordGenerator {
                pool: Pool::UPPERCASE,
                method: Method::BLAKE3,
                disable_str: "".to_string(),
                length: 8
            }
        )
    }
    #[test]
    fn pwgen_method_test() {
        let pwgen_method: PasswordGenerator = PasswordGenerator::new().method(Method::SHA2512);
        assert_eq!(
            pwgen_method,
            PasswordGenerator {
                pool: Pool::UPPERCASE | Pool::LOWERCASE,
                method: Method::SHA2512,
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
                method: Method::BLAKE3,
                disable_str: "abc".to_string(),
                length: 8
            }
        )
    }
    #[test]
    fn pwgen_length_test() {
        let pwgen_length: PasswordGenerator = PasswordGenerator::new().length(16);
        assert_eq!(
            pwgen_length,
            PasswordGenerator {
                pool: Pool::UPPERCASE | Pool::LOWERCASE,
                method: Method::BLAKE3,
                disable_str: "".to_string(),
                length: 16
            }
        )
    }

    #[test]
    fn sha2512_test() {
        let sha2512: u32 = Hash::sha2512("hihimamu".to_string());
        assert_eq!(sha2512, 2726927521);
    }
}
