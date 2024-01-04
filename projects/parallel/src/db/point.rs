use std::iter::FromIterator;
use super::*;


#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Default)]
pub struct Counter {
    pub n: u32,
}

impl AsRef<[u8]> for Point {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Point as *const u8,
                std::mem::size_of::<Point>(),
            )
        }
    }
}

impl Into<IVec> for Counter {
    fn into(self) -> IVec {
        IVec::from_iter(self.n.to_le_bytes())
    }
}