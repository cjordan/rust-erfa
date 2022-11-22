// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Time code.

mod s06;
pub use s06::S06;

use crate::constants::*;

/// Julian Date to Julian Epoch. (`eraEpj`)
///
/// Given:
/// * `dj1`,`dj2`: Julian Date (see note)
///
/// Returned (function value):
/// * Julian Epoch
///
/// # Note:
///
/// * The Julian Date is supplied in two pieces, in the usual ERFA manner, which
///   is designed to preserve time resolution.  The Julian Date is available as
///   a single number by adding `dj1` and `dj2`.  The maximum resolution is
///   achieved if `dj1` is 2451545.0 (J2000.0).
///
/// # Reference:
///
/// * Lieske, J.H., 1979, Astron.Astrophys. 73, 282.
///
pub fn julian_date_to_epoch(dj1: f64, dj2: f64) -> f64 {
    2000.0 + ((dj1 - ERFA_DJ00) + dj2) / ERFA_DJY
}

/// Julian Epoch to Julian Date. (`eraEpj2jd`)
///
/// Given:
/// * `epj`: Julian Epoch (e.g. 1996.8)
///
/// Returned:
/// * `djm0`: MJD zero-point: always 2400000.5
/// * `djm`: Modified Julian Date
///
/// # Note:
///
/// * The Julian Date is returned in two pieces, in the usual ERFA manner, which
///   is designed to preserve time resolution.  The Julian Date is available as
///   a single number by adding `djm0` and `djm`.
///
/// # Reference:
///
/// * Lieske, J.H., 1979, Astron.Astrophys. 73, 282.
///
pub fn julian_epoch_to_date(epj: f64) -> (f64, f64) {
    (ERFA_DJM0, ERFA_DJM00 + (epj - 2000.0) * 365.25)
}

/// Greenwich apparent sidereal time (consistent with IAU 2000 and 2006
/// resolutions). (`eraGst06a`)
///
/// Given:
/// * `uta`,`utb`: UT1 as a 2-part Julian Date (Notes 1,2)
/// * `tta`,`ttb`: TT as a 2-part Julian Date (Notes 1,2)
///
/// Returned:
/// * Greenwich apparent sidereal time (radians)
///
/// # Notes:
///
/// 1) The UT1 and TT dates `uta+utb` and `tta+ttb` respectively, are both
///    Julian Dates, apportioned in any convenient way between the argument
///    pairs.  For example, `JD(UT1)=2450123.7` could be expressed in any of
///    these ways, among others:
///
///    | `uta`     | `utb`   |                    |
///    |-----------|---------|--------------------|
///    | 2450123.7 |     0.0 | JD method          |
///    | 2451545.0 | -1421.3 | J2000 method       |
///    | 2400000.5 | 50123.2 | MJD method         |
///    | 2450123.5 |     0.2 | date & time method |
///
///    The JD method is the most natural and convenient to use in cases where
///    the loss of several decimal digits of resolution is acceptable (in the
///    case of UT;  the TT is not at all critical in this respect).  The J2000
///    and MJD methods are good compromises between resolution and convenience.
///    For UT, the date & time method is best matched to the algorithm that is
///    used by the Earth rotation angle function, called internally:  maximum
///    precision is delivered when the `uta` argument is for 0hrs UT1 on the day
///    in question and the utb argument lies in the range 0 to 1, or vice versa.
///
/// 2) Both UT1 and TT are required, UT1 to predict the Earth rotation and TT to
///    predict the effects of precession-nutation.  If UT1 is used for both
///    purposes, errors of order 100 microarcseconds result.
///
/// 3) This GAST is compatible with the IAU 2000/2006 resolutions and must be
///    used only in conjunction with IAU 2006 precession and IAU 2000A nutation.
///
/// 4) The result is returned in the range 0 to 2pi.
///
/// # Reference:
///
/// * Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
///
pub fn gst06a(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    /* Classical nutation x precession x bias matrix, IAU 2000A. */
    let rnpb = crate::prenut::pn_matrix_06a(tta, ttb);

    /* Greenwich apparent sidereal time. */
    gst06(uta, utb, tta, ttb, rnpb)
}

/// Greenwich apparent sidereal time, IAU 2006, given the NPB matrix.
/// (`eraGst06`)
///
/// Given:
/// * `uta`,`utb`: UT1 as a 2-part Julian Date (Notes 1,2)
/// * `tta`,`ttb`: TT as a 2-part Julian Date (Notes 1,2)
/// * `rnpb`: nutation x precession x bias matrix
///
/// Returned:
/// * Greenwich apparent sidereal time (radians)
///
/// # Notes:
///
/// 1) The UT1 and TT dates `uta+utb` and `tta+ttb` respectively, are both
///    Julian Dates, apportioned in any convenient way between the argument
///    pairs.  For example, `JD(UT1)=2450123.7` could be expressed in any of
///    these ways, among others:
///
///    | `uta`     | `utb`   |                    |
///    |-----------|---------|--------------------|
///    | 2450123.7 |     0.0 | JD method          |
///    | 2451545.0 | -1421.3 | J2000 method       |
///    | 2400000.5 | 50123.2 | MJD method         |
///    | 2450123.5 |     0.2 | date & time method |
///
///    The JD method is the most natural and convenient to use in cases where
///    the loss of several decimal digits of resolution is acceptable (in the
///    case of UT;  the TT is not at all critical in this respect).  The J2000
///    and MJD methods are good compromises between resolution and convenience.
///    For UT, the date & time method is best matched to the algorithm that is
///    used by the Earth rotation angle function, called internally:  maximum
///    precision is delivered when the `uta` argument is for 0hrs UT1 on the day
///    in question and the utb argument lies in the range 0 to 1, or vice versa.
///
/// 2) Both UT1 and TT are required, UT1 to predict the Earth rotation and TT to
///    predict the effects of precession-nutation.  If UT1 is used for both
///    purposes, errors of order 100 microarcseconds result.
///
/// 3) Although the function uses the IAU 2006 series for s+XY/2, it is
///    otherwise independent of the precession-nutation model and can in
///    practice be used with any equinox-based NPB matrix.
///
/// 4) The result is returned in the range 0 to 2pi.
///
/// # Reference:
///
/// * Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
///
pub fn gst06(uta: f64, utb: f64, tta: f64, ttb: f64, rnpb: [[f64; 3]; 3]) -> f64 {
    let (x, y) = crate::prenut::bpn_to_xy(rnpb);
    let s = S06(tta, ttb, x, y);
    let era = crate::earth::earth_rotation_angle_00(uta, utb);
    let eors = crate::prenut::eors(rnpb, s);
    crate::misc::norm_angle(era - eors)
}

/// Greenwich mean sidereal time (consistent with IAU 2006 precession).
/// (`eraGmst06`)
///
/// Given:
/// * `uta`,`utb`: UT1 as a 2-part Julian Date (Notes 1,2)
/// * `tta`,`ttb`: TT as a 2-part Julian Date (Notes 1,2)
///
/// Returned:
/// * Greenwich mean sidereal time (radians)
///
/// # Notes:
///
/// 1) The UT1 and TT dates `uta+utb` and `tta+ttb` respectively, are both
///    Julian Dates, apportioned in any convenient way between the argument
///    pairs.  For example, `JD=2450123.7` could be expressed in any of these
///    ways, among others:
///
///    | Part A    | Part B  |                    |
///    |-----------|---------|--------------------|
///    | 2450123.7 |     0.0 | JD method          |
///    | 2451545.0 | -1421.3 | J2000 method       |
///    | 2400000.5 | 50123.2 | MJD method         |
///    | 2450123.5 |     0.2 | date & time method |
///
///    The JD method is the most natural and convenient to use in cases where
///    the loss of several decimal digits of resolution is acceptable (in the
///    case of UT;  the TT is not at all critical in this respect).  The J2000
///    and MJD methods are good compromises between resolution and convenience.
///    For UT, the date & time method is best matched to the algorithm that is
///    used by the Earth rotation angle function, called internally:  maximum
///    precision is delivered when the `uta` argument is for 0hrs UT1 on the day
///    in question and the utb argument lies in the range 0 to 1, or vice versa.
///
/// 2) Both UT1 and TT are required, UT1 to predict the Earth rotation and TT to
///    predict the effects of precession.  If UT1 is used for both purposes,
///    errors of order 100 microarcseconds result.
///
/// 3) This GMST is compatible with the IAU 2006 precession and must not be used
///    with other precession models.
///
/// 4) The result is returned in the range 0 to 2pi.
///
/// # Reference:
///
/// * Capitaine, N., Wallace, P.T. & Chapront, J., 2005, Astron.Astrophys. 432,
///   355
///
pub fn gmst06(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    /* TT Julian centuries since J2000.0. */
    let t = ((tta - ERFA_DJ00) + ttb) / ERFA_DJC;

    /* Greenwich mean sidereal time, IAU 2006. */
    #[rustfmt::skip]
    let gmst = crate::misc::norm_angle(crate::earth::earth_rotation_angle_00(uta, utb) +
                  (    0.014506     +
                  (  4612.156534    +
                  (     1.3915817   +
                  (    -0.00000044  +
                  (    -0.000029956 +
                  (    -0.0000000368 )
          * t) * t) * t) * t) * t) * ERFA_DAS2R);

    gmst
}
