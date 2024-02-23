# source this file

update_miraland_dependencies() {
  declare project_root="$1"
  declare miraland_ver="$2"
  declare tomls=()
  while IFS='' read -r line; do tomls+=("$line"); done < <(find "$project_root" -name Cargo.toml)

  sed -i -e "s#\(miraland-program = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-program = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-program-test = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-program-test = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-sdk = \"\).*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-sdk = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-client = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-client = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-cli-config = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-cli-config = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-clap-utils = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-clap-utils = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-account-decoder = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-account-decoder = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-faucet = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-faucet = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-zk-token-sdk = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
  sed -i -e "s#\(miraland-zk-token-sdk = { version = \"\)[^\"]*\(\"\)#\1=$miraland_ver\2#g" "${tomls[@]}" || return $?
}

patch_crates_io_miraland() {
  declare Cargo_toml="$1"
  declare miraland_dir="$2"
  cat >> "$Cargo_toml" <<EOF
[patch.crates-io]
EOF
patch_crates_io_miraland_no_header "$Cargo_toml" "$miraland_dir"
}

patch_crates_io_miraland_no_header() {
  declare Cargo_toml="$1"
  declare miraland_dir="$2"
  cat >> "$Cargo_toml" <<EOF
miraland-account-decoder = { path = "$miraland_dir/account-decoder" }
miraland-clap-utils = { path = "$miraland_dir/clap-utils" }
miraland-client = { path = "$miraland_dir/client" }
miraland-cli-config = { path = "$miraland_dir/cli-config" }
miraland-program = { path = "$miraland_dir/sdk/program" }
miraland-program-test = { path = "$miraland_dir/program-test" }
miraland-sdk = { path = "$miraland_dir/sdk" }
miraland-faucet = { path = "$miraland_dir/faucet" }
miraland-zk-token-sdk = { path = "$miraland_dir/zk-token-sdk" }
EOF
}
