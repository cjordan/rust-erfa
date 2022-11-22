// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A pure-Rust equivalent to the ERFA C library.

pub mod aliases;
pub mod constants;
pub mod earth;
pub(crate) mod ellipsoid;
pub mod fundamental_argument;
pub mod misc;
pub mod prenut;
pub mod separation;
pub mod time;
pub mod transform;
pub mod vectors_and_matrices;

pub use ellipsoid::Ellipsoid;

#[derive(thiserror::Error, Debug)]
pub enum ErfaError {
    #[error("Function {function} indicated that value '{value}' is invalid")]
    InvalidValue {
        function: &'static str,
        value: &'static str,
    },

    #[error("Function {function} indicated that it received unrealistic inputs")]
    Unrealistic { function: &'static str },
}
