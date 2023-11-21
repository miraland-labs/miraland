#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

# check does it need to publish
if [[ -n $DO_NOT_PUBLISH_TAR ]]; then
  echo "Skipping publishing install wrapper"
  exit 0
fi

# check channel and tag
eval "$(ci/channel-info.sh)"

if [[ -n "$CI_TAG" ]]; then
  CHANNEL_OR_TAG=$CI_TAG
else
  CHANNEL_OR_TAG=$CHANNEL
fi

if [[ -z $CHANNEL_OR_TAG ]]; then
  echo +++ Unable to determine channel or tag to publish into, exiting.
  exit 0
fi

# upload install script
source ci/upload-ci-artifact.sh

cat >release.miraland.top-install <<EOF
MIRALAND_RELEASE=$CHANNEL_OR_TAG
MIRALAND_INSTALL_INIT_ARGS=$CHANNEL_OR_TAG
MIRALAND_DOWNLOAD_ROOT=https://release.miraland.top
EOF
cat install/miraland-install-init.sh >>release.miraland.top-install

echo --- AWS S3 Store: "install"
# upload-s3-artifact "/miraland/release.miraland.top-install" "s3://release.miraland.top/$CHANNEL_OR_TAG/install"
# temp using rc1.miraland.top
upload-s3-artifact-cmd "release.miraland.top-install" "s3://rc1.miraland.top/$CHANNEL_OR_TAG/install"
echo Published to:
ci/format-url.sh https://rc1.miraland.top/"$CHANNEL_OR_TAG"/install

echo --- Aliyun OSS Store: "install"
# upload-oss-artifact "/miraland/release.miraland.top-install" "oss://release-miraland-top/$CHANNEL_OR_TAG/install"
upload-oss-artifact "release.miraland.top-install" "oss://release-miraland-top/$CHANNEL_OR_TAG/install"
echo Published to:
ci/format-url.sh https://release.miraland.top/"$CHANNEL_OR_TAG"/install
