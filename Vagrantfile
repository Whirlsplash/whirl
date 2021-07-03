# -*- mode: ruby -*-
# vi: set ft=ruby :

# https://github.com/fideloper/Vaprobash

# Github Configuration
github_username   = "Whirlsplash"
github_repository = "whirl"
github_branch     = "develop"
github_url        = "https://raw.githubusercontent.com/#{github_username}/#{github_repository}/#{github_branch}"

# Virtual-Machine Configuration
hostname = "whirlsplash"

# Local private network IP (http://en.wikipedia.org/wiki/Private_network)
#
# Valid IP ranges:
#   10.0.0.1    -  10.255.255.254
#   172.16.0.1  -  172.31.255.254
#   192.168.0.1 - 192.168.255.254
server_ip = "192.168.22.10"
server_cpus = "1"     # Cores
server_memory = "384" # MB
server_swap = "768"   # MB | false
                      #
                      # Guideline: Keep between one or two times the value of
                      # `server_memory`.

# UTC        - Universal Coordinated Time
# EST        -      Eastern Standard Time
# CET        -      Central European Time
# US/Central -      American Central Time
# US/Eastern -      American Eastern Time
server_timezone  = "UTC"

# https://docs.vagrantup.com.
Vagrant.configure("2") do |config|
  # Operating system
  config.vm.box = "debian/buster64"

  if Vagrant.has_plugin?("vagrant-hostmanager")
    config.hostmanager.enabled = true
    config.hostmanager.manage_host = true
    config.hostmanager.ignore_private_ip = false
    config.hostmanager.include_offline = false
  end

  # Hostname, points to the VM's default vhost.
  #
  # Don't forget to include this in the `hosts` file!
  config.vm.hostname = hostname

  # Static IP
  if Vagrant.has_plugin?("vagrant-auto_network")
    config.vm.network :private_network, :ip => "0.0.0.0", :auto_network => true
  else
    config.vm.network :private_network, ip: server_ip

    config.vm.network :forwarded_port, guest: 80,   host: 8080 # API
    config.vm.network :forwarded_port, guest: 6650, host: 6650 # Distributor
    config.vm.network :forwarded_port, guest: 5673, host: 5673 # Hub
  end

  # Forward agent over SSH
  config.ssh.forward_agent = true

  # Disable rsync-ing the current directory; it takes too long and the
  # necessary files are rsync'd in the following command, anyway.
  # https://github.com/devopsgroup-io/vagrant-digitalocean/issues/255#issuecomment-352228157
  config.vm.synced_folder ".", "/vagrant", disabled: true
  # https://stackoverflow.com/a/44352166/14452787
  config.vm.synced_folder ".", "/home/vagrant/whirl",
                          type: "rsync",
                          rsync__auto: true,
                          rsync__exclude: %w[target/ node_modules/ result]

  # VirtualBox Configuration
  config.vm.provider :virtualbox do |vb|
    vb.name = hostname # Hostname

    # CPU core count
    vb.customize ["modifyvm", :id, "--cpus", server_cpus]

    # RAM amount
    vb.customize ["modifyvm", :id, "--memory", server_memory]

    # Set the Time-Sync threshold to ten seconds instead of the default twenty
    # minutes:
    #   if the clock gets more than fifteen minutes out of sync, -- due to your
    #   laptop going to sleep, for instance -- then some third-party services
    #   will reject requests.
    vb.customize ["guestproperty",
                  "set",
                  :id,
                  "/VirtualBox/GuestAdd/VBoxService/--timesync-set-threshold",
                  10000]

    # Prevent VMs running on Ubuntu from losing internet connection
    # vb.customize ["modifyvm", :id, "--natdnshostresolver1", "on"]
    # vb.customize ["modifyvm", :id, "--natdnsproxy1", "on"]
  end

  # VMWare Fusion Configuration
  config.vm.provider "vmware_fusion" do |vb, _override|
    vb.vmx["memsize"] = server_memory # RAM amount
  end

  # Provisioning
  # Base packages
  config.vm.provision "shell",
                      path: "./bin/base.sh", # #{github_url}
                      args: [github_url, server_swap, server_timezone]

  # Optimize base box
  config.vm.provision "shell",
                      path: "./bin/base_box_optimizations.sh",
                      privileged: true

  # Provision SQLite
  config.vm.provision "shell", path: "./bin/sqlite.sh"

  # Provision Rust
  config.vm.provision "shell", path: "./bin/rust.sh"
end
