#![allow(missing_docs)]
//! These are inner traits to support methods overloading for the `Client`

use crate::error::Error;
use std::collections::BTreeMap;

#[doc(hidden)]
pub trait FromMap: Sized {
    fn from_map(map: BTreeMap<String, String>) -> Result<Self, Error>;
}

#[doc(hidden)]
pub trait FromIter: Sized {
    fn from_iter<I: Iterator<Item = Result<(String, String), Error>>>(iter: I) -> Result<Self, Error>;
}

impl<T: FromMap> FromIter for T {
    fn from_iter<I: Iterator<Item = Result<(String, String), Error>>>(iter: I) -> Result<Self, Error> {
        iter.collect::<Result<BTreeMap<_, _>, _>>().and_then(FromMap::from_map)
    }
}
