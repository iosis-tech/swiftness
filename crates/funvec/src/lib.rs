use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize};
use solana_program::log::sol_log_64;

pub const FUNVEC_LAYERS: usize = 5;
pub const FUNVEC_OODS: usize = 256;
pub const FUNVEC_LEAVES: usize = 256;
pub const FUNVEC_AUTHENTICATIONS: usize = 256;
pub const FUNVEC_LAST_LAYER: usize = 256;
pub const FUNVEC_DECOMMITMENT_VALUES: usize = 256;
pub const FUNVEC_PAGES: usize = 1024;
pub const FUNVEC_SEGMENTS: usize = 12;
pub const FUNVEC_QUERIES: usize = 128;
pub const FUNVEC_COLUMN_VALUES: usize = 15;
pub fn print_address<T>(address: &T, label: u64) {
    sol_log_64(
        std::ptr::addr_of!(address) as u64,          // iteration
        std::ptr::addr_of!(address) as u64 & 0x3fff, // input variable
        std::ptr::addr_of!(address) as u64 & 0x3fff, // local variable
        (std::ptr::addr_of!(address) as u64 - 0x200000000) >> 12, // frame number
        label,
    );
}

#[test]
fn test_print_address() {
    let a: u64 = 0x7ffcbd7e45d8;
    let b: u64 = 0x688;

    let a = a % 0x3fff - 2970;
    assert_eq!(a, b);
}

#[inline(never)]
pub fn print_frame(i: u64, label: u64) {
    if i == 62 {
        return; // otherwise we hit the max call depth
    }

    // Solana uses 4kB, so 4096 bytes, that gives us 14 bits to address the frame, so a mask of 0x3fff

    let var = 10 * i;
    // sol_log_64(
    //     i,                                                                   // iteration
    //     std::ptr::addr_of!(i) as u64 & 0x3fff,                               // input variable
    //     std::ptr::addr_of!(var) as u64 & 0x3fff,                             // local variable
    //     (std::ptr::addr_of!(var) as u64 & 0xfffffff000 - 0x200000000) >> 12, // frame number
    //     label,
    // );

    sol_log_64(
        std::ptr::addr_of!(var) as u64,                       // iteration
        std::ptr::addr_of!(var) as u64 & 0x3fff,              // input variable
        std::ptr::addr_of!(var) as u64 & 0x3fff,              // local variable
        (std::ptr::addr_of!(var) as u64 - 0x200000000) >> 12, // frame number
        label,
    );

    // 0x0, 0x7ffec715d0c0, 0x7ffec715d0c8, 0x7ffcc715d, 0x31

    // print_frame(i + 1);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FunVec<T: Default, const N: usize> {
    len: usize,
    data: [T; N],
}

impl<T: Default, const N: usize> Default for FunVec<T, N> {
    fn default() -> Self {
        Self { len: 0, data: core::array::from_fn(|_| Default::default()) }
    }
}

impl<'de, T: Copy + Default + Deserialize<'de>, const N: usize> Deserialize<'de> for FunVec<T, N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let vec: Vec<T> = Vec::deserialize(deserializer)?;
        Ok(Self::from_vec(vec))
    }
}

impl<T: Copy + Default + Serialize, const N: usize> Serialize for FunVec<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let vec = self.to_vec();
        vec.serialize(serializer)
    }
}

impl<T: Copy + Default, const N: usize> FunVec<T, N> {
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut s = Self::default();
        s.data[..vec.len()].copy_from_slice(&vec);
        Self { len: vec.len(), data: s.data }
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.data[..self.len].to_vec()
    }

    pub fn to_vec_ref(&self) -> Vec<&T> {
        self.data[..self.len].iter().collect()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.len]
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data[..self.len]
    }

    pub fn unchecked_slice(&self, len: usize) -> &[T] {
        &self.data[..len]
    }

    pub fn unchecked_slice_mut(&mut self, len: usize) -> &mut [T] {
        &mut self.data[..len]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn at(&self, index: usize) -> &T {
        &self.data[index]
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(&self.data[index])
        }
    }

    pub fn at_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }

    pub fn push(&mut self, value: T) {
        self.data[self.len] = value;
        self.len += 1;
    }

    pub fn extend(&mut self, values: &[T]) {
        self.data[self.len..self.len + values.len()].copy_from_slice(values);
        self.len += values.len();
    }

    pub fn push_uninitialized(&mut self) -> &mut T {
        let index = self.len;
        self.len += 1;
        &mut self.data[index]
    }

    pub fn flush(&mut self) {
        self.len = 0;
    }

    pub fn inner(&mut self) -> &mut [T; N] {
        &mut self.data
    }
}

impl<'a, T: Copy + Default, const N: usize> IntoIterator for &'a FunVec<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[..self.len].iter()
    }
}

impl<T: Copy + Default, const N: usize> FunVec<T, N> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data[..self.len].iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data[..self.len].iter_mut()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunBox<T>(Box<T>);

impl<T> FunBox<T> {
    pub fn new(content: T) -> Self {
        Self(Box::new(content))
    }
}

impl<T> Deref for FunBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default> Default for FunBox<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for FunBox<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let content = Box::deserialize(deserializer)?;
        Ok(Self(content))
    }
}

impl<T: Serialize> Serialize for FunBox<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
