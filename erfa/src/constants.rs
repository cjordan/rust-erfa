// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! ERFA constants.

#![allow(clippy::excessive_precision)]

/// Pi
pub const ERFA_DPI: f64 = 3.141592653589793238462643;

/// 2Pi
pub const ERFA_D2PI: f64 = 6.283185307179586476925287;

/// Radians to degrees
pub const ERFA_DR2D: f64 = 57.29577951308232087679815;

/// Degrees to radians
pub const ERFA_DD2R: f64 = 1.745329251994329576923691e-2;

/// Radians to arcseconds
pub const ERFA_DR2AS: f64 = 206264.8062470963551564734;

/// Arcseconds to radians
pub const ERFA_DAS2R: f64 = 4.848136811095359935899141e-6;

/// Seconds of time to radians
pub const ERFA_DS2R: f64 = 7.272205216643039903848712e-5;

/// Arcseconds in a full circle
pub const ERFA_TURNAS: f64 = 1296000.0;

/// Milliarcseconds to radians
pub const ERFA_DMAS2R: f64 = ERFA_DAS2R / 1e3;

/// Length of tropical year B1900 (days)
pub const ERFA_DTY: f64 = 365.242198781;

/// Seconds per day.
pub const ERFA_DAYSEC: f64 = 86400.0;

/// Days per Julian year
pub const ERFA_DJY: f64 = 365.25;

/// Days per Julian century
pub const ERFA_DJC: f64 = 36525.0;

/// Days per Julian millennium
pub const ERFA_DJM: f64 = 365250.0;

/// Reference epoch (J2000.0), Julian Date
pub const ERFA_DJ00: f64 = 2451545.0;

/// Julian Date of Modified Julian Date zero
pub const ERFA_DJM0: f64 = 2400000.5;

/// Reference epoch (J2000.0), Modified Julian Date
pub const ERFA_DJM00: f64 = 51544.5;

/// 1977 Jan 1.0 as MJD
pub const ERFA_DJM77: f64 = 43144.0;

/// TT minus TAI (s)
pub const ERFA_TTMTAI: f64 = 32.184;

/// Astronomical unit (m, IAU 2012)
pub const ERFA_DAU: f64 = 149597870.7e3;

/// Speed of light (m/s)
pub const ERFA_CMPS: f64 = 299792458.0;

/// Light time for 1 au (s)
pub const ERFA_AULT: f64 = ERFA_DAU / ERFA_CMPS;

/// Speed of light (au per day)
pub const ERFA_DC: f64 = ERFA_DAYSEC / ERFA_AULT;

/// L_G = 1 - d(TT)/d(TCG)
pub const ERFA_ELG: f64 = 6.969290134e-10;

/// L_B = 1 - d(TDB)/d(TCB), and TDB (s) at TAI 1977/1/1.0
pub const ERFA_ELB: f64 = 1.550519768e-8;
pub const ERFA_TDB0: f64 = -6.55e-5;

/// Schwarzschild radius of the Sun (au)
/// = 2 * 1.32712440041e20 / (2.99792458e8)^2 / 1.49597870700e11
pub const ERFA_SRS: f64 = 1.97412574336e-8;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constants_are_equal() {
        for (c, c2) in [
            (ERFA_DPI, erfa_sys::ERFA_DPI),
            (ERFA_D2PI, erfa_sys::ERFA_D2PI),
            (ERFA_DR2D, erfa_sys::ERFA_DR2D),
            (ERFA_DD2R, erfa_sys::ERFA_DD2R),
            (ERFA_DR2AS, erfa_sys::ERFA_DR2AS),
            (ERFA_DAS2R, erfa_sys::ERFA_DAS2R),
            (ERFA_DS2R, erfa_sys::ERFA_DS2R),
            (ERFA_TURNAS, erfa_sys::ERFA_TURNAS),
            (ERFA_DMAS2R, erfa_sys::ERFA_DMAS2R),
            (ERFA_DTY, erfa_sys::ERFA_DTY),
            (ERFA_DAYSEC, erfa_sys::ERFA_DAYSEC),
            (ERFA_DJY, erfa_sys::ERFA_DJY),
            (ERFA_DJC, erfa_sys::ERFA_DJC),
            (ERFA_DJM, erfa_sys::ERFA_DJM),
            (ERFA_DJ00, erfa_sys::ERFA_DJ00),
            (ERFA_DJM0, erfa_sys::ERFA_DJM0),
            (ERFA_DJM00, erfa_sys::ERFA_DJM00),
            (ERFA_DJM77, erfa_sys::ERFA_DJM77),
            (ERFA_TTMTAI, erfa_sys::ERFA_TTMTAI),
            (ERFA_DAU, erfa_sys::ERFA_DAU),
            (ERFA_CMPS, erfa_sys::ERFA_CMPS),
            (ERFA_AULT, erfa_sys::ERFA_AULT),
            (ERFA_DC, erfa_sys::ERFA_DC),
            (ERFA_ELG, erfa_sys::ERFA_ELG),
            (ERFA_ELB, erfa_sys::ERFA_ELB),
            (ERFA_TDB0, erfa_sys::ERFA_TDB0),
            (ERFA_SRS, erfa_sys::ERFA_SRS),
        ] {
            assert!(c == c2);
        }
    }
}
