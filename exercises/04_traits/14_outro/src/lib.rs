use std::ops::Add;

// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct SaturatingU16 {
    value: u16
}

impl PartialEq for SaturatingU16 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other 
    }
}

impl SaturatingU16 {
    fn check_addition(val1: u16, val2: u16) -> u16 {
        val1.saturating_add(val2)
    }
}
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 {
            value
        }
    }
}

impl From<u8> for SaturatingU16{
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value as u16
        }
    }
}

impl From<&u16> for SaturatingU16{
    fn from(value: &u16) -> Self {
        SaturatingU16 {
            value: (*value) 
        }
    }
}

impl From<&u8> for SaturatingU16{
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: (*value) as u16
        }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: SaturatingU16::check_addition(self.value, rhs.value)
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 {
            value: SaturatingU16::check_addition(self.value, rhs)
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16 {
            value: SaturatingU16::check_addition(self.value, *rhs)
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: SaturatingU16::check_addition(self.value, (*rhs).value)
        }
    }
}