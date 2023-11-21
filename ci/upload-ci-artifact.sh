# |source| me

upload-ci-artifact() {
  echo "--- artifact: $1"
  if [[ -r "$1" ]]; then
    ls -l "$1"
    if ${BUILDKITE:-false}; then
      (
        set -x
        buildkite-agent artifact upload "$1"
      )
    fi
  else
    echo ^^^ +++
    echo "$1 not found"
  fi
}

upload-s3-artifact() {
  echo "--- artifact: $1 to $2"
  (
    args=(
      --rm
      --env AWS_ACCESS_KEY_ID
      --env AWS_SECRET_ACCESS_KEY
      --volume "$PWD:/miraland"

    )
    if [[ $(uname -m) = arm64 ]]; then
      # Ref: https://blog.jaimyn.dev/how-to-build-multi-architecture-docker-images-on-an-m1-mac/#tldr
      args+=(
        --platform linux/amd64
      )
    fi
    args+=(
      amazon/aws-cli:2.13.11
      s3 cp "$1" "$2" --acl public-read
    )
    set -x
    docker run "${args[@]}"
  )
}

# alternatively, upload with simple cli cmd w/o docker. MI
upload-s3-artifact-cmd() {
  echo "--- artifact: $1 to $2"

  # upon ```aws configure```
  # AWS_ACCESS_KEY_ID
  # AWS_SECRET_ACCESS_KEY
  # exists /usr/local/aws-cli and ~/.aws (contains ~/.aws/config and ~/.aws/credentials)
  # aws s3 cp /path/to/file s3://bucket_name/path/to/remote_file
  aws s3 cp "$1" "$2"
}

upload-oss-artifact() {
  echo "--- artifact: $1 to $2"

  # exists /usr/local/bin/ossutil and ~/.ossutilconfig
  # or
  # ./ossutil cp "$1" "$2" -e oss-cn-shanghai.aliyuncs.com -i $OSS_ACCESS_KEY_ID -k $OSS_ACCESS_KEY_SECRET
  ossutil cp "$1" "$2"
}