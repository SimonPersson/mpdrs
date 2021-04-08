//! The module describes output

use crate::convert::FromMap;
use crate::error::{Error, ProtoError};
use std::collections::BTreeMap;

/// Sound output
#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    /// id
    pub id: u32,
    /// name
    pub name: String,
    /// enabled state
    pub enabled: bool,
}

impl FromMap for Output {
    fn from_map(map: BTreeMap<String, String>) -> Result<Output, Error> {
        Ok(Output {
            id: map.get("outputid").ok_or(Error::Proto(ProtoError::NoField("outputid")))?.parse()?,
            name: map
                .get("outputname")
                .map(|v| v.to_owned())
                .ok_or(Error::Proto(ProtoError::NoField("outputname")))?,
            enabled: map
                .get("outputenabled")
                .ok_or(Error::Proto(ProtoError::NoField("outputenabled")))?
                .parse::<i32>()?
                == 1,
        })
    }
}
