#!/bin/bash

# This should be run in the erfa-sys project directory.

# This command would be all that's needed, but there seems to be a bug with
# bindgen in that it does not parse macro-defined floats with exponents listed
# (which are several of ERFA's constants.)
# https://github.com/rust-lang/rust-bindgen/issues/1928
bindgen ext/erfa-*.*.*/src/erfa.h --whitelist-function "era.*" --whitelist-var "ERFA_.*" > src/erfa.rs

# Get the missing floats.
grep "^#define ERFA.*[0123456789]e" ext/erfa-*.*.*/src/erfam.h | while IFS= read -r line; do
    echo $line
    NAME=$(echo "${line}" | cut -d' ' -f2)
    # Some values have unnecessary parentheses.
    VAL=$(echo "${line}" | cut -d' ' -f3- | tr -d '(' | tr -d ')')
    echo "pub const ${NAME}: f64 = ${VAL};" >> src/erfa.rs
done
