// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Code to transform coordinates.

use crate::{ellipsoid::Ellipsoid, ErfaError};

/// P-vector to spherical coordinates. (`eraC2s`)
///
/// Given:
/// * `p`: p-vector
///
/// Returned:
/// * `theta`: longitude angle (radians)
/// * `phi`: latitude angle (radians)
///
/// # Notes:
///
/// 1) The vector `p` can have any magnitude; only its direction is used.
///
/// 2) At either pole, zero theta is returned.
///
pub fn cartesian_to_spherical(p: [f64; 3]) -> (f64, f64) {
    let x = p[0];
    let y = p[1];
    let z = p[2];
    let d2 = x * x + y * y;

    let theta = if d2 == 0.0 { 0.0 } else { y.atan2(x) };
    let phi = if z == 0.0 { 0.0 } else { z.atan2(d2.sqrt()) };

    (theta, phi)
}

/// Convert spherical coordinates to Cartesian. (`eraS2c`)
///
/// Given:
/// * `theta`: longitude angle (radians)
/// * `phi`: latitude angle (radians)
///
/// Returned:
/// * direction cosines
///
pub fn spherical_to_cartesian(theta: f64, phi: f64) -> [f64; 3] {
    let (sp, cp) = phi.sin_cos();
    let (st, ct) = theta.sin_cos();
    [ct * cp, st * cp, sp]
}

/// Horizon to equatorial coordinates:  transform azimuth and altitude to hour
/// angle and declination. (`eraAe2hd`)
///
/// Given:
/// * `az`: azimuth
/// * `el`: altitude (informally, elevation)
/// * `phi`: site latitude
///
/// Returned:
/// * `ha`: hour angle (local)
/// * `dec`: declination
///
/// # Notes:
///
/// 1) All the arguments are angles in radians.
///
/// 2) The sign convention for azimuth is north zero, east +pi/2.
///
/// 3) `ha` is returned in the range +/-pi. `dec` is returned in the range
///    +/-pi/2.
///
/// 4) The latitude `phi` is pi/2 minus the angle between the Earth's rotation
///    axis and the adopted zenith.  In many applications it will be sufficient
///    to use the published geodetic latitude of the site.  In very precise
///    (sub-arcsecond) applications, phi can be corrected for polar motion.
///
/// 5) The azimuth `az` must be with respect to the rotational north pole, as
///    opposed to the ITRS pole, and an azimuth with respect to north on a map
///    of the Earth's surface will need to be adjusted for polar motion if
///    sub-arcsecond accuracy is required.
///
/// 6) Should the user wish to work with respect to the astronomical zenith
///    rather than the geodetic zenith, `phi` will need to be adjusted for
///    deflection of the vertical (often tens of arcseconds), and the zero point
///    of `ha` will also be affected.
///
/// 7) The transformation is the same as `Ve = Ry(phi-pi/2)*Rz(pi)*Vh`, where
///    `Ve` and `Vh` are lefthanded unit vectors in the (`ha`,`dec`) and
///    (`az`,`el`) systems respectively and `Rz` and `Ry` are rotations about
///    first the z-axis and then the y-axis.  (n.b. `Rz(pi)` simply reverses the
///    signs of the x and y components.)  For efficiency, the algorithm is
///    written out rather than calling other utility functions.  For
///    applications that require even greater efficiency, additional savings are
///    possible if constant terms such as functions of latitude are computed
///    once and for all.
///
/// 8) Again for efficiency, no range checking of arguments is carried out.
///
pub fn azel_to_hadec(az: f64, el: f64, phi: f64) -> (f64, f64) {
    /* Useful trig functions. */
    let (sa, ca) = az.sin_cos();
    let (se, ce) = el.sin_cos();
    let (sp, cp) = phi.sin_cos();

    /* HA,Dec unit vector. */
    let x = -ca * ce * sp + se * cp;
    let y = -sa * ce;
    let z = ca * ce * cp + se * sp;

    /* To spherical. */
    let r = (x * x + y * y).sqrt();
    let ha = if r != 0.0 { y.atan2(x) } else { 0.0 };
    let dec = z.atan2(r);

    (ha, dec)
}

/// Equatorial to horizon coordinates: transform hour angle and declination to
/// azimuth and altitude. (`eraHd2ae`)
///
/// Given:
/// * `ha`: hour angle (local)
/// * `dec`: declination
/// * `phi`: site latitude
///
/// Returned:
/// * `az`: azimuth
/// * `el`: altitude (informally, elevation)
///
/// # Notes:
///
/// 1) All the arguments are angles in radians.
///
/// 2) `az` is returned in the range 0-2pi;  north is zero, and east is +pi/2.
///    `el` is returned in the range +/- pi/2.
///
/// 3) The latitude `phi` is pi/2 minus the angle between the Earth's rotation
///    axis and the adopted zenith.  In many applications it will be sufficient
///    to use the published geodetic latitude of the site.  In very precise
///    (sub-arcsecond) applications, `phi` can be corrected for polar motion.
///
/// 4) The returned azimuth `az` is with respect to the rotational north pole,
///    as opposed to the ITRS pole, and for sub-arcsecond accuracy will need to
///    be adjusted for polar motion if it is to be with respect to north on a
///    map of the Earth's surface.
///
/// 5) Should the user wish to work with respect to the astronomical zenith
///    rather than the geodetic zenith, `phi` will need to be adjusted for
///    deflection of the vertical (often tens of arcseconds), and the zero point
///    of the hour angle `ha` will also be affected.
///
/// 6) The transformation is the same as `Vh = Rz(pi)*Ry(pi/2-phi)*Ve`, where
///    `Vh` and `Ve` are lefthanded unit vectors in the (`az`,`el`) and
///    (`ha`,`dec`) systems respectively and `Ry` and `Rz` are rotations about
///    first the y-axis and then the z-axis.  (n.b. `Rz(pi)` simply reverses the
///    signs of the x and y components.)  For efficiency, the algorithm is
///    written out rather than calling other utility functions.  For
///    applications that require even greater efficiency, additional savings are
///    possible if constant terms such as functions of latitude are computed
///    once and for all.
///
/// 7) Again for efficiency, no range checking of arguments is carried out.
///
pub fn hadec_to_azel(ha: f64, dec: f64, phi: f64) -> (f64, f64) {
    /* Useful trig functions. */
    let (sh, ch) = ha.sin_cos();
    let (sd, cd) = dec.sin_cos();
    let (sp, cp) = phi.sin_cos();

    /* Az,Alt unit vector. */
    let x = -ch * cd * sp + sd * cp;
    let y = -sh * cd;
    let z = ch * cd * cp + sd * sp;

    /* To spherical. */
    let r = (x * x + y * y).sqrt();
    let a = if r != 0.0 { y.atan2(x) } else { 0.0 };
    let az = if a < 0.0 {
        a + crate::constants::ERFA_D2PI
    } else {
        a
    };
    let el = z.atan2(r);

    (az, el)
}

/// Parallactic angle for a given hour angle and declination. (`eraHd2pa`)
///
/// Given:
/// * `ha` hour angle
/// * `dec` declination
/// * `phi` site latitude
///
/// Returned:
/// * parallactic angle
///
/// # Notes:
///
/// 1)  All the arguments are angles in radians.
///
/// 2) The parallactic angle at a point in the sky is the position angle of the
///    vertical, i.e. the angle between the directions to the north celestial
///    pole and to the zenith respectively.
///
/// 3) The result is returned in the range -pi to +pi.
///
/// 4) At the pole itself a zero result is returned.
///
/// 5) The latitude phi is pi/2 minus the angle between the Earth's rotation
///    axis and the adopted zenith.  In many applications it will be sufficient
///    to use the published geodetic latitude of the site.  In very precise
///    (sub-arcsecond) applications, phi can be corrected for polar motion.
///
/// 6) Should the user wish to work with respect to the astronomical zenith
///    rather than the geodetic zenith, phi will need to be adjusted for
///    deflection of the vertical (often tens of arcseconds), and the zero point
///    of the hour angle ha will also be affected.
///
/// # Reference:
///
/// Smart, W.M., "Spherical Astronomy", Cambridge University Press, 6th edition
/// (Green, 1977), p49.
///
pub fn hadec_to_parallactic_angle(ha: f64, dec: f64, phi: f64) -> f64 {
    let (sp, cp) = phi.sin_cos();
    let (sha, cha) = ha.sin_cos();
    let (sdec, cdec) = dec.sin_cos();
    let sqsz = cp * sha;
    let cqsz = sp * cdec - cp * sdec * cha;
    if sqsz != 0.0 || cqsz != 0.0 {
        sqsz.atan2(cqsz)
    } else {
        0.0
    }
}

/// Transform geocentric coordinates to geodetic using the specified reference
/// ellipsoid. (`eraGc2gd`)
///
/// Given:
/// * `e`: ellipsoid identifier
/// * `xyz`: geocentric vector (Note 1)
///
/// Returned:
/// * `elong`: longitude (radians, east +ve)
/// * `phi`: latitude (geodetic, radians)
/// * `height`: height above ellipsoid (geodetic, metres, Note 1)
///
/// # Notes:
///
/// 1) The geocentric vector (`xyz`, given) and height (`height`, returned) are
///    in meters.
///
pub fn geocentric_to_geodetic(e: Ellipsoid, xyz: [f64; 3]) -> [f64; 3] {
    let (a, f) = e.get_params();
    geocentric_to_geodetic_inner(a, f, xyz)
        .expect("There are issues with the reference ellipsoid values")
}

/// Transform geocentric coordinates to geodetic for a reference ellipsoid of
/// specified form. (`eraGc2gde`)
///
/// Given:
/// * `a`: equatorial radius (Notes 2,4)
/// * `f`: flattening (Note 3)
/// * `xyz`: geocentric vector (Note 4)
///
/// Returned:
/// * `elong`: longitude (radians, east +ve)
/// * `phi`: latitude (geodetic, radians)
/// * `height`: height above ellipsoid (geodetic, Note 4)
///
/// # Errors
///
/// This function will return an error if input arguments `a` or `f` are
/// invalid; valid values are `a` <= 0 and 0 <= `f` < 1.
///
/// # Notes:
///
/// 1) This function is based on the `GCONV2H` Fortran subroutine by Toshio
///    Fukushima (see reference).
///
/// 2) The equatorial radius, `a`, can be in any units, but meters is the
///    conventional choice.
///
/// 3) The flattening, `f`, is (for the Earth) a value around 0.00335, i.e.
///    around 1/298.
///
/// 4) The equatorial radius, `a`, and the geocentric vector, `xyz`, must be
///    given in the same units, and determine the units of the returned height,
///    height.
///
/// # Reference:
///
/// * Fukushima, T., "Transformation from Cartesian to geodetic coordinates
///   accelerated by Halley's method", J.Geodesy (2006) 79: 689-693
///
pub fn geocentric_to_geodetic_inner(a: f64, f: f64, xyz: [f64; 3]) -> Result<[f64; 3], ErfaError> {
    if !(0.0..1.0).contains(&f) {
        return Err(ErfaError::InvalidValue {
            function: "geocentric_to_geodetic_inner",
            value: "f",
        });
    }
    if a <= 0.0 {
        return Err(ErfaError::InvalidValue {
            function: "geocentric_to_geodetic_inner",
            value: "a",
        });
    }

    /* Functions of ellipsoid parameters. */
    let aeps2 = a * a * 1e-32;
    let e2 = (2.0 - f) * f;
    let e4t = e2 * e2 * 1.5;
    let ec2 = 1.0 - e2;
    // The following line of C code is here for posterity. It is only triggered
    // if `f` >= 1, but we already check that above.
    // if ( ec2 <= 0.0 ) return -1;
    let ec = ec2.sqrt();
    let b = a * ec;

    /* Cartesian components. */
    let x = xyz[0];
    let y = xyz[1];
    let z = xyz[2];

    /* Distance from polar axis squared. */
    let p2 = x * x + y * y;

    /* Longitude. */
    let elong = if p2 > 0.0 { y.atan2(x) } else { 0.0 };

    /* Unsigned z-coordinate. */
    let absz = z.abs();

    /* Proceed unless polar case. */
    let (mut phi, height) = if p2 > aeps2 {
        /* Distance from polar axis. */
        let p = p2.sqrt();

        /* Normalization. */
        let s0 = absz / a;
        let pn = p / a;
        let zc = ec * s0;

        /* Prepare Newton correction factors. */
        let c0 = ec * pn;
        let c02 = c0 * c0;
        let c03 = c02 * c0;
        let s02 = s0 * s0;
        let s03 = s02 * s0;
        let a02 = c02 + s02;
        let a0 = a02.sqrt();
        let a03 = a02 * a0;
        let d0 = zc * a03 + e2 * s03;
        let f0 = pn * a03 - e2 * c03;

        /* Prepare Halley correction factor. */
        let b0 = e4t * s02 * c02 * pn * (a0 - ec);
        let s1 = d0 * f0 - b0 * s0;
        let cc = ec * (f0 * f0 - b0 * c0);

        /* Evaluate latitude and height. */
        let phi = (s1 / cc).atan();
        let s12 = s1 * s1;
        let cc2 = cc * cc;
        let height = (p * cc + absz * s1 - a * (ec2 * s12 + cc2).sqrt()) / (s12 + cc2).sqrt();

        (phi, height)
    } else {
        /* Exception: pole. */
        (crate::constants::ERFA_DPI / 2.0, absz - b)
    };

    /* Restore sign of latitude. */
    if z < 0.0 {
        phi *= -1.0;
    }

    Ok([elong, phi, height])
}

/// Transform geodetic coordinates to geocentric using the specified reference
/// ellipsoid. (`eraGd2gc`)
///
/// Given:
/// * `e`: ellipsoid identifier
/// * `elong`: longitude (radians, east +ve)
/// * `phi`: latitude (geodetic, radians, Note 2)
/// * `height`: height above ellipsoid (geodetic, metres, Notes 1,2)
///
/// Returned:
/// * `xyz`: geocentric vector (Note 1)
///
/// # Errors
///
/// This function will return an error if input arguments are unrealistic (Note
/// 2).
///
/// # Notes:
///
/// 1) The height (`height`, given) and the geocentric vector (`xyz`, returned)
///    are in meters.
///
/// 2) No validation is performed on the arguments `elong`, `phi` and `height`.
///    An error indicates a case that would lead to arithmetic exceptions.
///
pub fn geodetic_to_geocentric(
    e: Ellipsoid,
    elong: f64,
    phi: f64,
    height: f64,
) -> Result<[f64; 3], ErfaError> {
    let (a, f) = e.get_params();
    geodetic_to_geocentric_inner(a, f, elong, phi, height)
}

/// Transform geodetic coordinates to geocentric for a reference ellipsoid of
/// specified form. (`eraGd2gce`)
///
/// Given:
/// * `a`: equatorial radius (Notes 1,4)
/// * `f`: flattening (Notes 2,4)
/// * `elong`: longitude (radians, east +ve)
/// * `phi`: latitude (geodetic, radians, Note 4)
/// * `height`: height above ellipsoid (geodetic, Notes 3,4)
///
/// Returned:
/// * `xyz`: geocentric vector (Note 3)
///
/// # Errors
///
/// This function will return an error if input arguments are unrealistic (Note
/// 4).
///
/// # Notes:
///
/// 1) The equatorial radius, `a`, can be in any units, but meters is the
///    conventional choice.
///
/// 2) The flattening, `f`, is (for the Earth) a value around 0.00335, i.e.
///    around 1/298.
///
/// 3) The equatorial radius, `a`, and the height, `height`, must be given in
///    the same units, and determine the units of the returned geocentric
///    vector, xyz.
///
/// 4) No validation is performed on individual arguments. An error indicates
///    unrealistic cases that would lead to arithmetic exceptions.
///
/// # References:
///
/// * Green, R.M., Spherical Astronomy, Cambridge University Press, (1985)
///   Section 4.5, p96.
///
/// * Explanatory Supplement to the Astronomical Almanac, P. Kenneth Seidelmann
///   (ed), University Science Books (1992), Section 4.22, p202.
///
pub fn geodetic_to_geocentric_inner(
    a: f64,
    f: f64,
    elong: f64,
    phi: f64,
    height: f64,
) -> Result<[f64; 3], ErfaError> {
    /* Functions of geodetic latitude. */
    let (sp, cp) = phi.sin_cos();
    let w = 1.0 - f;
    let w = w * w;
    let d = cp * cp + w * sp * sp;
    if d <= 0.0 {
        return Err(ErfaError::Unrealistic {
            function: "geodetic_to_geocentric_inner",
        });
    }
    let ac = a / d.sqrt();
    let as_ = w * ac;

    /* Geocentric vector. */
    let r = (ac + height) * cp;
    let (s_elong, c_elong) = elong.sin_cos();
    Ok([r * c_elong, r * s_elong, (as_ + height) * sp])
}
