use std::ops::{BitOr, BitAnd};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pool(u8);

impl Pool {
    pub const UPPERCASE: Self = Self(1);
    pub const LOWERCASE: Self = Self(2);
    pub const NUMBER: Self = Self(4);
    pub const SYMBOL: Self = Self(8);
}

impl BitOr for Pool {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)    
    }
}

impl BitAnd for Pool {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self(3)
//     =Self(Pool::UPPERCASE.0 | Pool::LOWERCASE.0)
    }
}

impl Pool {
    pub(in crate::password_generator) fn to_string(pool: Pool) -> String {
        let mut res: String = String::new();
        if pool & Pool::UPPERCASE == Pool::UPPERCASE {
            res.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); 
        }
        if pool & Pool::LOWERCASE == Pool::LOWERCASE {
            res.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if pool & Pool::NUMBER == Pool::NUMBER {
            res.push_str("1234567890");
        }
        if pool & Pool::SYMBOL == Pool::SYMBOL {
            res.push_str("!@#$%^&*()-_=+[{{]}}\\|;:'\"");
        }
        res
    }
}
