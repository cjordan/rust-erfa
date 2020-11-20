// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use criterion::*;

use erfa_sys::{eraGd2gc, ERFA_WGS84};

fn erfa(c: &mut Criterion) {
    let mwa_latitude_radians = -0.4660608448386394;
    let mwa_longitude_radians = 2.0362898668561042;
    let mwa_altitude_metres = 377.827;
    let mut mwa_xyz: [f64; 3] = [0.0; 3];

    c.bench_function("eraGd2gc", |b| {
        b.iter(|| {
            unsafe {
                eraGd2gc(
                    ERFA_WGS84 as i32,     // ellipsoid identifier (Note 1)
                    mwa_longitude_radians, // longitude (radians, east +ve)
                    mwa_latitude_radians,  // latitude (geodetic, radians, Note 3)
                    mwa_altitude_metres,   // height above ellipsoid (geodetic, Notes 2,3)
                    mwa_xyz.as_mut_ptr(),  // geocentric vector (Note 2)
                );
            }
        })
    });
}

criterion_group!(benches, erfa);
criterion_main!(benches);
