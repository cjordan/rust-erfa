// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Precession and nutation code.

mod nut00a;
pub use nut00a::nut00a;

use crate::constants::*;

/// Mean obliquity of the ecliptic, IAU 2006 precession model. (`eraObl06`)
///
/// Given:
/// * `date1`,`date2`: TT as a 2-part Julian Date (Note 1)
///
/// Returned:
/// * obliquity of the ecliptic (radians, Note 2)
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
/// 2) The result is the angle between the ecliptic and mean equator of date
///    `date1+date2`.
///
/// # References:
///
/// * Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
///
pub fn obliquity_06(date1: f64, date2: f64) -> f64 {
    /* Interval between fundamental date J2000.0 and given date (JC). */
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    /* Mean obliquity. */
    #[rustfmt::skip]
   let eps0 = (84381.406     +
                   (-46.836769    +
                   ( -0.0001831   +
                   (  0.00200340  +
                   ( -0.000000576 +
                   ( -0.0000000434) * t) * t) * t) * t) * t) * ERFA_DAS2R;
    eps0
}

/// Precession angles, IAU 2006, equinox based. (`eraP06e`)
///
/// Given:
///  * `date1`,`date2`: TT as a 2-part Julian Date (Note 1)
///
/// Returned (see Note 2):
///  * `eps0`: epsilon_0
///  * `psia`: psi_A
///  * `oma`: omega_A
///  * `bpa`: P_A
///  * `bqa`: Q_A
///  * `pia`: pi_A
///  * `bpia`: Pi_A
///  * `epsa`: obliquity epsilon_A
///  * `chia`: chi_A
///  * `za`: z_A
///  * `zetaa`: zeta_A
///  * `thetaa`: theta_A
///  * `pa`: p_A
///  * `gam`: F-W angle gamma_J2000
///  * `phi`: F-W angle phi_J2000
///  * `psi`: F-W angle psi_J2000
///
/// Notes:
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
/// 2) This function returns the set of equinox based angles for the Capitaine
///    et al. "P03" precession theory, adopted by the IAU in
///    2006.  The angles are set out in Table 1 of Hilton et al. (2006):
///
///    * `eps0`   `epsilon_0`   obliquity at J2000.0
///    * `psia`   `psi_A`       luni-solar precession
///    * `oma`    `omega_A`     inclination of equator wrt J2000.0 ecliptic
///    * `bpa`    `P_A`         ecliptic pole x, J2000.0 ecliptic triad
///    * `bqa`    `Q_A`         ecliptic pole -y, J2000.0 ecliptic triad
///    * `pia`    `pi_A`        angle between moving and J2000.0 ecliptics
///    * `bpia`   `Pi_A`        longitude of ascending node of the ecliptic
///    * `epsa`   `epsilon_A`   obliquity of the ecliptic
///    * `chia`   `chi_A`       planetary precession
///    * `za`     `z_A`         equatorial precession: -3rd 323 Euler angle
///    * `zetaa`  `zeta_A`      equatorial precession: -1st 323 Euler angle
///    * `thetaa` `theta_A`     equatorial precession: 2nd 323 Euler angle
///    * `pa`     `p_A`         general precession (n.b. see below)
///    * `gam`    `gamma_J2000` J2000.0 RA difference of ecliptic poles
///    * `phi`    `phi_J2000`   J2000.0 codeclination of ecliptic pole
///    * `psi`    `psi_J2000`   longitude difference of equator poles, J2000.0
///
///    The returned values are all radians.
///
///    Note that the `t^5` coefficient in the series for `p_A` from Capitaine et
///    al. (2003) is incorrectly signed in Hilton et al. (2006).
///
/// 3) Hilton et al. (2006) Table 1 also contains angles that depend on models
///    distinct from the P03 precession theory itself, namely the IAU 2000A
///    frame bias and nutation.  The quoted polynomials are used in other ERFA
///    functions:
///
///    * `eraXy06` contains the polynomial parts of the `X` and `Y` series.
///
///    * `eraS06` contains the polynomial part of the `s+XY/2` series.
///
///    * `eraPfw06`: implements the series for the Fukushima-Williams angles
///      that are with respect to the GCRS pole (i.e. the variants that include
///      frame bias).
///
/// 4) The IAU resolution stipulated that the choice of parameterization was
///    left to the user, and so an IAU compliant precession implementation can
///    be constructed using various combinations of the angles returned by the
///    present function.
///
/// 5) The parameterization used by ERFA is the version of the Fukushima-
///    Williams angles that refers directly to the GCRS pole.  These angles may
///    be calculated by calling the function
///    [`eraPfw06`](precession_angles_fw06).  ERFA also supports the direct
///    computation of the CIP GCRS `X`,`Y` by series, available by calling
///    `eraXy06`.
///
/// 6) The agreement between the different parameterizations is at the 1
///    microarcsecond level in the present era.
///
/// 7) When constructing a precession formulation that refers to the GCRS pole
///    rather than the dynamical pole, it may (depending on the choice of
///    angles) be necessary to introduce the frame bias explicitly.
///
/// 8) It is permissible to re-use the same variable in the returned arguments.
///    The quantities are stored in the stated order.
///
/// References:
///
/// * Capitaine, N., Wallace, P.T. & Chapront, J., 2003, Astron.Astrophys., 412,
///   567
///
/// * Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
///
#[allow(clippy::type_complexity)]
pub fn precession_angles(
    date1: f64,
    date2: f64,
) -> (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
) {
    /* Interval between fundamental date J2000.0 and given date (JC). */
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    /* Obliquity at J2000.0. */
    let eps0 = 84381.406 * ERFA_DAS2R;

    /* Luni-solar precession. */
    #[rustfmt::skip]
    let psia = ( 5038.481507     +
               (   -1.0790069    +
               (   -0.00114045   +
               (    0.000132851  +
               (   -0.0000000951 )
               * t) * t) * t) * t) * t * ERFA_DAS2R;

    /* Inclination of mean equator with respect to the J2000.0 ecliptic. */
    #[rustfmt::skip]
    let oma = eps0 + ( -0.025754     +
                     (  0.0512623    +
                     ( -0.00772503   +
                     ( -0.000000467  +
                     (  0.0000003337 )
                     * t) * t) * t) * t) * t * ERFA_DAS2R;

    /* Ecliptic pole x, J2000.0 ecliptic triad. */
    #[rustfmt::skip]
    let bpa = (  4.199094     +
              (  0.1939873    +
              ( -0.00022466   +
              ( -0.000000912  +
              (  0.0000000120 )
              * t) * t) * t) * t) * t * ERFA_DAS2R;

    /* Ecliptic pole -y, J2000.0 ecliptic triad. */
    #[rustfmt::skip]
    let bqa = ( -46.811015     +
              (   0.0510283    +
              (   0.00052413   +
              (  -0.000000646  +
              (  -0.0000000172 )
              * t) * t) * t) * t) * t * ERFA_DAS2R;

    /* Angle between moving and J2000.0 ecliptics. */
    #[rustfmt::skip]
    let pia = ( 46.998973     +
              ( -0.0334926    +
              ( -0.00012559   +
              (  0.000000113  +
              ( -0.0000000022 )
              * t) * t) * t) * t) * t * ERFA_DAS2R;

    /* Longitude of ascending node of the moving ecliptic. */
    #[rustfmt::skip]
    let bpia = ( 629546.7936      +
               (   -867.95758     +
               (      0.157992    +
               (     -0.0005371   +
               (     -0.00004797  +
               (      0.000000072 )
               * t) * t) * t) * t) * t) * ERFA_DAS2R;

    /* Mean obliquity of the ecliptic. */
    let epsa = obliquity_06(date1, date2);

    /* Planetary precession. */
    #[rustfmt::skip]
    let chia = ( 10.556403     +
               ( -2.3814292    +
               ( -0.00121197   +
               (  0.000170663  +
               ( -0.0000000560 )
               * t) * t) * t) * t) * t * ERFA_DAS2R;

    /* Equatorial precession: minus the third of the 323 Euler angles. */
    #[rustfmt::skip]
    let za = (   -2.650545     +
             ( 2306.077181     +
             (    1.0927348    +
             (    0.01826837   +
             (   -0.000028596  +
             (   -0.0000002904 )
             * t) * t) * t) * t) * t) * ERFA_DAS2R;

    /* Equatorial precession: minus the first of the 323 Euler angles. */
    #[rustfmt::skip]
    let zetaa = (    2.650545     +
                ( 2306.083227     +
                (    0.2988499    +
                (    0.01801828   +
                (   -0.000005971  +
                (   -0.0000003173 )
                * t) * t) * t) * t) * t) * ERFA_DAS2R;

    /* Equatorial precession: second of the 323 Euler angles. */
    #[rustfmt::skip]
    let thetaa = ( 2004.191903     +
                 (   -0.4294934    +
                 (   -0.04182264   +
                 (   -0.000007089  +
                 (   -0.0000001274 )
                 * t) * t) * t) * t) * t * ERFA_DAS2R;

    /* General precession. */
    #[rustfmt::skip]
    let pa = ( 5028.796195     +
             (    1.1054348    +
             (    0.00007964   +
             (   -0.000023857  +
             (   -0.0000000383 )
             * t) * t) * t) * t) * t * ERFA_DAS2R;

    /* Fukushima-Williams angles for precession. */
    #[rustfmt::skip]
   let gam = ( 10.556403     +
             (  0.4932044    +
             ( -0.00031238   +
             ( -0.000002788  +
             (  0.0000000260 )
             * t) * t) * t) * t) * t * ERFA_DAS2R;

    #[rustfmt::skip]
   let phi = eps0 + ( -46.811015     +
                    (   0.0511269    +
                    (   0.00053289   +
                    (  -0.000000440  +
                    (  -0.0000000176 )
                    * t) * t) * t) * t) * t * ERFA_DAS2R;

    #[rustfmt::skip]
    let psi = ( 5038.481507     +
              (    1.5584176    +
              (   -0.00018522   +
              (   -0.000026452  +
              (   -0.0000000148 )
              * t) * t) * t) * t) * t * ERFA_DAS2R;

    (
        eps0, psia, oma, bpa, bqa, pia, bpia, epsa, chia, za, zetaa, thetaa, pa, gam, phi, psi,
    )
}

/// Precession angles, IAU 2006 (Fukushima-Williams 4-angle formulation).
/// (`eraPfw06`)
///
/// Given:
///  * `date1`,`date2`: TT as a 2-part Julian Date (Note 1)
///
/// Returned:
///  * `gamb`: F-W angle gamma_bar (radians)
///  * `phib`: F-W angle phi_bar (radians)
///  * `psib`: F-W angle psi_bar (radians)
///  * `epsa`: F-W angle epsilon_A (radians)
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
/// 2) Naming the following points:
///
///    * `e` = J2000.0 ecliptic pole,
///    * `p` = GCRS pole,
///    * `E` = mean ecliptic pole of date,
///    * `P` = mean pole of date,
///
///    the four Fukushima-Williams angles are as follows:
///
///    * `gamb` = gamma_bar = epE
///    * `phib` = phi_bar = pE
///    * `psib` = psi_bar = pEP
///    * `epsa` = epsilon_A = EP
///
/// 3) The matrix representing the combined effects of frame bias and precession
///    is:
///
///    `PxB = R_1(-epsa).R_3(-psib).R_1(phib).R_3(gamb)``
///
/// 4) The matrix representing the combined effects of frame bias, precession
///    and nutation is simply:
///
///    `NxPxB = R_1(-epsa-dE).R_3(-psib-dP).R_1(phib).R_3(gamb)``
///
///    where `dP` and `dE` are the nutation components with respect to the
///    ecliptic of date.
///
/// References:
///
/// * Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
///
pub fn precession_angles_fw06(date1: f64, date2: f64) -> (f64, f64, f64, f64) {
    /* Interval between fundamental date J2000.0 and given date (JC). */
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    /* P03 bias+precession angles. */
    #[rustfmt::skip]
    let gamb = (    -0.052928     +
                    (    10.556378     +
                    (     0.4932044    +
                    (    -0.00031238   +
                    (    -0.000002788  +
                    (     0.0000000260 )
                    * t) * t) * t) * t) * t) * ERFA_DAS2R;
    #[rustfmt::skip]
    let phib = ( 84381.412819     +
                    (   -46.811016     +
                    (     0.0511268    +
                    (     0.00053289   +
                    (    -0.000000440  +
                    (    -0.0000000176 )
                    * t) * t) * t) * t) * t) * ERFA_DAS2R;
    #[rustfmt::skip]
    let psib = (    -0.041775     +
                    (  5038.481484     +
                    (     1.5584175    +
                    (    -0.00018522   +
                    (    -0.000026452  +
                    (    -0.0000000148 )
                    * t) * t) * t) * t) * t) * ERFA_DAS2R;
    let epsa = obliquity_06(date1, date2);

    (gamb, phib, psib, epsa)
}

/// IAU 2000A nutation with adjustments to match the IAU 2006 precession.
/// (`eraNut06a`)
///
/// Given:
///  * `date1`,`date2`: TT as a 2-part Julian Date (Note 1)
///
/// Returned:
///  * `dpsi`,`deps`: nutation, luni-solar + planetary (Note 2)
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
/// 2) The nutation components in longitude and obliquity are in radians and
///    with respect to the mean equinox and ecliptic of date, IAU 2006
///    precession model (Hilton et al. 2006, Capitaine et al. 2005).
///
/// 3) The function first computes the IAU 2000A nutation, then applies
///    adjustments for (i) the consequences of the change in obliquity from the
///    IAU 1980 ecliptic to the IAU 2006 ecliptic and (ii) the secular variation
///    in the Earth's dynamical form factor J2.
///
/// 4) The present function provides classical nutation, complementing the IAU
///    2000 frame bias and IAU 2006 precession.  It delivers a pole which is at
///    current epochs accurate to a few tens of microarcseconds, apart from the
///    free core nutation.
///
/// # References:
///
/// * Chapront, J., Chapront-Touze, M. & Francou, G. 2002, Astron.Astrophys.
///   387, 700
/// * Lieske, J.H., Lederle, T., Fricke, W. & Morando, B. 1977,
///   Astron.Astrophys. 58, 1-16
/// * Mathews, P.M., Herring, T.A., Buffet, B.A. 2002, J.Geophys.Res. 107, B4.
///   The MHB_2000 code itself was obtained on 9th September 2002 from
///   <ftp://maia.usno.navy.mil/conv2000/chapter5/IAU2000A>.
/// * Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M., Francou,
///   G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
/// * Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///   Astron.Astrophys.Supp.Ser. 135, 111
/// * Wallace, P.T., "Software for Implementing the IAU 2000 Resolutions", in
///   IERS Workshop 5.1 (2002)
///
pub fn nut06a(date1: f64, date2: f64) -> (f64, f64) {
    /* Interval between fundamental date J2000.0 and given date (JC). */
    let t = ((date1 - ERFA_DJ00) + date2) / ERFA_DJC;

    /* Factor correcting for secular variation of J2. */
    let fj2 = -2.7774e-6 * t;

    /* Obtain IAU 2000A nutation. */
    let (dp, de) = nut00a(date1, date2);

    /* Apply P03 adjustments (Wallace & Capitaine, 2006, Eqs.5). */
    let dpsi = dp + dp * (0.4697e-6 + fj2);
    let deps = de + de * fj2;

    (dpsi, deps)
}

/// Form rotation matrix given the Fukushima-Williams angles. (`eraFw2m`)
///
/// Given:
///  * `gamb`: F-W angle gamma_bar (radians)
///  * `phib`: F-W angle phi_bar (radians)
///  * `psi`: F-W angle psi (radians)
///  * `eps`: F-W angle epsilon (radians)
///
/// Returned:
///  * `r`: rotation matrix
///
/// # Notes:
///
/// 1) Naming the following points:
///
///    * `e` = J2000.0 ecliptic pole,
///    * `p` = GCRS pole,
///    * `E` = ecliptic pole of date,
///    * `P` = CIP,
///
///    the four Fukushima-Williams angles are as follows:
///
///    * `gamb = gamma = epE`,
///    * `phib = phi = pE`,
///    * `psi = psi = pEP`,
///    * `eps = epsilon = EP`
///
/// 2) The matrix representing the combined effects of frame bias, precession
///    and nutation is:
///
///    `NxPxB = R_1(-eps).R_3(-psi).R_1(phib).R_3(gamb)`
///
/// 3) The present function can construct three different matrices, depending on
///    which angles are supplied as the arguments `gamb`, `phib`, `psi` and
///    `eps`:
///
///    -  To obtain the nutation x precession x frame bias matrix, first
///       generate the four precession angles known conventionally as
///       `gamma_bar`, `phi_bar`, `psi_bar` and `epsilon_A`, then generate the
///       nutation components `Dpsi` and `Depsilon` and add them to `psi_bar`
///       and `epsilon_A`, and finally call the present function using those
///       four angles as arguments.
///
///    -  To obtain the precession x frame bias matrix, generate the four
///       precession angles and call the present function.
///
///    -  To obtain the frame bias matrix, generate the four precession angles
///       for date J2000.0 and call the present function.
///
///    The nutation-only and precession-only matrices can if necessary be
///    obtained by combining these three appropriately.
///
/// # References:
///
/// * Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
/// * Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
///
pub fn fw_to_matrix(gamb: f64, phib: f64, psi: f64, eps: f64) -> [[f64; 3]; 3] {
    use crate::vectors_and_matrices::{init_matrix, rotate_x, rotate_z};

    let mut r = [[0.0; 3]; 3];

    /* Construct the matrix. */
    init_matrix(&mut r);
    rotate_z(gamb, &mut r);
    rotate_x(phib, &mut r);
    rotate_z(-psi, &mut r);
    rotate_x(-eps, &mut r);

    r
}

/// Form the matrix of precession-nutation for a given date (including frame
/// bias), equinox based, IAU 2006 precession and IAU 2000A nutation models.
/// (`eraPnm06a`)
///
/// Given:
///  * `date1`,`date2`: TT as a 2-part Julian Date (Note 1)
///
/// Returned:
///  * `rbpn`: bias-precession-nutation matrix (Note 2)
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
/// 2) The matrix operates in the sense `V(date) = rbpn * V(GCRS)`, where the
///    p-vector `V(date)` is with respect to the true equatorial triad of date
///    `date1+date2` and the p-vector `V(GCRS)` is with respect to the
///    Geocentric Celestial Reference System (IAU, 2000).
///
/// # References:
///
/// * Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855.
///
pub fn pn_matrix_06a(date1: f64, date2: f64) -> [[f64; 3]; 3] {
    /* Fukushima-Williams angles for frame bias and precession. */
    let (gamb, phib, psib, epsa) = precession_angles_fw06(date1, date2);

    /* Nutation components. */
    let (dp, de) = nut06a(date1, date2);

    /* Equinox based nutation x precession x bias matrix. */
    fw_to_matrix(gamb, phib, psib + dp, epsa + de)
}

/// Extract from the bias-precession-nutation matrix the X,Y coordinates of the
/// Celestial Intermediate Pole. (`eraBpn2xy`)
///
/// Given:
///  * `rbpn`: celestial-to-true matrix (Note 1)
///
/// Returned:
///  * `x`,`y`: Celestial Intermediate Pole (Note 2)
///
/// # Notes:
///
/// 1) The matrix `rbpn` transforms vectors from GCRS to true equator (and CIO
///    or equinox) of date, and therefore the Celestial Intermediate Pole unit
///    vector is the bottom row of the matrix.
///
/// 2) `x` and `y` are components of the Celestial Intermediate Pole unit vector
///    in the Geocentric Celestial Reference System.
///
/// # Reference:
///
/// * "Expressions for the Celestial Intermediate Pole and Celestial Ephemeris
///   Origin consistent with the IAU 2000A precession- nutation model",
///   Astron.Astrophys. 400, 1145-1154 (2003)
///
/// n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///      intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
pub fn bpn_to_xy(rbpn: [[f64; 3]; 3]) -> (f64, f64) {
    (rbpn[2][0], rbpn[2][1])
}

/// Equation of the origins, given the classical NPB matrix and the quantity
/// `s`. (`eraEors`)
///
/// Given:
///  * `rnpb` classical nutation x precession x bias matrix
///  * `s`: the quantity s (the CIO locator) in radians
///
/// Returned:
///  * the equation of the origins in radians
///
/// # Notes:
///
/// 1) The equation of the origins is the distance between the true equinox and
///    the celestial intermediate origin and, equivalently, the difference
///    between Earth rotation angle and Greenwich apparent sidereal time
///    (ERA-GST).  It comprises the precession (since J2000.0) in right
///    ascension plus the equation of the equinoxes (including the small
///    correction terms).
///
/// 2) The algorithm is from Wallace & Capitaine (2006).
///
/// # References:
///
/// * Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
/// * Wallace, P. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
///
pub fn eors(rnpb: [[f64; 3]; 3], s: f64) -> f64 {
    /* Evaluate Wallace & Capitaine (2006) expression (16). */
    let x = rnpb[2][0];
    let ax = x / (1.0 + rnpb[2][2]);
    let xs = 1.0 - ax * x;
    let ys = -ax * rnpb[2][1];
    let zs = -x;
    let p = rnpb[0][0] * xs + rnpb[0][1] * ys + rnpb[0][2] * zs;
    let q = rnpb[1][0] * xs + rnpb[1][1] * ys + rnpb[1][2] * zs;
    if (p != 0.0) || (q != 0.0) {
        s - q.atan2(p)
    } else {
        s
    }
}

/// Precession matrix (including frame bias) from GCRS to a specified date, IAU
/// 2006 model. (`eraPmat06`)
///
/// Given:
/// * `date1`,`date2`: TT as a 2-part Julian Date (Note 1)
///
/// Returned:
/// * `rbp`: bias-precession matrix (Note 2)
///
/// # Notes:
///
/// 1) The TT date `date1+date2` is a Julian Date, apportioned in any
///    convenient way between the two arguments.  For example,
///    `JD(TT)=2450123.7` could be expressed in any of these ways, among others:
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
/// 2) The matrix operates in the sense `V(date) = rbp * V(GCRS)`, where the
///    p-vector `V(GCRS)` is with respect to the Geocentric Celestial Reference
///    System (IAU, 2000) and the p-vector `V(date)` is with respect to the mean
///    equatorial triad of the given date.
///
/// # References:
///
/// * Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
/// * IAU: Trans. International Astronomical Union, Vol. XXIVB;  Proc. 24th
///   General Assembly, Manchester, UK.  Resolutions B1.3, B1.6. (2000)
///
/// * Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
///
pub fn precession_matrix_06(date1: f64, date2: f64) -> [[f64; 3]; 3] {
    /* Bias-precession Fukushima-Williams angles. */
    let (gamb, phib, psib, epsa) = precession_angles_fw06(date1, date2);

    /* Form the matrix. */
    fw_to_matrix(gamb, phib, psib, epsa)
}
