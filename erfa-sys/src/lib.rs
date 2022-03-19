// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Provide Rust bindings to the ERFA C library.

#![allow(non_snake_case)]
#![allow(clippy::approx_constant)]
#![allow(clippy::excessive_precision)]
// deref_nullptr hides clippy warnings from erfa.rs, which was produced by bindgen.
#![allow(deref_nullptr)]

include!("erfa.rs");

#[cfg(test)]
mod tests {
    use super::*;

    /// eraEform takes a C int for its ellipsoid type. This test helps to ensure
    /// that when the ellipsoid ints are read with bindgen that they are the
    /// right integer type.
    #[test]
    fn test_eraEform_works() {
        unsafe {
            let mut a = 0.0;
            let mut f = 0.0;
            let j = eraEform(ERFA_WGS84, &mut a, &mut f);
            assert_eq!(j, 0);
            let j = eraEform(ERFA_GRS80, &mut a, &mut f);
            assert_eq!(j, 0);
            let j = eraEform(ERFA_WGS72, &mut a, &mut f);
            assert_eq!(j, 0);
        }
    }
}
