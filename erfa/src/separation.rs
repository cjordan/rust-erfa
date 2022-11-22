// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Code to calculate angular separations.

/// Angular separation between two sets of spherical coordinates. (`eraSeps`)
///
/// Given:
/// * `a_long`: first longitude (radians)
/// * `a_lat`: first latitude (radians)
/// * `b_long`: second longitude (radians)
/// * `b_lat`: second latitude (radians)
///
/// Returned:
/// * angular separation (radians)
///
pub fn sep_spherical_coords(a_long: f64, a_lat: f64, b_long: f64, b_lat: f64) -> f64 {
    use crate::transform::spherical_to_cartesian;

    let ac = spherical_to_cartesian(a_long, a_lat);
    let bc = spherical_to_cartesian(b_long, b_lat);

    /* Angle between the vectors. */
    sep_vectors(ac, bc)
}

/// Angular separation between two p-vectors. (`eraSepp`)
///
/// Given:
/// * `a`: first p-vector (not necessarily unit length)
/// * `b`: second p-vector (not necessarily unit length)
///
/// Returned:
/// * angular separation (radians, always positive)
///
/// Notes:
///
/// * The angular separation is most simply formulated in terms of
///   scalar product.  However, this gives poor accuracy for angles
///   near zero and pi.  The present algorithm uses both cross product
///   and dot product, to deliver full accuracy whatever the size of
///   the angle.
///
pub fn sep_vectors(a: [f64; 3], b: [f64; 3]) -> f64 {
    use crate::vectors_and_matrices;

    /* Sine of angle between the vectors, multiplied by the two moduli. */
    let axb = vectors_and_matrices::outer_product(a, b);
    let ss = vectors_and_matrices::modulus(axb);

    /* Cosine of the angle, multiplied by the two moduli. */
    let cs = vectors_and_matrices::inner_product(a, b);

    /* The angle. */
    if (ss != 0.0) || (cs != 0.0) {
        ss.atan2(cs)
    } else {
        0.0
    }
}
