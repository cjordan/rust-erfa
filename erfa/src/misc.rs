// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::constants::ERFA_D2PI;

/// Normalize angle into the range `0 <= a < 2pi`. (`eraAnp`)
///
/// Given:
/// * `a`: angle (radians)
///
/// Returned:
/// * angle in range `0-2pi`
///
pub fn norm_angle(a: f64) -> f64 {
    let mut w = a % ERFA_D2PI;
    if w < 0.0 {
        w += ERFA_D2PI;
    }

    w
}
