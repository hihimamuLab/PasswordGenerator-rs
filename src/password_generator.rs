/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* password_generator.rs                                                                   :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/21 22:39:06 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/23 17:29:24 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

pub mod method;
pub mod pool;
pub mod hash;

use method::Method;
use pool::Pool;

#[derive(Debug, PartialEq)]
pub struct PasswordGenerator {
    pub pool: Pool,
    pub method: Method,
    pub disable_str: String,
    pub length: u8,
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
    pub fn pool(mut self, pool: Pool) -> Self {
        self.pool = pool;
        self
    }
    pub fn method(mut self, method: Method) -> Self {
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
