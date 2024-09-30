fs = import('fs')

generate_enums = fs.copyfile('generate_enums.py')
iface_xml = fs.copyfile('org.freedesktop.Speech.Provider.xml')
install_data('org.freedesktop.Speech.Provider.xml', install_dir : get_option('datadir') / 'speech-provider')