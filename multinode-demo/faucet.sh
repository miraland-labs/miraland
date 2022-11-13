#!/usr/bin/env bash
#
# Starts an instance of solana-faucet
#
here=$(dirname "$0")

# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

[[ -f "$MIRALAND_CONFIG_DIR"/faucet.json ]] || {
  echo "$MIRALAND_CONFIG_DIR/faucet.json not found, create it by running:"
  echo
  echo "  ${here}/setup.sh"
  exit 1
}

set -x
# shellcheck disable=SC2086 # Don't want to double quote $miraland_faucet
exec $miraland_faucet --keypair "$MIRALAND_CONFIG_DIR"/faucet.json "$@"
