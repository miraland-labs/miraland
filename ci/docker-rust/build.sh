#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"


platform=()
if [[ $(uname -m) = arm64 ]]; then
  # Ref: https://blog.jaimyn.dev/how-to-build-multi-architecture-docker-images-on-an-m1-mac/#tldr
  platform+=(--platform linux/amd64)
fi

docker build "${platform[@]}" -t miralandlabs/rust .

read -r rustc version _ < <(docker run miralandlabs/rust rustc --version)
[[ $rustc = rustc ]]
docker tag miralandlabs/rust:latest miralandlabs/rust:"$version"
docker push miralandlabs/rust:"$version"
docker push miralandlabs/rust:latest
