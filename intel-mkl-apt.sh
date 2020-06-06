#!/bin/bash
set -eux

apt update
apt install -y curl gnupg pkg-config

curl -sSf https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS-2019.PUB | apt-key add -
curl -sSf https://apt.repos.intel.com/setup/intelproducts.list -o /etc/apt/sources.list.d/intelproducts.list
echo deb https://apt.repos.intel.com/mkl all main > /etc/apt/sources.list.d/intel-mkl.list

apt update
apt install -y intel-mkl-2020.1-102
