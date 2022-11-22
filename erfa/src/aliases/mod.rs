// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Function names used by the ERFA C library.

#[cfg(test)]
mod tests;

pub use crate::{
    earth::{earth_rotation_angle_00 as eraEra00, position_velocity_00 as eraEpv00},
    fundamental_argument::{
        d03 as eraFad03, e03 as eraFae03, f03 as eraFaf03, ju03 as eraFaju03, l03 as eraFal03,
        lp03 as eraFalp03, ma03 as eraFama03, me03 as eraFame03, om03 as eraFaom03,
        pa03 as eraFapa03, sa03 as eraFasa03, ur03 as eraFaur03, ve03 as eraFave03,
    },
    misc::norm_angle as eraAnp,
    prenut::{
        bpn_to_xy as eraBpn2xy, eors as eraEors, fw_to_matrix as eraFw2m, nut00a as eraNut00a,
        nut06a as eraNut06a, obliquity_06 as eraObl06, pn_matrix_06a as eraPnm06a,
        precession_angles as eraP06e, precession_angles_fw06 as eraPfw06,
        precession_matrix_06 as eraPmat06,
    },
    separation::{sep_spherical_coords as eraSeps, sep_vectors as eraSepp},
    time::{
        gmst06 as eraGmst06, gst06 as eraGst06, gst06a as eraGst06a,
        julian_date_to_epoch as eraEpj, julian_epoch_to_date as eraEpj2jd, S06 as eraS06,
    },
    transform::{
        azel_to_hadec as eraAe2hd, cartesian_to_spherical as eraC2s,
        geocentric_to_geodetic as eraGc2gd, geocentric_to_geodetic_inner as eraGc2gde,
        geodetic_to_geocentric as eraGd2gc, geodetic_to_geocentric_inner as eraGd2gce,
        hadec_to_azel as eraHd2ae, hadec_to_parallactic_angle as eraHd2pa,
        spherical_to_cartesian as eraS2c,
    },
    vectors_and_matrices::{
        copy_matrix as eraCr, copy_vector as eraCp, init_matrix as eraIr, inner_product as eraPdp,
        mat_mul_pvec as eraRxp, mat_mul_pvvec as eraRxpv, modulus as eraPm,
        modulus_and_unit_vector as eraPn, multiply as eraSxp, multiply_matrices as eraRxr,
        outer_product as eraPvxpv, rotate_x as eraRx, rotate_z as eraRz,
    },
};
