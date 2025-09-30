use serde::{Deserialize, Serialize};
use core::ops::{BitAnd, BitOr, BitXor, Not};

pub trait BitwisePermissions: Sized {
    type Bits: Copy
    + BitAnd<Output = Self::Bits>
    + BitOr<Output = Self::Bits>
    + BitXor<Output = Self::Bits>
    + Not<Output = Self::Bits>
    + PartialEq;
    type Permission: Copy;

    const EMPTY: Self::Bits;
    const ALL: Self::Bits;

    fn bits(&self) -> Self::Bits;
    fn set_bits(&mut self, bits: Self::Bits);
    fn from_bits(bits: Self::Bits) -> Self;

    fn mask(p: Self::Permission) -> Self::Bits;

    #[inline]
    fn empty() -> Self {
        Self::from_bits(Self::EMPTY)
    }

    #[inline]
    fn all() -> Self {
        Self::from_bits(Self::ALL)
    }

    #[inline]
    fn contains(&self, perm: Self::Permission) -> bool {
        let m = Self::mask(perm);
        (self.bits() & m) == m
    }

    #[inline]
    fn contains_all(&self, other: &Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }

    #[inline]
    fn insert(&mut self, perm: Self::Permission) {
        self.set_bits(self.bits() | Self::mask(perm));
    }

    #[inline]
    fn remove(&mut self, perm: Self::Permission) {
        self.set_bits(self.bits() & !Self::mask(perm));
    }

    #[inline]
    fn toggle(&mut self, perm: Self::Permission) {
        self.set_bits(self.bits() ^ Self::mask(perm));
    }

    #[inline]
    fn clear(&mut self) {
        self.set_bits(Self::EMPTY);
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.bits() == Self::EMPTY
    }

    #[inline]
    fn is_all(&self) -> bool {
        self.bits() == Self::ALL
    }

    #[inline]
    fn plus(&self, other: &Self) -> Self {
        Self::from_bits(self.bits() | other.bits())
    }

    #[inline]
    fn minus(&self, other: &Self) -> Self {
        Self::from_bits(self.bits() & !other.bits())
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealmPermissions(pub i16);

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealmPermission {
    ManageEvents  = 0b0001,
    ManageTasks   = 0b0010,
}

impl BitwisePermissions for RealmPermissions {
    type Bits = i16;
    type Permission = RealmPermission;

    const EMPTY: i16 = 0;
    const ALL: i16 = i16::MAX;

    #[inline] fn bits(&self) -> i16 { self.0 }
    #[inline] fn set_bits(&mut self, bits: i16) { self.0 = bits; }
    #[inline] fn from_bits(bits: i16) -> Self { Self(bits) }
    #[inline] fn mask(p: RealmPermission) -> i16 { p as i16 }
}

impl RealmPermissions {
    #[inline] pub const fn new(value: i16) -> Self { Self(value) }

    #[inline]
    pub fn from_slice(perms: &[RealmPermission]) -> Self {
        let mut bits = 0i16;
        for &p in perms { bits |= p as i16; }
        Self(bits)
    }
}