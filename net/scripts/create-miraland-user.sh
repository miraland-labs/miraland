#!/usr/bin/env bash
set -ex

[[ $(uname) = Linux ]] || exit 1
[[ $USER = root ]] || exit 1

if grep -q miraland /etc/passwd ; then
  echo "User miraland already exists"
else
  adduser miraland --gecos "" --disabled-password --quiet
  adduser miraland sudo
  adduser miraland adm
  echo "miraland ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
  id miraland

  [[ -r /miraland-scratch/id_ecdsa ]] || exit 1
  [[ -r /miraland-scratch/id_ecdsa.pub ]] || exit 1

  sudo -u miraland bash -c "
    echo 'PATH=\"/home/miraland/.cargo/bin:$PATH\"' > /home/miraland/.profile
    mkdir -p /home/miraland/.ssh/
    cd /home/miraland/.ssh/
    cp /miraland-scratch/id_ecdsa.pub authorized_keys
    umask 377
    cp /miraland-scratch/id_ecdsa id_ecdsa
    echo \"
      Host *
      BatchMode yes
      IdentityFile ~/.ssh/id_ecdsa
      StrictHostKeyChecking no
    \" > config
  "
fi
