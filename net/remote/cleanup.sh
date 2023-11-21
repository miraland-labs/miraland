#!/usr/bin/env bash

set -x
! tmux list-sessions || tmux kill-session
declare sudo=
if sudo true; then
  sudo="sudo -n"
fi

echo "pwd: $(pwd)"
for pid in miraland/*.pid; do
  pgid=$(ps opgid= "$(cat "$pid")" | tr -d '[:space:]')
  if [[ -n $pgid ]]; then
    $sudo kill -- -"$pgid"
  fi
done
if [[ -f miraland/netem.cfg ]]; then
  miraland/scripts/netem.sh delete < miraland/netem.cfg
  rm -f miraland/netem.cfg
fi
miraland/scripts/net-shaper.sh cleanup
for pattern in validator.sh boostrap-leader.sh miraland- remote- iftop validator client node; do
  echo "killing $pattern"
  pkill -f $pattern
done
