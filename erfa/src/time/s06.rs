// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{constants::*, fundamental_argument::*};

/// The CIO locator s, positioning the Celestial Intermediate Origin on the
/// equator of the Celestial Intermediate Pole, given the CIP's X,Y coordinates.
/// Compatible with IAU 2006/2000A precession-nutation. (`eraS06`)
///
/// Given:
///  * `date1`,`date2`: TT as a 2-part Julian Date (Note 1)
///  * `x`,`y`: CIP coordinates (Note 3)
///
/// Returned:
///  * the CIO locator s in radians (Note 2)
///
/// # Notes:
///
/// 1) The TT date `date1+date2` is a Julian Date, apportioned in any convenient
///    way between the two arguments.  For example, `JD(TT)=2450123.7` could be
///    expressed in any of these ways, among others:
///
///    | `date1`   | `date2` |                    |
///    |-----------|---------|--------------------|
///    | 2450123.7 |     0.0 | JD method          |
///    | 2451545.0 | -1421.3 | J2000 method       |
///    | 2400000.5 | 50123.2 | MJD method         |
///    | 2450123.5 |     0.2 | date & time method |
///
///    The JD method is the most natural and convenient to use in cases where
///    the loss of several decimal digits of resolution is acceptable.  The
///    J2000 method is best matched to the way the argument is handled
///    internally and will deliver the optimum resolution.  The MJD method and
///    the date & time methods are both good compromises between resolution and
///    convenience.
///
/// 2) The CIO locator s is the difference between the right ascensions of the
///    same point in two systems:  the two systems are the GCRS and the CIP,CIO,
///    and the point is the ascending node of the CIP equator.  The quantity s
///    remains below 0.1 arcsecond throughout 1900-2100.
///
/// 3) The series used to compute s is in fact for s+XY/2, where X and Y are the
///    x and y components of the CIP unit vector;  this series is more compact
///    than a direct series for s would be.  This function requires X,Y to be
///    supplied by the caller, who is responsible for providing values that are
///    consistent with the supplied date.
///
/// 4) The model is consistent with the "P03" precession (Capitaine et al.
///    2003), adopted by IAU 2006 Resolution 1, 2006, and the IAU 2000A nutation
///    (with P03 adjustments).
///
/// # References:
///
/// * Capitaine, N., Wallace, P.T. & Chapront, J., 2003, Astron. Astrophys. 432,
///   355
///
/// * McCarthy, D.D., Petit, G. (eds.) 2004, IERS Conventions (2003), IERS
///   Technical Note No. 32, BKG
///
#[allow(non_snake_case)]
pub fn S06(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    /* Interval between fundamental epoch J2000.0 and current date (JC). */
    let t = (date1 - ERFA_DJ00 + date2) / ERFA_DJC;

    /* Fundamental Arguments (from IERS Conventions 2003) */
    let fa: [f64; 8] = [
        /* Mean anomaly of the Moon. */
        l03(t),
        /* Mean anomaly of the Sun. */
        lp03(t),
        /* Mean longitude of the Moon minus that of the ascending node. */
        f03(t),
        /* Mean elongation of the Moon from the Sun. */
        d03(t),
        /* Mean longitude of the ascending node of the Moon. */
        om03(t),
        /* Mean longitude of Venus. */
        ve03(t),
        /* Mean longitude of Earth. */
        e03(t),
        /* General precession in longitude. */
        pa03(t),
    ];

    /* Evaluate s. */
    let mut w0 = SP[0];
    let mut w1 = SP[1];
    let mut w2 = SP[2];
    let mut w3 = SP[3];
    let mut w4 = SP[4];
    let w5 = SP[5];

    for s0 in S0.iter().rev() {
        let a = s0
            .nfa
            .iter()
            .copied()
            .zip(fa.iter().copied())
            .fold(0.0, |acc, (nfa, fa)| acc + f64::from(nfa) * fa);
        w0 += s0.s * a.sin() + s0.c * a.cos();
    }
    for s1 in S1.iter().rev() {
        let a = s1
            .nfa
            .iter()
            .copied()
            .zip(fa.iter().copied())
            .fold(0.0, |acc, (nfa, fa)| acc + f64::from(nfa) * fa);
        w1 += s1.s * a.sin() + s1.c * a.cos();
    }
    for s2 in S2.iter().rev() {
        let a = s2
            .nfa
            .iter()
            .copied()
            .zip(fa.iter().copied())
            // .fold(0.0, |acc, (nfa, fa)| acc + f64::from(nfa) + fa);
            .fold(0.0, |acc, (nfa, fa)| acc + f64::from(nfa) * fa);
        w2 += s2.s * a.sin() + s2.c * a.cos();
    }
    for s3 in S3.iter().rev() {
        let a = s3
            .nfa
            .iter()
            .copied()
            .zip(fa.iter().copied())
            .fold(0.0, |acc, (nfa, fa)| acc + f64::from(nfa) * fa);
        w3 += s3.s * a.sin() + s3.c * a.cos();
    }
    for s4 in S4.iter().rev() {
        let a = s4
            .nfa
            .iter()
            .copied()
            .zip(fa.iter().copied())
            .fold(0.0, |acc, (nfa, fa)| acc + f64::from(nfa) * fa);
        w4 += s4.s * a.sin() + s4.c * a.cos();
    }

    (w0 + (w1 + (w2 + (w3 + (w4 + w5 * t) * t) * t) * t) * t) * ERFA_DAS2R - x * y / 2.0
}

/* Polynomial coefficients */
const SP: [f64; 6] = [
    94.00e-6,
    3808.65e-6,
    -122.68e-6,
    -72574.11e-6,
    27.98e-6,
    15.62e-6,
];

/* --------------------- */
/* The series for s+XY/2 */
/* --------------------- */
struct Term {
    /// coefficients of l,l',F,D,Om,LVe,LE,pA
    nfa: [i32; 8],
    /// sine coefficients
    s: f64,
    /// cosine coefficients
    c: f64,
}

/* Terms of order t^0 */
const S0: [Term; 33] = [
    Term {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: -2640.73e-6,
        c: 0.39e-6,
    },
    Term {
        nfa: [0, 0, 0, 0, 2, 0, 0, 0],
        s: -63.53e-6,
        c: 0.02e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 3, 0, 0, 0],
        s: -11.75e-6,
        c: -0.01e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 1, 0, 0, 0],
        s: -11.21e-6,
        c: -0.01e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 2, 0, 0, 0],
        s: 4.57e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, 0, 3, 0, 0, 0],
        s: -2.02e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, 0, 1, 0, 0, 0],
        s: -1.98e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 0, 0, 3, 0, 0, 0],
        s: 1.72e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 1, 0, 0, 1, 0, 0, 0],
        s: 1.41e-6,
        c: 0.01e-6,
    },
    Term {
        nfa: [0, 1, 0, 0, -1, 0, 0, 0],
        s: 1.26e-6,
        c: 0.01e-6,
    },
    Term {
        nfa: [1, 0, 0, 0, -1, 0, 0, 0],
        s: 0.63e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 0, 0, 1, 0, 0, 0],
        s: 0.63e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 1, 2, -2, 3, 0, 0, 0],
        s: -0.46e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 1, 2, -2, 1, 0, 0, 0],
        s: -0.45e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 4, -4, 4, 0, 0, 0],
        s: -0.36e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 1, -1, 1, -8, 12, 0],
        s: 0.24e-6,
        c: 0.12e-6,
    },
    Term {
        nfa: [0, 0, 2, 0, 0, 0, 0, 0],
        s: -0.32e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, 0, 2, 0, 0, 0],
        s: -0.28e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 2, 0, 3, 0, 0, 0],
        s: -0.27e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 2, 0, 1, 0, 0, 0],
        s: -0.26e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 0, 0, 0, 0],
        s: 0.21e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 1, -2, 2, -3, 0, 0, 0],
        s: -0.19e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 1, -2, 2, -1, 0, 0, 0],
        s: -0.18e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 0, 0, 0, 8, -13, -1],
        s: 0.10e-6,
        c: -0.05e-6,
    },
    Term {
        nfa: [0, 0, 0, 2, 0, 0, 0, 0],
        s: -0.15e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [2, 0, -2, 0, -1, 0, 0, 0],
        s: 0.14e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 1, 2, -2, 2, 0, 0, 0],
        s: 0.14e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 0, -2, 1, 0, 0, 0],
        s: -0.14e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 0, -2, -1, 0, 0, 0],
        s: -0.14e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 4, -2, 4, 0, 0, 0],
        s: -0.13e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 4, 0, 0, 0],
        s: 0.11e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, -2, 0, -3, 0, 0, 0],
        s: -0.11e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, -2, 0, -1, 0, 0, 0],
        s: -0.11e-6,
        c: 0.00e-6,
    },
];

/* Terms of order t^1 */
const S1: [Term; 3] = [
    Term {
        nfa: [0, 0, 0, 0, 2, 0, 0, 0],
        s: -0.07e-6,
        c: 3.57e-6,
    },
    Term {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: 1.73e-6,
        c: -0.03e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 3, 0, 0, 0],
        s: 0.00e-6,
        c: 0.48e-6,
    },
];

/* Terms of order t^2 */
const S2: [Term; 25] = [
    Term {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: 743.52e-6,
        c: -0.17e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 2, 0, 0, 0],
        s: 56.91e-6,
        c: 0.06e-6,
    },
    Term {
        nfa: [0, 0, 2, 0, 2, 0, 0, 0],
        s: 9.84e-6,
        c: -0.01e-6,
    },
    Term {
        nfa: [0, 0, 0, 0, 2, 0, 0, 0],
        s: -8.85e-6,
        c: 0.01e-6,
    },
    Term {
        nfa: [0, 1, 0, 0, 0, 0, 0, 0],
        s: -6.38e-6,
        c: -0.05e-6,
    },
    Term {
        nfa: [1, 0, 0, 0, 0, 0, 0, 0],
        s: -3.07e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 1, 2, -2, 2, 0, 0, 0],
        s: 2.23e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, 0, 1, 0, 0, 0],
        s: 1.67e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 2, 0, 2, 0, 0, 0],
        s: 1.30e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 1, -2, 2, -2, 0, 0, 0],
        s: 0.93e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 0, -2, 0, 0, 0, 0],
        s: 0.68e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 1, 0, 0, 0],
        s: -0.55e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, -2, 0, -2, 0, 0, 0],
        s: 0.53e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 0, 2, 0, 0, 0, 0],
        s: -0.27e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 0, 0, 1, 0, 0, 0],
        s: -0.27e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, -2, -2, -2, 0, 0, 0],
        s: -0.26e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 0, 0, -1, 0, 0, 0],
        s: -0.25e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 2, 0, 1, 0, 0, 0],
        s: 0.22e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [2, 0, 0, -2, 0, 0, 0, 0],
        s: -0.21e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [2, 0, -2, 0, -1, 0, 0, 0],
        s: 0.20e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, 2, 2, 0, 0, 0],
        s: 0.17e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [2, 0, 2, 0, 2, 0, 0, 0],
        s: 0.13e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [2, 0, 0, 0, 0, 0, 0, 0],
        s: -0.13e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [1, 0, 2, -2, 2, 0, 0, 0],
        s: -0.12e-6,
        c: 0.00e-6,
    },
    Term {
        nfa: [0, 0, 2, 0, 0, 0, 0, 0],
        s: -0.11e-6,
        c: 0.00e-6,
    },
];

/* Terms of order t^3 */
const S3: [Term; 4] = [
    Term {
        nfa: [0, 0, 0, 0, 1, 0, 0, 0],
        s: 0.30e-6,
        c: -23.42e-6,
    },
    Term {
        nfa: [0, 0, 2, -2, 2, 0, 0, 0],
        s: -0.03e-6,
        c: -1.46e-6,
    },
    Term {
        nfa: [0, 0, 2, 0, 2, 0, 0, 0],
        s: -0.01e-6,
        c: -0.25e-6,
    },
    Term {
        nfa: [0, 0, 0, 0, 2, 0, 0, 0],
        s: 0.00e-6,
        c: 0.23e-6,
    },
];

/* Terms of order t^4 */
const S4: [Term; 1] = [Term {
    nfa: [0, 0, 0, 0, 1, 0, 0, 0],
    s: -0.26e-6,
    c: -0.01e-6,
}];
