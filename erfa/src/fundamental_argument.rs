// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Fundamental argument code.

use std::ops::Rem;

use crate::constants::*;

/// Fundamental argument, IERS Conventions (2003): mean anomaly of the Moon.
/// (`eraFal03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * `l`, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and is from
///    Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
pub fn l03(t: f64) -> f64 {
    /* Mean anomaly of the Moon (IERS Conventions 2003). */
    #[rustfmt::skip]
    let a =
        (     485868.249036  +
    t * ( 1717915923.2178 +
    t * (         31.8792 +
    t * (          0.051635 +
    t * (        - 0.00024470 ) ) ) ) ).rem(ERFA_TURNAS) * ERFA_DAS2R;
    a
}

/// Fundamental argument, IERS Conventions (2003): mean anomaly of the Sun. (`eraFalp03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * `l'`, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and is from
///    Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
pub fn lp03(t: f64) -> f64 {
    /* Mean anomaly of the Sun (IERS Conventions 2003). */
    #[rustfmt::skip]
   let a =
       (         1287104.793048 +
         t * ( 129596581.0481 +
         t * (       - 0.5532 +
         t * (         0.000136 +
         t * (       - 0.00001149 ) ) ) ) ).rem(ERFA_TURNAS) * ERFA_DAS2R;
    a
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of the Moon
/// minus mean longitude of the ascending node. (`eraFaf03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * `F`, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and is from
///    Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
pub fn f03(t: f64) -> f64 {
    /* Mean longitude of the Moon minus that of the ascending node */
    /* (IERS Conventions 2003).                                    */
    #[rustfmt::skip]
   let a =
   (           335779.526232 +
     t * ( 1739527262.8478 +
     t * (       - 12.7512 +
     t * (        - 0.001037 +
     t * (          0.00000417 ) ) ) ) ).rem(ERFA_TURNAS) * ERFA_DAS2R;
    a
}

/// Fundamental argument, IERS Conventions (2003): mean elongation of the Moon
/// from the Sun. (`eraFad03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * `D`, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and is from
///    Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
pub fn d03(t: f64) -> f64 {
    /* Mean elongation of the Moon from the Sun (IERS Conventions 2003). */
    #[rustfmt::skip]
    let a = ((   1072260.703692 +
        t * ( 1602961601.2090 +
        t * (        - 6.3706 +
        t * (          0.006593 +
        t * (        - 0.00003169 ) ) ) )) % ERFA_TURNAS )
        * ERFA_DAS2R;
    a
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of the Moon's
/// ascending node. (`eraFaom03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * `Omega`, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and is from
///    Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J., 1994, Astron.Astrophys. 282, 663-683.
///
pub fn om03(t: f64) -> f64 {
    /* Mean longitude of the Moon's ascending node */
    /* (IERS Conventions 2003).                    */
    #[rustfmt::skip]
    let a =
    ((          450160.398036 +
       t * ( - 6962890.5431 +
       t * (         7.4722 +
       t * (         0.007702 +
       t * (       - 0.00005939 ) ) ) )) % ERFA_TURNAS ) * ERFA_DAS2R;
    a
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Mercury.
/// (`eraFame03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * mean longitude of Mercury, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and comes
///    from Souchay et al. (1999) after Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
/// * Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///   Astron.Astrophys.Supp.Ser. 135, 111
///
pub fn me03(t: f64) -> f64 {
    /* Mean longitude of Mercury (IERS Conventions 2003). */
    (4.402608842 + 2608.7903141574 * t) % ERFA_D2PI
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Venus.
/// (`eraFave03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * mean longitude of Venus, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and comes
///    from Souchay et al. (1999) after Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
/// * Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///   Astron.Astrophys.Supp.Ser. 135, 111
///
pub fn ve03(t: f64) -> f64 {
    /* Mean longitude of Venus (IERS Conventions 2003). */
    (3.176146697 + 1021.3285546211 * t) % ERFA_D2PI
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Earth.
/// (`eraFae03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * mean longitude of Earth, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and comes
///    from Souchay et al. (1999) after Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
/// * Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///   Astron.Astrophys.Supp.Ser. 135, 111
///
pub fn e03(t: f64) -> f64 {
    /* Mean longitude of Earth (IERS Conventions 2003). */
    (1.753470314 + 628.3075849991 * t) % ERFA_D2PI
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Mars.
/// (`eraFama03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * mean longitude of Mars, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and comes
///    from Souchay et al. (1999) after Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
/// * Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///   Astron.Astrophys.Supp.Ser. 135, 111
///
pub fn ma03(t: f64) -> f64 {
    /* Mean longitude of Mars (IERS Conventions 2003). */
    (6.203480913 + 334.0612426700 * t) % ERFA_D2PI
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Jupiter.
/// (`eraFaju03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * mean longitude of Jupiter, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and comes
///    from Souchay et al. (1999) after Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
/// * Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///   Astron.Astrophys.Supp.Ser. 135, 111
///
pub fn ju03(t: f64) -> f64 {
    /* Mean longitude of Jupiter (IERS Conventions 2003). */
    (0.599546497 + 52.9690962641 * t) % ERFA_D2PI
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Saturn.
/// (`eraFasa03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * mean longitude of Saturn, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and comes
///    from Souchay et al. (1999) after Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
/// * Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///   Astron.Astrophys.Supp.Ser. 135, 111
///
pub fn sa03(t: f64) -> f64 {
    /* Mean longitude of Saturn (IERS Conventions 2003). */
    (0.874016757 + 21.3299104960 * t) % ERFA_D2PI
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Uranus.
/// (`eraFaur03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * mean longitude of Uranus, radians (Note 2)
///
/// # Notes:
///
/// 1) Though t is strictly TDB, it is usually more convenient to use TT, which
///    makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and is
///    adapted from Simon et al. (1994).
///
/// # References:
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
pub fn ur03(t: f64) -> f64 {
    /* Mean longitude of Uranus (IERS Conventions 2003). */
    (5.481293872 + 7.4781598567 * t) % ERFA_D2PI
}

/// Fundamental argument, IERS Conventions (2003): general accumulated
/// precession in longitude. (`eraFapa03`)
///
/// Given:
///  * `t`: TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned:
///  * general precession in longitude, radians (Note 2)
///
/// # Notes:
///
/// 1) Though `t` is strictly TDB, it is usually more convenient to use TT,
///    which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003).  It is
///    taken from Kinoshita & Souchay (1990) and comes originally from Lieske et
///    al. (1977).
///
/// # References:
///
/// * Kinoshita, H. and Souchay J. 1990, Celest.Mech. and Dyn.Astron. 48, 187
///
/// * Lieske, J.H., Lederle, T., Fricke, W. & Morando, B. 1977,
///   Astron.Astrophys. 58, 1-16
///
/// * McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003), IERS Technical
///   Note No. 32, BKG (2004)
///
pub fn pa03(t: f64) -> f64 {
    /* General accumulated precession in longitude. */
    (0.024381750 + 0.00000538691 * t) * t
}
