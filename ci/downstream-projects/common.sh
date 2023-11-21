#!/usr/bin/env bash
set -e

source ci/_
source ci/semver_bash/semver.sh
source scripts/patch-crates.sh
source scripts/read-cargo-variable.sh

MIRALAND_VER=$(readCargoVariable version Cargo.toml)
export MIRALAND_VER
export MIRALAND_DIR=$PWD
export CARGO="$MIRALAND_DIR"/cargo
export CARGO_BUILD_SBF="$MIRALAND_DIR"/cargo-build-sbf
export CARGO_TEST_SBF="$MIRALAND_DIR"/cargo-test-sbf

mkdir -p target/downstream-projects
cd target/downstream-projects
