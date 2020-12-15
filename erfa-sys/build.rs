// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This code is adapted from pkg-config-rs
// (https://github.com/rust-lang/pkg-config-rs).
#[cfg(not(feature = "static"))]
#[allow(clippy::if_same_then_else, clippy::needless_bool)]
fn infer_static(name: &str) -> bool {
    if std::env::var(format!("{}_STATIC", name.to_uppercase())).is_ok() {
        true
    } else if std::env::var(format!("{}_DYNAMIC", name.to_uppercase())).is_ok() {
        false
    } else if std::env::var("PKG_CONFIG_ALL_STATIC").is_ok() {
        true
    } else if std::env::var("PKG_CONFIG_ALL_DYNAMIC").is_ok() {
        false
    } else {
        false
    }
}

#[cfg(not(feature = "static"))]
fn bind_erfa() {
    // See if ERFA_LIB is defined. If so, use it to search and link the library.
    match std::env::var("ERFA_LIB") {
        Ok(lib) => {
            println!("cargo:rustc-link-search=native={}", lib);
            println!("cargo:rustc-link-lib=erfa");
        }

        // Search via pkg-config.
        Err(_) => {
            pkg_config::probe_library("erfa")
                .unwrap_or_else(|_| panic!("Couldn't find the ERFA library via pkg-config"));
        }
    }

    // Because pkg-config-rs is very restrictive of allowing things to be
    // compiled statically, manually specify that we should link statically here
    // (https://github.com/rust-lang/pkg-config-rs/issues/102).
    if infer_static("ERFA") {
        println!("cargo:rustc-link-lib=static=erfa");
    }
}

fn main() {
    // The "static" feature means that we compile the ERFA source directly and
    // link it. If we're not using that feature, then we need to find the
    // library and link that instead.
    #[cfg(not(feature = "static"))]
    bind_erfa();

    #[cfg(feature = "static")]
    {
        // Change this directory if the source code is updated.
        let erfa_project_dir = std::path::PathBuf::from("ext/erfa-1.7.1");
        if !erfa_project_dir.exists() {
            panic!(
                "Expected to find ERFA source directory {}",
                erfa_project_dir.display()
            );
        }

        // Translate rustc optimisation levels to things a C compiler can
        // understand. I don't know if all C compilers agree here, but it should
        // at least work for gcc.
        let opt_level: String = match std::env::var("OPT_LEVEL").as_ref().map(|o| o.as_str()) {
            Err(_) => panic!("Something wrong with OPT_LEVEL"),
            // gcc doesn't handle 'z'. Just set it to 's', which also optimises
            // for size.
            Ok("z") => "s",
            Ok(o) => o
        }.to_string();
        let dst = autotools::Config::new(erfa_project_dir)
            .disable_shared()
            .cflag("-Wall")
            .cflag(format!("-O{}", opt_level))
            .build();

        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-lib=static=erfa");
    }
}
