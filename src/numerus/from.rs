use crate::Numerus;

use std::convert::TryInto;

impl From<Numerus> for i16 {
    fn from(n: Numerus) -> Self {
        debug_assert!(Numerus::MAX.vis < i16::MAX as u16);
        n.vis.try_into().unwrap()
    }
}

impl From<Numerus> for i32 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

impl From<Numerus> for i64 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

impl From<Numerus> for i128 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

impl From<Numerus> for isize {
    fn from(n: Numerus) -> Self {
        debug_assert!((Numerus::MAX.vis as usize) < (isize::MAX as usize));
        n.vis.try_into().unwrap()
    }
}

impl From<Numerus> for u16 {
    fn from(n: Numerus) -> Self {
        n.vis
    }
}

impl From<Numerus> for u32 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

impl From<Numerus> for u64 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

impl From<Numerus> for u128 {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}

impl From<Numerus> for usize {
    fn from(n: Numerus) -> Self {
        n.vis.into()
    }
}
