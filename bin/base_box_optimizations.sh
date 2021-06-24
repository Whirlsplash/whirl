#!/bin/bash

# Exit the script if the executor is not root
if [[ $EUID -ne 0 ]]; then
  cat <<END
you need to run this script as root
use :privileged => true in Vagrantfile
END

  exit 0
fi

# Optimize APT sources for fastest mirror
perl -pi -e 's@^\s*(deb(\-src)?)\s+http://us.archive.*?\s+@\1 mirror://mirrors.ubuntu.com/mirrors.txt @g' /etc/apt/sources.list

# Update repositories
apt-get update
