# -*- mode: ruby -*-

# Force use of the Virtualbox provider.
# TODO: Support using other providers.
ENV['VAGRANT_DEFAULT_PROVIDER'] = 'virtualbox'

Vagrant.configure('2') do |config|
  config.vm.guest = :freebsd
  config.vm.network('private_network', ip: '10.0.1.10')

  # Setup the synced folder to use Rsync.
  # TODO: Get NFS working.
  config.vm.synced_folder(
    '.', '/vagrant',
    type: 'rsync',
    id: 'vagrant-root',
    rysnc__auto: true
  )

  config.vm.provider :virtualbox do |vb, override|
    override.vm.box_url = 'https://wunki.org/files/freebsd-10.2-amd64-wunki.box'
    override.vm.box = 'freebsd-10.2-amd64-wunki'

    # vb.customize(["startvm", :id, "--type", "gui"])
    vb.customize(['modifyvm', :id, '--memory', '512'])
    vb.customize(['modifyvm', :id, '--cpus', '2'])
    vb.customize(['modifyvm', :id, '--hwvirtex', 'on'])
    vb.customize(['modifyvm', :id, '--audio', 'none'])
    vb.customize(['modifyvm', :id, '--nictype1', 'virtio'])
    vb.customize(['modifyvm', :id, '--nictype2', 'virtio'])
  end
end
