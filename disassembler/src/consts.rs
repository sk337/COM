use std::fmt;
use std::ops::*;

/// address size in bits
pub const SIZE: u32 = 16;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Address(pub u16);

impl Address {
    pub fn get(self) -> u16 {
        self.0
    }
}

// -- Conversions --
macro_rules! impl_from_for_address {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Address {
                fn from(value: $t) -> Self {
                    Address(value as u16)
                }
            }
        )*
    };
}
impl_from_for_address!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl From<Address> for u16 {
    fn from(addr: Address) -> Self {
        addr.0
    }
}

// -- Display / Debug --
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

// -- Arithmetic Operators --
impl Add<u16> for Address {
    type Output = Address;
    fn add(self, rhs: u16) -> Self::Output {
        Address(self.0.wrapping_add(rhs))
    }
}

impl Sub<u16> for Address {
    type Output = Address;
    fn sub(self, rhs: u16) -> Self::Output {
        Address(self.0.wrapping_sub(rhs))
    }
}

impl AddAssign<u16> for Address {
    fn add_assign(&mut self, rhs: u16) {
        self.0 = self.0.wrapping_add(rhs);
    }
}

impl SubAssign<u16> for Address {
    fn sub_assign(&mut self, rhs: u16) {
        self.0 = self.0.wrapping_sub(rhs);
    }
}

// -- Bitwise Operators --
impl BitAnd<u16> for Address {
    type Output = Address;
    fn bitand(self, rhs: u16) -> Self::Output {
        Address(self.0 & rhs)
    }
}

impl BitOr<u16> for Address {
    type Output = Address;
    fn bitor(self, rhs: u16) -> Self::Output {
        Address(self.0 | rhs)
    }
}

impl BitXor<u16> for Address {
    type Output = Address;
    fn bitxor(self, rhs: u16) -> Self::Output {
        Address(self.0 ^ rhs)
    }
}

// Optional: Neg for fun
impl Neg for Address {
    type Output = Address;
    fn neg(self) -> Self::Output {
        Address(self.0.wrapping_neg())
    }
}

// --- Arithmetic with Address ---
impl Add for Address {
    type Output = Address;
    fn add(self, rhs: Address) -> Self::Output {
        Address(self.0.wrapping_add(rhs.0))
    }
}

impl Sub for Address {
    type Output = Address;
    fn sub(self, rhs: Address) -> Self::Output {
        Address(self.0.wrapping_sub(rhs.0))
    }
}

impl AddAssign for Address {
    fn add_assign(&mut self, rhs: Address) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}

impl SubAssign for Address {
    fn sub_assign(&mut self, rhs: Address) {
        self.0 = self.0.wrapping_sub(rhs.0);
    }
}

// --- Bitwise with Address ---
impl BitAnd for Address {
    type Output = Address;
    fn bitand(self, rhs: Address) -> Self::Output {
        Address(self.0 & rhs.0)
    }
}

impl BitOr for Address {
    type Output = Address;
    fn bitor(self, rhs: Address) -> Self::Output {
        Address(self.0 | rhs.0)
    }
}

impl BitXor for Address {
    type Output = Address;
    fn bitxor(self, rhs: Address) -> Self::Output {
        Address(self.0 ^ rhs.0)
    }
}
