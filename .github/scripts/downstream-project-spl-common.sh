#!/usr/bin/env bash
set -e

here="$(dirname "${BASH_SOURCE[0]}")"

#shellcheck source=ci/downstream-projects/common.sh
source "$here"/../../ci/downstream-projects/common.sh

set -x
rm -rf spl
git clone https://github.com/miraland-labs/solarti-program-library.git spl

# copy toolchain file to use miraland's rust version
cp "$MIRALAND_DIR"/rust-toolchain.toml spl/
cd spl || exit 1

project_used_miraland_version=$(sed -nE 's/miraland-sdk = \"[>=<~]*(.*)\"/\1/p' <"token/program/Cargo.toml")
echo "used miraland version: $project_used_miraland_version"
if semverGT "$project_used_miraland_version" "$MIRALAND_VER"; then
  echo "skip"
  return
fi

./patch.crates-io.sh "$MIRALAND_DIR"
