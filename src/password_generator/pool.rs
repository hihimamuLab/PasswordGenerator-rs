/* ************************************************************************************************************************************************ */
/*                                                                                                                                                  */
/*                                                                                           :::   :::        :::          :::   :::     :::    ::: */
/* pool.rs                                                                                 :+:+: :+:+:      :+: :+:      :+:+: :+:+:    :+:    :+:  */
/*                                                                                       +:+ +:+:+ +:+    +:+   +:+    +:+ +:+:+ +:+   +:+    +:+   */
/* By: hihimamu <hihimamu@gmail.com>                                                    +#+  +:+  +#+   +#++:++#++:   +#+  +:+  +#+   +#+    +:+    */
/*                                                                                     +#+       +#+   +#+     +#+   +#+       +#+   +#+    +#+     */
/* Created: 2024/09/23 10:34:21 by hihimamu                                           #+#       #+#   #+#     #+#   #+#       #+#   #+#    #+#      */
/* Updated: 2024/09/23 13:53:52 by hihimamu                                          ###       ###   ###     ###   ###       ###    ########.       */
/*                                                                                                                                                  */
/* ************************************************************************************************************************************************ */

use std::{ops::BitOr, ops::BitAnd,  process::Output};

#[derive(Debug, PartialEq)]
pub struct Pool {
    pub value: u8,
}

impl Pool {
    pub const UPPERCASE: Self = Pool { value: 1 };
    pub const LOWERCASE: Self = Pool { value: 2 };
    pub const SYMBOL: Self = Pool { value: 4 };
}

impl BitOr for Pool {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}

impl BitAnd for Pool {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value:  self.value & rhs.value
        }
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self::UPPERCASE | Self::LOWERCASE
    }
}
