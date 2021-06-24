#!/usr/bin/env bash

echo ">>> Setting Timezone & Locale to $3 & en_US.UTF-8"
sudo ln -sf /usr/share/zoneinfo/$3 /etc/localtime
sudo apt-get install -qq language-pack-en
sudo locale-gen en_US
sudo update-locale LANG=en_US.UTF-8 LC_CTYPE=en_US.UTF-8

echo ">>> Installing Base Packages"
# if [[ -z $1 ]]; then
#   github_url="https://raw.githubusercontent.com/fideloper/Vaprobash/master"
# else
#   github_url="$1"
# fi

sudo apt-get update

sudo apt-get install -qq curl unzip git-core ack-grep software-properties-common build-essential cachefilesd

# Setup swap
# Disable case sensitivity
shopt -s nocasematch

if [[ -n $2 && ! $2 =~ false && $2 =~ ^[0-9]*$ ]]; then
  echo ">>> Setting up swap ($2 MB)"

  # Create the swap file
  fallocate -l "$2M" /swapfile

  # Correct swap permissions
  chmod 600 /swapfile

  # Setup swap space
  mkswap /swapfile

  # Enable swap space
  swapon /swapfile

  # Make swap file permanent
  echo "/swapfile   none    swap    sw    0   0" | tee -a /etc/fstab

  # Add some swap settings:
  # vm.swappiness=10: Only activate swap when the amount of RAM left is 10% or less.
  # vm.vfs_cache_pressure=50: http://rudd-o.com/linux-and-free-software/tales-from-responsivenessland-why-linux-feels-slow-and-how-to-fix-that
  printf "vm.swappiness=10\nvm.vfs_cache_pressure=50" | tee -a /etc/sysctl.conf && sysctl -p
fi

# Enable case sensitivity
shopt -u nocasematch

# Enable cachefilesd
echo "RUN=yes" > /etc/default/cachefilesd
