#!/bin/bash

# This should be run in the erfa-sys project directory.

# This command would be all that's needed, but there seems to be issues in
# bindgen with parsing macro-defined floats (which are several of ERFA's
# constants). https://github.com/rust-lang/rust-bindgen/issues/1928
bindgen ext/erfa/src/erfa.h \
    --allowlist-function "era.*" \
    --allowlist-var "ERFA_.*" \
    > src/erfa.rs

# Get the missing floats.
while IFS= read -r LINE; do
    echo "${LINE}"
    NAME=$(echo "${LINE}" | cut -d' ' -f2)
    # Some values have unnecessary parentheses.
    VAL=$(echo "${LINE}" | cut -d' ' -f3- | tr -d '(' | tr -d ')')
    if [[ "${VAL}" == *e* || "${VAL}" == *.* || "${VAL}" == */* ]]; then
        echo "pub const ${NAME}: f64 = ${VAL};" >> src/erfa.rs
    else
        echo "pub const ${NAME}: i32 = ${VAL};" >> src/erfa.rs
    fi
done < <(awk '/^#define ERFA_.*/ && !/</ && !/>/' ext/erfa/src/erfam.h)
