// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Ellipsoid code.

/// Available ellipsoid models. If in doubt, use `WGS84`.
#[derive(Clone, Copy, Debug)]
pub enum Ellipsoid {
    /// [World Geodetic System 1984
    /// ensemble](https://en.wikipedia.org/wiki/World_Geodetic_System)
    WGS84 = 1,
    /// [Geodetic Reference System
    /// 1980](https://en.wikipedia.org/wiki/Geodetic_Reference_System_1980)
    GRS80 = 2,
    /// [World Geodetic System 1972
    /// ensemble](https://en.wikipedia.org/wiki/World_Geodetic_System)
    WGS72 = 3,
}

impl Default for Ellipsoid {
    fn default() -> Self {
        Self::WGS84
    }
}

impl Ellipsoid {
    /// Get the parameters of the supplied [`Ellipsoid`] in the form of
    /// equatorial radius in meters (`a`) and flattening (`f`).  The latter is a
    /// number around 0.00335, i.e. around 1/298.
    ///
    /// # References:
    ///
    /// * Department of Defense World Geodetic System 1984, National Imagery and
    ///   Mapping Agency Technical Report 8350.2, Third Edition, p3-2.
    ///
    /// * Moritz, H., Bull. Geodesique 66-2, 187 (1992).
    ///
    /// * The Department of Defense World Geodetic System 1972, World Geodetic
    ///   System Committee, May 1974.
    ///
    /// * Explanatory Supplement to the Astronomical Almanac, P. Kenneth
    ///   Seidelmann (ed), University Science Books (1992), p220.
    ///
    pub fn get_params(self) -> (f64, f64) {
        match self {
            Ellipsoid::WGS84 => (6378137.0, 1.0 / 298.257223563),
            Ellipsoid::GRS80 => (6378137.0, 1.0 / 298.257222101),
            Ellipsoid::WGS72 => (6378135.0, 1.0 / 298.26),
        }
    }
}
