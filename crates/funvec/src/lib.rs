use std::convert::identity;

use serde::{Deserialize, Deserializer, Serialize};

pub const FUNVEC_LAYERS: usize = 5;
pub const FUNVEC_OODS: usize = 256;
pub const FUNVEC_LEAVES: usize = 256;
pub const FUNVEC_AUTHENTICATIONS: usize = 256;
pub const FUNVEC_LAST_LAYER: usize = 256;
pub const FUNVEC_DECOMMITMENT_VALUES: usize = 256;
pub const FUNVEC_PAGES: usize = 1024;
pub const FUNVEC_SEGMENTS: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FunVec<T: Default, const N: usize> {
    vec: [Option<T>; N],
}

impl<T: Default, const N: usize> Default for FunVec<T, N> {
    fn default() -> Self {
        Self { vec: core::array::from_fn(|_| Default::default()) }
    }
}

impl<'de, T: Copy + Default + Deserialize<'de>, const N: usize> Deserialize<'de> for FunVec<T, N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let vec: Vec<T> = Vec::deserialize(deserializer)?;
        Ok(Self::from_vec(vec))
    }
}

impl<'de, T: Copy + Default + Serialize, const N: usize> Serialize for FunVec<T, N> {
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
        let mut fun_vec = [Default::default(); N];
        for (i, value) in vec.iter().enumerate() {
            fun_vec[i] = Some(*value);
        }
        Self { vec: fun_vec }
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.vec.iter().copied().filter_map(identity).collect()
    }

    pub fn to_vec_ref(&self) -> Vec<&T> {
        self.vec.iter().filter_map(|x| x.as_ref()).collect()
    }

    pub fn len(&self) -> usize {
        self.vec.iter().count()
    }
}

impl<T: Copy + Default, const N: usize> IntoIterator for FunVec<T, N> {
    type Item = T;
    type IntoIter =
        std::iter::FilterMap<std::array::IntoIter<Option<T>, N>, fn(Option<T>) -> Option<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter().filter_map(identity)
    }
}

impl<'a, T: Copy + Default, const N: usize> IntoIterator for &'a FunVec<T, N> {
    type Item = &'a T;
    type IntoIter =
        std::iter::FilterMap<std::slice::Iter<'a, Option<T>>, fn(&'a Option<T>) -> Option<&'a T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter().filter_map(Option::as_ref)
    }
}

impl<T: Copy + Default, const N: usize> FunVec<T, N> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter().filter_map(Option::as_ref)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.vec.iter_mut().filter_map(Option::as_mut)
    }
}
