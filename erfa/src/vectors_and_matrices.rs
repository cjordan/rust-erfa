// This Source Code Form is subject to the terms of the Moz[i]lla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Code for vectors ("p-vectors" and "pv-vectors") and "r-matrices".

/// Multiply a p-vector by a scalar. (`eraSxp`)
///
/// Given:
/// * `s`: scalar
/// * `p`: p-vector
///
/// Returned:
/// * `s` * `p`
///
pub fn multiply(s: f64, p: [f64; 3]) -> [f64; 3] {
    [s * p[0], s * p[1], s * p[2]]
}

/// Modulus of p-vector. (`eraPm`)
///
/// Given:
/// * `p`: p-vector
///
/// Returned:
/// * modulus
///
pub fn modulus(p: [f64; 3]) -> f64 {
    (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()
}

/// Convert a p-vector into modulus and unit vector. (`eraPn`)
///
/// Given:
/// * `p`: p-vector
///
/// Returned:
/// * `r`: modulus
/// * `u`: unit vector
pub fn modulus_and_unit_vector(p: [f64; 3]) -> (f64, [f64; 3]) {
    /* Obtain the modulus and test for zero. */
    let w = modulus(p);
    if w == 0.0 {
        /* Null vector. */
        (0.0, [0.0; 3])
    } else {
        /* Unit vector. */
        let u = multiply(1.0 / w, p);
        (w, u)
    }
}

/// p-vector outer (=vector=cross) product. (`eraPvxpv`)
///
/// Given:
/// * `a`: first p-vector
/// * `b`: second p-vector
///
/// Returned:
/// * `a x b`
///
pub fn outer_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// p-vector inner (=scalar=dot) product. (`eraPdp`)
///
/// Given:
/// * `a`: first p-vector
/// * `b`: second p-vector
///
/// Returned:
/// * `a . b`
///
pub fn inner_product(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Multiply a p-vector by an r-matrix. (`eraRxp`)
///
/// Given:
/// * `r`: r-matrix
/// * `p`: p-vector
///
/// Returned:
/// * `rp`: `r * p`
///
pub fn mat_mul_pvec(r: [[f64; 3]; 3], p: [f64; 3]) -> [f64; 3] {
    let mut wrp = [0.0; 3];
    /* Matrix r * vector p. */
    for (r, wrp) in r.iter().zip(wrp.iter_mut()) {
        let mut w = 0.0;
        for (r, p) in r.iter().zip(p) {
            w += r * p;
        }
        *wrp = w;
    }

    wrp
}

/// Multiply a pv-vector by an r-matrix. (`eraRxpv`)
///
/// Given:
/// * `r`: r-matrix
/// * `pv`: pv-vector
///
/// Returned:
/// * `rpv`: `r * pv`
///
/// # Note:
///
/// 1) The algorithm is for the simple case where the r-matrix `r` is not a
///    function of time.  The case where `r` is a function of time leads to an
///    additional velocity component equal to the product of the derivative of
///    `r` and the position vector.
///
pub fn mat_mul_pvvec(r: [[f64; 3]; 3], pv: [[f64; 3]; 2]) -> [[f64; 3]; 2] {
    let rp1 = mat_mul_pvec(r, pv[0]);
    let rp2 = mat_mul_pvec(r, pv[1]);
    [rp1, rp2]
}

/// Multiply two r-matrices. (`eraRxr`)
///
/// Given:
/// * `a`: first r-matrix
/// * `b`: second r-matrix
///
/// Returned:
/// * `a * b`
///
pub fn multiply_matrices(a: [[f64; 3]; 3], b: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut wm = [[0.0; 3]; 3];

    for (wm, a) in wm.iter_mut().zip(a.iter()) {
        for j in 0..3 {
            let mut w = 0.0;
            for (k, b) in b.iter().enumerate() {
                w += a[k] * b[j];
            }
            wm[j] = w;
        }
    }

    wm
}

/// Initialize an r-matrix to the identity matrix. (`eraIr`)
///
/// Modified:
///  * `r`: r-matrix
///
pub fn init_matrix(r: &mut [[f64; 3]; 3]) {
    r[0][0] = 1.0;
    r[0][1] = 0.0;
    r[0][2] = 0.0;
    r[1][0] = 0.0;
    r[1][1] = 1.0;
    r[1][2] = 0.0;
    r[2][0] = 0.0;
    r[2][1] = 0.0;
    r[2][2] = 1.0;
}

/// Rotate an r-matrix about the x-axis. (`eraRx`)
///
/// Given:
///  * `phi`: angle (radians)
///
/// Given and returned:
///  * `r`: r-matrix, rotated
///
/// # Notes:
///
/// 1) Calling this function with positive `phi` incorporates in the supplied
///    r-matrix `r` an additional rotation, about the x-axis, anticlockwise as
///    seen looking towards the origin from positive x.
///
/// 2) The additional rotation can be represented by this matrix:
///
///    |---|-----------|-----------|
///    | 1 |     0     |     0     |
///    | 0 | +cos(phi) | +sin(phi) |
///    | 0 | -sin(phi) | +cos(phi) |
///
pub fn rotate_x(phi: f64, r: &mut [[f64; 3]; 3]) {
    let (s, c) = phi.sin_cos();

    let a10 = c * r[1][0] + s * r[2][0];
    let a11 = c * r[1][1] + s * r[2][1];
    let a12 = c * r[1][2] + s * r[2][2];
    let a20 = -s * r[1][0] + c * r[2][0];
    let a21 = -s * r[1][1] + c * r[2][1];
    let a22 = -s * r[1][2] + c * r[2][2];

    r[1][0] = a10;
    r[1][1] = a11;
    r[1][2] = a12;
    r[2][0] = a20;
    r[2][1] = a21;
    r[2][2] = a22;
}

/// Rotate an r-matrix about the z-axis. (`eraRz`)
///
/// Given:
///  * `psi`: angle (radians)
///
/// Modified:
///  * `r`: r-matrix, rotated
///
/// # Notes:
///
/// 1) Calling this function with positive `psi` incorporates in the supplied
///    r-matrix `r` an additional rotation, about the z-axis, anticlockwise as
///    seen looking towards the origin from positive z.
///
/// 2) The additional rotation can be represented by this matrix:
///
///    | +cos(psi) | +sin(psi) | 0 |
///    |           |           |   |
///    | -sin(psi) | +cos(psi) | 0 |
///    |           |           |   |
///    |     0     |     0     | 1 |
///
pub fn rotate_z(psi: f64, r: &mut [[f64; 3]; 3]) {
    let (s, c) = psi.sin_cos();

    let a00 = c * r[0][0] + s * r[1][0];
    let a01 = c * r[0][1] + s * r[1][1];
    let a02 = c * r[0][2] + s * r[1][2];
    let a10 = -s * r[0][0] + c * r[1][0];
    let a11 = -s * r[0][1] + c * r[1][1];
    let a12 = -s * r[0][2] + c * r[1][2];

    r[0][0] = a00;
    r[0][1] = a01;
    r[0][2] = a02;
    r[1][0] = a10;
    r[1][1] = a11;
    r[1][2] = a12;
}

/// Copy a p-vector. (`eraCp`)
///
/// Given:
/// * `p`: p-vector to be copied
///
/// Returned:
/// * `c`: copy
///
pub fn copy_vector(p: [f64; 3]) -> [f64; 3] {
    p
}

/// Copy an r-matrix. (`eraCr`)
///
/// Given:
/// * `r`: r-matrix to be copied
///
/// Returned:
/// * `c`: copy
///
pub fn copy_matrix(r: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    r
}
