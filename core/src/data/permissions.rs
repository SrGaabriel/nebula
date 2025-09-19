// rust
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

    // Convert a permission to its bit mask.
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
pub struct RealmPermissions(u8);

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealmPermission {
    Read  = 0b0001,
    Write = 0b0010,
    Admin = 0b0100,
}

impl BitwisePermissions for RealmPermissions {
    type Bits = u8;
    type Permission = RealmPermission;

    const EMPTY: u8 = 0;
    const ALL: u8 = 0xFF;

    #[inline] fn bits(&self) -> u8 { self.0 }
    #[inline] fn set_bits(&mut self, bits: u8) { self.0 = bits; }
    #[inline] fn from_bits(bits: u8) -> Self { Self(bits) }
    #[inline] fn mask(p: RealmPermission) -> u8 { p as u8 }
}

impl RealmPermissions {
    #[inline] pub const fn new(value: u8) -> Self { Self(value) }

    #[inline] pub const fn read() -> Self { Self(RealmPermission::Read as u8) }
    #[inline] pub const fn write() -> Self { Self(RealmPermission::Write as u8) }
    #[inline] pub const fn admin() -> Self { Self(RealmPermission::Admin as u8) }

    #[inline] pub fn can_read(&self) -> bool { <Self as BitwisePermissions>::contains(self, RealmPermission::Read) }
    #[inline] pub fn can_write(&self) -> bool { <Self as BitwisePermissions>::contains(self, RealmPermission::Write) }
    #[inline] pub fn is_admin(&self) -> bool { <Self as BitwisePermissions>::contains(self, RealmPermission::Admin) }

    #[inline]
    pub fn to_array(self) -> ([Option<RealmPermission>; 3], usize) {
        let mut arr = [None, None, None];
        let mut len = 0;
        if self.can_read()  { arr[len] = Some(RealmPermission::Read);  len += 1; }
        if self.can_write() { arr[len] = Some(RealmPermission::Write); len += 1; }
        if self.is_admin()  { arr[len] = Some(RealmPermission::Admin); len += 1; }
        (arr, len)
    }

    #[inline]
    pub fn from_slice(perms: &[RealmPermission]) -> Self {
        let mut bits = 0u8;
        for &p in perms { bits |= p as u8; }
        Self(bits)
    }
}