# populate this on the stable branch
solartiTokenCliVersion=

maybeSolartiTokenCliVersionArg=
if [[ -n "$solartiTokenCliVersion" ]]; then
    # shellcheck disable=SC2034
    maybeSolartiTokenCliVersionArg="--version $solartiTokenCliVersion"
fi
