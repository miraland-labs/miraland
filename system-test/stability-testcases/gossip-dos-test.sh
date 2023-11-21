#!/usr/bin/env bash

set -e
cd "$(dirname "$0")"
MIRALAND_ROOT="$(cd ../..; pwd)"

logDir="$PWD"/logs
rm -rf "$logDir"
mkdir "$logDir"

solanaInstallDataDir=$PWD/releases
solanaInstallGlobalOpts=(
  --data-dir "$solanaInstallDataDir"
  --config "$solanaInstallDataDir"/config.yml
  --no-modify-path
)

# Install all the solana versions
bootstrapInstall() {
  declare v=$1
  if [[ ! -h $solanaInstallDataDir/active_release ]]; then
    sh "$MIRALAND_ROOT"/install/miraland-install-init.sh "$v" "${solanaInstallGlobalOpts[@]}"
  fi
  export PATH="$solanaInstallDataDir/active_release/bin/:$PATH"
}

bootstrapInstall "edge"
miraland-install-init --version
miraland-install-init edge
miraland-gossip --version
miraland-dos --version

killall miraland-gossip || true
miraland-gossip spy --gossip-port 8001 > "$logDir"/gossip.log 2>&1 &
solanaGossipPid=$!
echo "miraland-gossip pid: $solanaGossipPid"
sleep 5
miraland-dos --mode gossip --data-type random --data-size 1232 &
dosPid=$!
echo "miraland-dos pid: $dosPid"

pass=true

SECONDS=
while ((SECONDS < 600)); do
  if ! kill -0 $solanaGossipPid; then
    echo "miraland-gossip is no longer running after $SECONDS seconds"
    pass=false
    break
  fi
  if ! kill -0 $dosPid; then
    echo "miraland-dos is no longer running after $SECONDS seconds"
    pass=false
    break
  fi
  sleep 1
done

kill $solanaGossipPid || true
kill $dosPid || true
wait || true

$pass && echo Pass
