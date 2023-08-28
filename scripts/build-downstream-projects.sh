#!/usr/bin/env bash
#
# Builds known downstream projects against local solana source
#

set -e
cd "$(dirname "$0")"/..
source ci/_
source ci/semver_bash/semver.sh
source scripts/patch-crates.sh
source scripts/read-cargo-variable.sh

miraland_ver=$(readCargoVariable version sdk/Cargo.toml)
miraland_dir=$PWD
cargo="$miraland_dir"/cargo
cargo_build_bpf="$miraland_dir"/cargo-build-bpf
cargo_test_bpf="$miraland_dir"/cargo-test-bpf

mkdir -p target/downstream-projects
cd target/downstream-projects

example_helloworld() {
  (
    set -x
    rm -rf example-helloworld
    git clone https://github.com/solana-labs/example-helloworld.git
    cd example-helloworld

    update_solana_dependencies src/program-rust "$miraland_ver"
    patch_crates_io_solana src/program-rust/Cargo.toml "$miraland_dir"
    echo "[workspace]" >> src/program-rust/Cargo.toml

    $cargo_build_bpf \
      --manifest-path src/program-rust/Cargo.toml

    # TODO: Build src/program-c/...
  )
}

spl() {
  (
    # Mind the order!
    PROGRAMS=(
      instruction-padding/program
      token/program
      token/program-2022
      token/program-2022-test
      associated-token-account/program
      token-upgrade/program
      feature-proposal/program
      governance/addin-mock/program
      governance/program
      memo/program
      name-service/program
      stake-pool/program
    )
    set -x
    rm -rf spl
    # MI: explicitly specify branch for solarti
    git clone --branch solarti-token-3.5 https://github.com/miraland-labs/solarti-program-library.git spl
    cd spl

    project_used_miraland_version=$(sed -nE 's/miraland-sdk = \"[>=<~]*(.*)\"/\1/p' <"token/program/Cargo.toml")
    echo "used miraland version: $project_used_miraland_version"
    if semverGT "$project_used_miraland_version" "$miraland_ver"; then
      echo "skip"
      return
    fi

    ./patch.crates-io.sh "$miraland_dir"

    for program in "${PROGRAMS[@]}"; do
      $cargo_test_bpf --manifest-path "$program"/Cargo.toml
    done

    # TODO better: `build.rs` for solarti-token-cli doesn't seem to properly build
    # the required programs to run the tests, so instead we run the tests
    # after we know programs have been built
    $cargo build
    $cargo test
  )
}

serum_dex() {
  (
    set -x
    rm -rf serum-dex
    git clone https://github.com/project-serum/serum-dex.git
    cd serum-dex

    update_solana_dependencies . "$miraland_ver"
    patch_crates_io_solana Cargo.toml "$miraland_dir"
    patch_crates_io_solana dex/Cargo.toml "$miraland_dir"
    cat >> dex/Cargo.toml <<EOF
[workspace]
exclude = [
    "crank",
    "permissioned",
]
EOF
    $cargo build

    $cargo_build_bpf \
      --manifest-path dex/Cargo.toml --no-default-features --features program

    $cargo test \
      --manifest-path dex/Cargo.toml --no-default-features --features program
  )
}

_ example_helloworld
_ spl
# MI, comment out since serum_dex has not been adapted.
# _ serum_dex
