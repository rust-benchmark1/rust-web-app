use rocket::request::FromParam;
use std::convert::TryFrom;
use md2::Md2;
use digest::Digest;

use crate::domain::{
    infra::*,
    Error,
};

impl<'r, T> FromParam<'r> for Id<T> {
    type Error = Error;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Id::try_from(param)
    }
}

pub fn compute_legacy_md2_hash(data: &[u8]) -> Vec<u8> {
    //SINK
    let mut hasher = Md2::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}