// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(unused_imports)]

use packable::{
    error::UnpackErrorExt,
    error::{UnknownTagError, UnpackError},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use core::convert::Infallible;

#[derive(Debug)]
pub struct PickyError(u8);

impl From<Infallible> for PickyError {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

fn verify_value<const VERIFY: bool>(&value: &u8, _: &()) -> Result<(), PickyError> {
    if !VERIFY || value == 42 {
        Ok(())
    } else {
        Err(PickyError(value))
    }
}

#[derive(Packable)]
#[packable(unpack_error = PickyError)]
pub struct Picky(#[packable(verify_with = verify_value)] u8);

fn main() {}
