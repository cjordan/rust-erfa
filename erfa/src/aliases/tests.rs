// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_snake_case)]

use approx::assert_abs_diff_eq;

use super::{
    eraAe2hd, eraAnp, eraBpn2xy, eraC2s, eraCp, eraCr, eraEors, eraEpj, eraEpj2jd, eraEpv00,
    eraEra00, eraFad03, eraFae03, eraFaf03, eraFaju03, eraFal03, eraFalp03, eraFama03, eraFame03,
    eraFaom03, eraFapa03, eraFasa03, eraFaur03, eraFave03, eraFw2m, eraGc2gd, eraGc2gde, eraGd2gc,
    eraGd2gce, eraGmst06, eraGst06, eraGst06a, eraHd2ae, eraHd2pa, eraIr, eraNut00a, eraNut06a,
    eraObl06, eraP06e, eraPdp, eraPfw06, eraPm, eraPmat06, eraPn, eraPnm06a, eraPvxpv, eraRx,
    eraRxp, eraRxpv, eraRxr, eraRz, eraS06, eraS2c, eraSepp, eraSeps, eraSxp,
};
use crate::Ellipsoid;

#[test]
fn test_eraAe2hd() {
    let az = 0.123;
    let el = 0.456;
    let phi = 1.23;
    let result = eraAe2hd(az, el, phi);
    let expected = unsafe {
        let mut ha = 0.0;
        let mut dec = 0.0;
        erfa_sys::eraAe2hd(az, el, phi, &mut ha, &mut dec);
        (ha, dec)
    };
    assert_abs_diff_eq!(result.0, expected.0);
    assert_abs_diff_eq!(result.1, expected.1);
}

#[test]
fn test_eraAnp() {
    for a in [-0.5, 0.5, 3.5] {
        let result = eraAnp(a);
        let expected = unsafe { erfa_sys::eraAnp(a) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraBpn2xy() {
    let mut m = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    let result = eraBpn2xy(m);
    let expected = unsafe {
        let mut x = 0.0;
        let mut y = 0.0;
        erfa_sys::eraBpn2xy(m.as_mut_ptr(), &mut x, &mut y);
        (x, y)
    };
    assert_abs_diff_eq!(result.0, expected.0);
    assert_abs_diff_eq!(result.1, expected.1);
}

#[test]
fn test_eraCp() {
    let mut p = [1.0, 2.0, 3.0];
    let result = eraCp(p);
    let expected = unsafe {
        let mut p2 = [0.0; 3];
        erfa_sys::eraCp(p.as_mut_ptr(), p2.as_mut_ptr());
        p2
    };
    assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
}

#[test]
fn test_eraCr() {
    let mut r = [[1.0, 2.0, 3.0], [1.0, 2.0, 3.0], [1.0, 2.0, 3.0]];
    let result = eraCr(r);
    let expected = unsafe {
        let mut r2 = [[0.0; 3]; 3];
        erfa_sys::eraCr(r.as_mut_ptr(), r2.as_mut_ptr());
        r2
    };
    assert_abs_diff_eq!(result[0].as_slice(), expected[0].as_slice());
    assert_abs_diff_eq!(result[1].as_slice(), expected[1].as_slice());
    assert_abs_diff_eq!(result[2].as_slice(), expected[2].as_slice());
}

#[test]
fn test_eraC2s() {
    let mut p = [1.0, 2.0, 3.0];
    let result = eraC2s(p);
    let expected = unsafe {
        let mut theta = 0.0;
        let mut phi = 0.0;
        erfa_sys::eraC2s(p.as_mut_ptr(), &mut theta, &mut phi);
        (theta, phi)
    };
    assert_abs_diff_eq!(result.0, expected.0);
    assert_abs_diff_eq!(result.1, expected.1);
}

#[test]
fn test_eraEpj() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraEpj(date1, date2);
        let expected = unsafe { erfa_sys::eraEpj(date1, date2) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraEpj2jd() {
    for epj in [1996.8, 2006.8] {
        let result = eraEpj2jd(epj);
        let expected = unsafe {
            let mut djm0 = 0.0;
            let mut djm = 0.0;
            erfa_sys::eraEpj2jd(epj, &mut djm0, &mut djm);
            (djm0, djm)
        };
        assert_abs_diff_eq!(result.0, expected.0);
        assert_abs_diff_eq!(result.1, expected.1);
    }
}

#[test]
fn test_eraEors() {
    let mut m = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    for s in [-0.5, 0.5, 4.1] {
        let result = eraEors(m, s);
        let expected = unsafe { erfa_sys::eraEors(m.as_mut_ptr(), s) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraEpv00() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraEpv00(date1, date2);
        let expected = unsafe {
            let mut pvh = [[0.0; 3]; 2];
            let mut pvb = [[0.0; 3]; 2];
            let status = erfa_sys::eraEpv00(date1, date2, pvh.as_mut_ptr(), pvb.as_mut_ptr());
            (status, pvh, pvb)
        };
        assert_eq!(result.0, expected.0 == 1);
        assert_abs_diff_eq!(result.1[0].as_slice(), expected.1[0].as_slice());
        assert_abs_diff_eq!(result.1[1].as_slice(), expected.1[1].as_slice());
        assert_abs_diff_eq!(result.2[0].as_slice(), expected.2[0].as_slice());
        assert_abs_diff_eq!(result.2[1].as_slice(), expected.2[1].as_slice());
    }
}

#[test]
fn test_eraEra00() {
    for (dj1, dj2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraEra00(dj1, dj2);
        let expected = unsafe { erfa_sys::eraEra00(dj1, dj2) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFad03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFad03(t);
        let expected = unsafe { erfa_sys::eraFad03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFae03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFae03(t);
        let expected = unsafe { erfa_sys::eraFae03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFaf03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFaf03(t);
        let expected = unsafe { erfa_sys::eraFaf03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFaju03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFaju03(t);
        let expected = unsafe { erfa_sys::eraFaju03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFal03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFal03(t);
        let expected = unsafe { erfa_sys::eraFal03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFalp03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFalp03(t);
        let expected = unsafe { erfa_sys::eraFalp03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFama03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFama03(t);
        let expected = unsafe { erfa_sys::eraFama03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFame03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFame03(t);
        let expected = unsafe { erfa_sys::eraFame03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFaom03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFaom03(t);
        let expected = unsafe { erfa_sys::eraFaom03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFapa03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFapa03(t);
        let expected = unsafe { erfa_sys::eraFapa03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFasa03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFasa03(t);
        let expected = unsafe { erfa_sys::eraFasa03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFaur03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFaur03(t);
        let expected = unsafe { erfa_sys::eraFaur03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFave03() {
    for t in [0.1, 1.2, 12.34] {
        let result = eraFave03(t);
        let expected = unsafe { erfa_sys::eraFave03(t) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraFw2m() {
    let gamb = 0.1;
    let phib = 0.2;
    let psi = 0.3;
    let eps = 0.4;
    let result = eraFw2m(gamb, phib, psi, eps);
    let expected = unsafe {
        let mut r = [[0.0; 3]; 3];
        erfa_sys::eraFw2m(gamb, phib, psi, eps, r.as_mut_ptr());
        r
    };
    assert_abs_diff_eq!(result[0].as_slice(), expected[0].as_slice());
    assert_abs_diff_eq!(result[1].as_slice(), expected[1].as_slice());
    assert_abs_diff_eq!(result[2].as_slice(), expected[2].as_slice());
}

#[test]
fn test_eraGc2gd() {
    let mut xyz = [0.1, 0.2, 0.3];
    for e in [Ellipsoid::WGS84, Ellipsoid::GRS80, Ellipsoid::WGS72] {
        let result = eraGc2gd(e, xyz);
        let mut elong = 0.0;
        let mut phi = 0.0;
        let mut height = 0.0;
        let expected = unsafe {
            let status = erfa_sys::eraGc2gd(
                e as i32,
                xyz.as_mut_ptr(),
                &mut elong,
                &mut phi,
                &mut height,
            );
            if status != 0 {
                panic!("erfa_sys::eraGc2gd failed: status = {}", status);
            }
            [elong, phi, height]
        };
        assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
    }
}

#[test]
fn test_eraGc2gde() {
    let mut xyz = [0.1, 0.2, 0.3];
    for e in [Ellipsoid::WGS84, Ellipsoid::GRS80, Ellipsoid::WGS72] {
        let (a, f) = e.get_params();
        let result = eraGc2gde(a, f, xyz).unwrap();
        let mut elong = 0.0;
        let mut phi = 0.0;
        let mut height = 0.0;
        let expected = unsafe {
            let status =
                erfa_sys::eraGc2gde(a, f, xyz.as_mut_ptr(), &mut elong, &mut phi, &mut height);
            if status != 0 {
                panic!("erfa_sys::eraGc2gde failed: status = {}", status);
            }
            [elong, phi, height]
        };
        assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
    }
}

#[test]
fn test_eraGd2gc() {
    let (elong, phi, height) = (0.1, 0.2, 0.3);
    for e in [Ellipsoid::WGS84, Ellipsoid::GRS80, Ellipsoid::WGS72] {
        let result = eraGd2gc(e, elong, phi, height).unwrap();
        let expected = unsafe {
            let mut xyz = [0.0; 3];
            let status = erfa_sys::eraGd2gc(e as i32, elong, phi, height, xyz.as_mut_ptr());
            if status != 0 {
                panic!("erfa_sys::eraGd2gc failed: status = {}", status);
            }
            xyz
        };
        assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
    }
}

#[test]
fn test_eraGd2gce() {
    let (elong, phi, height) = (0.1, 0.2, 0.3);
    for e in [Ellipsoid::WGS84, Ellipsoid::GRS80, Ellipsoid::WGS72] {
        let (a, f) = e.get_params();
        let result = eraGd2gce(a, f, elong, phi, height).unwrap();
        let expected = unsafe {
            let mut xyz = [0.0; 3];
            let status = erfa_sys::eraGd2gce(a, f, elong, phi, height, xyz.as_mut_ptr());
            if status != 0 {
                panic!("erfa_sys::eraGd2gc failed: status = {}", status);
            }
            xyz
        };
        assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
    }
}

#[test]
fn test_eraGmst06() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraGmst06(date1, date2, date1, date2);
        let expected = unsafe { erfa_sys::eraGmst06(date1, date2, date1, date2) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraGst06() {
    let mut rnpb = [[0.9, 0.05, 0.0], [0.05, 0.9, 0.05], [0.0, 0.05, 0.9]];
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraGst06(date1, date2, date1, date2, rnpb);
        let expected = unsafe { erfa_sys::eraGst06(date1, date2, date1, date2, rnpb.as_mut_ptr()) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraGst06a() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraGst06a(date1, date2, date1, date2);
        let expected = unsafe { erfa_sys::eraGst06a(date1, date2, date1, date2) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraHd2ae() {
    let ha = 0.123;
    let dec = 0.456;
    let phi = 1.23;
    let result = eraHd2ae(ha, dec, phi);
    let expected = unsafe {
        let mut az = 0.0;
        let mut el = 0.0;
        erfa_sys::eraHd2ae(ha, dec, phi, &mut az, &mut el);
        (az, el)
    };
    assert_abs_diff_eq!(result.0, expected.0);
    assert_abs_diff_eq!(result.1, expected.1);
}

#[test]
fn test_eraHd2pa() {
    let ha = 0.123;
    let dec = 0.456;
    let phi = 1.23;
    let result = eraHd2pa(ha, dec, phi);
    let expected = unsafe { erfa_sys::eraHd2pa(ha, dec, phi) };
    assert_abs_diff_eq!(result, expected);
}

#[test]
fn test_eraIr() {
    let mut r = [[0.0; 3]; 3];
    let mut r2 = [[0.0; 3]; 3];
    eraIr(&mut r);
    unsafe {
        erfa_sys::eraIr(r2.as_mut_ptr());
    }
    assert_abs_diff_eq!(r[0].as_slice(), r2[0].as_slice());
    assert_abs_diff_eq!(r[1].as_slice(), r2[1].as_slice());
    assert_abs_diff_eq!(r[2].as_slice(), r2[2].as_slice());
}

#[test]
fn test_eraNut00a() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraNut00a(date1, date2);
        let expected = unsafe {
            let mut expected = (0.0, 0.0);
            erfa_sys::eraNut00a(date1, date2, &mut expected.0, &mut expected.1);
            expected
        };
        assert_abs_diff_eq!(result.0, expected.0);
        assert_abs_diff_eq!(result.1, expected.1);
    }
}

#[test]
fn test_eraNut06a() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraNut06a(date1, date2);
        let expected = unsafe {
            let mut expected = (0.0, 0.0);
            erfa_sys::eraNut06a(date1, date2, &mut expected.0, &mut expected.1);
            expected
        };
        assert_abs_diff_eq!(result.0, expected.0);
        assert_abs_diff_eq!(result.1, expected.1);
    }
}

#[test]
fn test_eraObl06() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraObl06(date1, date2);
        let expected = unsafe { erfa_sys::eraObl06(date1, date2) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraP06e() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraP06e(date1, date2);
        let expected = unsafe {
            let mut expected = (
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            );
            erfa_sys::eraP06e(
                date1,
                date2,
                &mut expected.0,
                &mut expected.1,
                &mut expected.2,
                &mut expected.3,
                &mut expected.4,
                &mut expected.5,
                &mut expected.6,
                &mut expected.7,
                &mut expected.8,
                &mut expected.9,
                &mut expected.10,
                &mut expected.11,
                &mut expected.12,
                &mut expected.13,
                &mut expected.14,
                &mut expected.15,
            );
            expected
        };
        assert_abs_diff_eq!(result.0, expected.0);
        assert_abs_diff_eq!(result.1, expected.1);
        assert_abs_diff_eq!(result.2, expected.2);
        assert_abs_diff_eq!(result.3, expected.3);
        assert_abs_diff_eq!(result.4, expected.4);
        assert_abs_diff_eq!(result.5, expected.5);
        assert_abs_diff_eq!(result.6, expected.6);
        assert_abs_diff_eq!(result.7, expected.7);
        assert_abs_diff_eq!(result.8, expected.8);
        assert_abs_diff_eq!(result.9, expected.9);
        assert_abs_diff_eq!(result.10, expected.10);
        assert_abs_diff_eq!(result.11, expected.11);
        assert_abs_diff_eq!(result.12, expected.12);
        assert_abs_diff_eq!(result.13, expected.13);
        assert_abs_diff_eq!(result.14, expected.14);
        assert_abs_diff_eq!(result.15, expected.15);
    }
}

#[test]
fn test_eraPdp() {
    let mut a = [1.0, 2.0, 3.0];
    let mut b = [2.0, -3.0, 4.0];
    let result = eraPdp(a, b);
    let expected = unsafe { erfa_sys::eraPdp(a.as_mut_ptr(), b.as_mut_ptr()) };
    assert_abs_diff_eq!(result, expected);
}

#[test]
fn test_eraPfw06() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraPfw06(date1, date2);
        let expected = unsafe {
            let mut expected = (0.0, 0.0, 0.0, 0.0);
            erfa_sys::eraPfw06(
                date1,
                date2,
                &mut expected.0,
                &mut expected.1,
                &mut expected.2,
                &mut expected.3,
            );
            expected
        };
        assert_abs_diff_eq!(result.0, expected.0);
        assert_abs_diff_eq!(result.1, expected.1);
        assert_abs_diff_eq!(result.2, expected.2);
        assert_abs_diff_eq!(result.3, expected.3);
    }
}

#[test]
fn test_eraPm() {
    let mut a = [1.0, 2.0, 3.0];
    let result = eraPm(a);
    let expected = unsafe { erfa_sys::eraPm(a.as_mut_ptr()) };
    assert_abs_diff_eq!(result, expected);
}

#[test]
fn test_eraPmat06() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraPmat06(date1, date2);
        let expected = unsafe {
            let mut expected = [[0.0; 3]; 3];
            erfa_sys::eraPmat06(date1, date2, expected.as_mut_ptr());
            expected
        };
        assert_abs_diff_eq!(result[0].as_slice(), expected[0].as_slice());
        assert_abs_diff_eq!(result[1].as_slice(), expected[1].as_slice());
        assert_abs_diff_eq!(result[2].as_slice(), expected[2].as_slice());
    }
}

#[test]
fn test_eraPn() {
    let mut a = [1.0, 2.0, 3.0];
    let result = eraPn(a);
    let expected = unsafe {
        let mut expected = (0.0, [0.0; 3]);
        erfa_sys::eraPn(a.as_mut_ptr(), &mut expected.0, expected.1.as_mut_ptr());
        expected
    };
    assert_abs_diff_eq!(result.0, expected.0);
    assert_abs_diff_eq!(result.1.as_slice(), expected.1.as_slice());
}

#[test]
fn test_eraPnm06a() {
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraPnm06a(date1, date2);
        let expected = unsafe {
            let mut expected = [[0.0; 3]; 3];
            erfa_sys::eraPnm06a(date1, date2, expected.as_mut_ptr());
            expected
        };
        assert_abs_diff_eq!(result[0].as_slice(), expected[0].as_slice());
        assert_abs_diff_eq!(result[1].as_slice(), expected[1].as_slice());
        assert_abs_diff_eq!(result[2].as_slice(), expected[2].as_slice());
    }
}

#[test]
fn test_eraPvxpv() {
    let mut a = [1.0, 2.0, 3.0];
    let mut b = [2.0, -3.0, 4.0];
    let result = eraPvxpv(a, b);
    let expected = unsafe {
        let mut expected = [0.0; 3];
        erfa_sys::eraPvxpv(&mut a, &mut b, &mut expected);
        expected
    };
    assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
}

#[test]
fn test_eraRx() {
    for phi in [0.12, 4.5, 123.69] {
        let mut r = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 11.0]];
        let mut r2 = r;
        eraRx(phi, &mut r);
        unsafe {
            erfa_sys::eraRx(phi, r2.as_mut_ptr());
        };
        assert_abs_diff_eq!(r[0].as_slice(), r2[0].as_slice());
        assert_abs_diff_eq!(r[1].as_slice(), r2[1].as_slice());
        assert_abs_diff_eq!(r[2].as_slice(), r2[2].as_slice());
    }
}

#[test]
fn test_eraRxp() {
    let mut r = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]];
    let mut p = [4.0, 5.0, 6.0];
    let result = eraRxp(r, p);
    let expected = unsafe {
        let mut rp = [0.0; 3];
        erfa_sys::eraRxp(r.as_mut_ptr(), p.as_mut_ptr(), rp.as_mut_ptr());
        rp
    };
    assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
}

#[test]
fn test_eraRxr() {
    let mut a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]];
    let mut b = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 11.0]];
    let result = eraRxr(a, b);
    let expected = unsafe {
        let mut atb = [[0.0; 3]; 3];
        erfa_sys::eraRxr(a.as_mut_ptr(), b.as_mut_ptr(), atb.as_mut_ptr());
        atb
    };
    assert_abs_diff_eq!(result[0].as_slice(), expected[0].as_slice());
    assert_abs_diff_eq!(result[1].as_slice(), expected[1].as_slice());
    assert_abs_diff_eq!(result[2].as_slice(), expected[2].as_slice());
}

#[test]
fn test_eraRxpv() {
    let mut r = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]];
    let mut pv = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    let result = eraRxpv(r, pv);
    let expected = unsafe {
        let mut rpv = [[0.0; 3]; 2];
        erfa_sys::eraRxpv(r.as_mut_ptr(), pv.as_mut_ptr(), rpv.as_mut_ptr());
        rpv
    };
    assert_abs_diff_eq!(result[0].as_slice(), expected[0].as_slice());
    assert_abs_diff_eq!(result[1].as_slice(), expected[1].as_slice());
}

#[test]
fn test_eraRz() {
    for phi in [0.12, 4.5, 123.69] {
        let mut r = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 11.0]];
        let mut r2 = r;
        eraRz(phi, &mut r);
        unsafe {
            erfa_sys::eraRz(phi, r2.as_mut_ptr());
        };
        assert_abs_diff_eq!(r[0].as_slice(), r2[0].as_slice());
        assert_abs_diff_eq!(r[1].as_slice(), r2[1].as_slice());
        assert_abs_diff_eq!(r[2].as_slice(), r2[2].as_slice());
    }
}

#[test]
fn test_eraS06() {
    let x = 0.123;
    let y = -0.234;
    for (date1, date2) in [
        (2450123.7, 0.0),
        (2451545.0, -1421.3),
        (2400000.5, 50123.2),
        (2450123.5, 0.2),
        (2450143.5, -0.2),
    ] {
        let result = eraS06(date1, date2, x, y);
        let expected = unsafe { erfa_sys::eraS06(date1, date2, x, y) };
        assert_abs_diff_eq!(result, expected);
    }
}

#[test]
fn test_eraS2c() {
    let theta = 0.45;
    let phi = 0.3;
    let result = eraS2c(theta, phi);
    let expected = unsafe {
        let mut expected = [0.0; 3];
        erfa_sys::eraS2c(theta, phi, expected.as_mut_ptr());
        expected
    };
    assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
}

#[test]
fn test_eraSepp() {
    let mut a = [0.0, 2.0, 1.0];
    let mut b = [0.0, 1.0, 2.0];
    let result = eraSepp(a, b);
    let expected = unsafe { erfa_sys::eraSepp(a.as_mut_ptr(), b.as_mut_ptr()) };
    assert_abs_diff_eq!(result, expected);
}

#[test]
fn test_eraSeps() {
    let a_long = 0.4;
    let a_lat = 0.3;
    let b_long = 0.1;
    let b_lat = 0.6;
    let result = eraSeps(a_long, a_lat, b_long, b_lat);
    let expected = unsafe { erfa_sys::eraSeps(a_long, a_lat, b_long, b_lat) };
    assert_abs_diff_eq!(result, expected);
}

#[test]
fn test_eraSxp() {
    let s = 0.4;
    let mut p = [3.0, 2.0, 1.0];
    let result = eraSxp(s, p);
    let expected = unsafe {
        let mut expected = [0.0; 3];
        erfa_sys::eraSxp(s, p.as_mut_ptr(), expected.as_mut_ptr());
        expected
    };
    assert_abs_diff_eq!(result.as_slice(), expected.as_slice());
}
