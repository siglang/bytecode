use std::num::ParseIntError;

#[inline(always)]
pub fn str_to_isize(s: &str) -> Result<isize, ParseIntError> {
    isize::from_str_radix(s, 16)
}

#[inline(always)]
pub fn str_to_usize(s: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(s, 16)
}
