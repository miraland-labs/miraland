#!/usr/bin/env bash
#
# Fetches the latest SPL programs and produces the miraland-genesis command-line
# arguments needed to install them
#

set -e

fetch_program() {
  declare name=$1
  declare version=$2
  declare address=$3
  declare loader=$4

  declare so=spl_$name-$version.so

  genesis_args+=(--bpf-program "$address" "$loader" "$so")

  if [[ -r $so ]]; then
    return
  fi

  if [[ -r ~/.cache/solarti-spl/$so ]]; then
    cp ~/.cache/solarti-spl/"$so" "$so"
  else
    echo "Downloading $name $version"
    so_name="spl_${name//-/_}.so"
    (
      set -x
      curl -L --retry 5 --retry-delay 2 --retry-connrefused \
        -o "$so" \
        "https://github.com/miraland-labs/solarti-program-library/releases/download/$name-v$version/$so_name"
    )

    mkdir -p ~/.cache/solarti-spl
    cp "$so" ~/.cache/solarti-spl/"$so"
  fi

}

fetch_program token 3.5.3 Token4Q2B47VCdUy8u3rSTMMk2bGA1k7eN8qfKSzdiM BPFLoader2111111111111111111111111111111111
fetch_program token-2022 0.6.4 Token8N5ecJeFxL83iFa2h7AgJ8AtufM7bbg63LrW89 BPFLoaderUpgradeab1e11111111111111111111111
fetch_program memo  1.0.1 MemojWWmbFiRdUEQtRpMkeeyNB181Mr9uWEzdrgHUnc BPFLoader1111111111111111111111111111111111
fetch_program memo  3.0.4 MemoE5FsL5zWDjihivRfHTpkR8RdviRbziKsfGS9Ntd BPFLoader2111111111111111111111111111111111
fetch_program associated-token-account 1.1.5 ATAccPjxdgWfJKKN4PmfJ55FbEDEwD8zJUwVjuL9MuHy BPFLoader2111111111111111111111111111111111
fetch_program feature-proposal 1.0.1 FeatQtFWK7aZCvBCVURnhLaUvrm2m8tPN4jHhriettbc BPFLoader2111111111111111111111111111111111

echo "${genesis_args[@]}" > spl-genesis-args.sh

echo
echo "Available SPL programs:"
ls -l spl_*.so

echo
echo "miraland-genesis command-line arguments (spl-genesis-args.sh):"
cat spl-genesis-args.sh
