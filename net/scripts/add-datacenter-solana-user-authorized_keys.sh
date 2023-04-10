#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

# shellcheck source=net/scripts/miraland-user-authorized_keys.sh
source miraland-user-authorized_keys.sh

# miraland-user-authorized_keys.sh defines the public keys for users that should
# automatically be granted access to ALL datacenter nodes.
for i in "${!MIRALAND_USERS[@]}"; do
  echo "environment=\"MIRALAND_USER=${MIRALAND_USERS[i]}\" ${MIRALAND_PUBKEYS[i]}"
done

