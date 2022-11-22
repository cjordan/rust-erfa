// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod position_velocity_00;
pub use position_velocity_00::position_velocity_00;

use crate::{
    constants::{ERFA_D2PI, ERFA_DJ00},
    misc::norm_angle,
};

/// Earth rotation angle (IAU 2000 model). (`eraEra00`)
///
/// Given:
///  * `dj1`,`dj2`: UT1 as a 2-part Julian Date (see note)
///
/// Returned:
///  * Earth rotation angle (radians), range 0-2pi
///
/// # Notes:
///
/// 1) The UT1 date `dj1`+`dj2` is a Julian Date, apportioned in any convenient
///    way between the arguments `dj1` and `dj2`.  For example,
///    `JD(UT1)=2450123.7` could be expressed in any of these ways, among
///    others:
///
///    | `dj1`     | `dj2`   |                    |
///    |-----------|---------|--------------------|
///    | 2450123.7 |     0.0 | JD method          |
///    | 2451545.0 | -1421.3 | J2000 method       |
///    | 2400000.5 | 50123.2 | MJD method         |
///    | 2450123.5 |     0.2 | date & time method |
///
///    The JD method is the most natural and convenient to use in cases where
///    the loss of several decimal digits of resolution is acceptable.  The
///    J2000 and MJD methods are good compromises between resolution and
///    convenience.  The date & time method is best matched to the algorithm
///    used:  maximum precision is delivered when the `dj1` argument is for 0hrs
///    UT1 on the day in question and the `dj2` argument lies in the range 0 to
///    1, or vice versa.
///
/// 2) The algorithm is adapted from Expression 22 of Capitaine et al.
///    2000.  The time argument has been expressed in days directly, and, to
///    retain precision, integer contributions have been eliminated.  The same
///    formulation is given in IERS Conventions (2003), Chap. 5, Eq. 14.
///
/// # References:
///
/// * Capitaine N., Guinot B. and McCarthy D.D, 2000, Astron. Astrophys., 355,
///   398-405.
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
pub fn earth_rotation_angle_00(dj1: f64, dj2: f64) -> f64 {
    /* Days since fundamental epoch. */
    let (d1, d2) = if dj1 < dj2 { (dj1, dj2) } else { (dj2, dj1) };
    let t = d1 + (d2 - ERFA_DJ00);

    /* Fractional part of T (days). */
    let f = d1 % 1.0 + d2 % 1.0;

    /* Earth rotation angle at this UT1. */
    norm_angle(ERFA_D2PI * (f + 0.7790572732640 + 0.00273781191135448 * t))
}
