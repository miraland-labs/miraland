#!/usr/bin/env bash

here=$(dirname "$0")
# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

set -e

rm -rf "$MIRALAND_CONFIG_DIR"/latest-mainnet-snapshot
mkdir -p "$MIRALAND_CONFIG_DIR"/latest-mainnet-snapshot
(
  cd "$MIRALAND_CONFIG_DIR"/latest-mainnet-snapshot || exit 1
  set -x
  wget http://api.mainnet.miraland.top/genesis.tar.bz2
  wget --trust-server-names http://api.mainnet.miraland.top/snapshot.tar.bz2
)

snapshot=$(ls "$MIRALAND_CONFIG_DIR"/latest-mainnet-snapshot/snapshot-[0-9]*-*.tar.zst)
if [[ -z $snapshot ]]; then
  echo Error: Unable to find latest snapshot
  exit 1
fi

if [[ ! $snapshot =~ snapshot-([0-9]*)-.*.tar.zst ]]; then
  echo Error: Unable to determine snapshot slot for "$snapshot"
  exit 1
fi

snapshot_slot="${BASH_REMATCH[1]}"

rm -rf "$MIRALAND_CONFIG_DIR"/bootstrap-validator
mkdir -p "$MIRALAND_CONFIG_DIR"/bootstrap-validator


# Create genesis ledger
if [[ -r $FAUCET_KEYPAIR ]]; then
  cp -f "$FAUCET_KEYPAIR" "$MIRALAND_CONFIG_DIR"/faucet.json
else
  $miraland_keygen new --no-passphrase -fso "$MIRALAND_CONFIG_DIR"/faucet.json
fi

if [[ -f $BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR ]]; then
  cp -f "$BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR" "$MIRALAND_CONFIG_DIR"/bootstrap-validator/identity.json
else
  $miraland_keygen new --no-passphrase -so "$MIRALAND_CONFIG_DIR"/bootstrap-validator/identity.json
fi

$miraland_keygen new --no-passphrase -so "$MIRALAND_CONFIG_DIR"/bootstrap-validator/vote-account.json
$miraland_keygen new --no-passphrase -so "$MIRALAND_CONFIG_DIR"/bootstrap-validator/stake-account.json

$miraland_ledger_tool create-snapshot \
  --ledger "$MIRALAND_CONFIG_DIR"/latest-mainnet-snapshot \
  --faucet-pubkey "$MIRALAND_CONFIG_DIR"/faucet.json \
  --faucet-lamports 500000000000000000 \
  --bootstrap-validator "$MIRALAND_CONFIG_DIR"/bootstrap-validator/identity.json \
                        "$MIRALAND_CONFIG_DIR"/bootstrap-validator/vote-account.json \
                        "$MIRALAND_CONFIG_DIR"/bootstrap-validator/stake-account.json \
  --hashes-per-tick sleep \
  "$snapshot_slot" "$MIRALAND_CONFIG_DIR"/bootstrap-validator

$miraland_ledger_tool modify-genesis \
  --ledger "$MIRALAND_CONFIG_DIR"/latest-mainnet-snapshot \
  --hashes-per-tick sleep \
  "$MIRALAND_CONFIG_DIR"/bootstrap-validator
