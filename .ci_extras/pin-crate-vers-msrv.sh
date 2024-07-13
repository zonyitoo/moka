#!/bin/sh

set -eux

# Pin some dependencies to specific versions for the MSRV.
# cargo update -p <crate> --precise <version>
cargo update -p actix-rt --precise 2.9.0
cargo update -p cc --precise 1.0.105
