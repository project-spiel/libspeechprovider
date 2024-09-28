fs = import('fs')

generate_enums = fs.copyfile('generate_enums.py')
iface_xml = fs.copyfile('org.freedesktop.Speech.Provider.xml')
iface_xml_dest = install_data(iface_xml.full_path(), install_dir : get_option('datadir') / 'speech-provider')