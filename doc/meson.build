toml_conf = configuration_data()
toml_conf.set('version', meson.project_version())
toml_conf.set('license', meson.project_license())

source_toml = configure_file(
  input: 'SpeechProvider.toml.in',
  output: 'SpeechProvider.toml',
  configuration: toml_conf,
)

interface_src_dir = join_paths(meson.project_source_root(), 'interface')
logo = join_paths(meson.project_source_root(), 'spiel-logo.svg')

fs = import('fs')
urlmap_doc = fs.copyfile('urlmap.js')
logo_file = fs.copyfile(logo)

gidocgen = find_program('gi-docgen', required: true)

gidocgen_common_args = [
  '--quiet',
  '--no-namespace-dir',
]

if get_option('werror')
  gidocgen_common_args += ['--fatal-warnings']
endif

python_module = import('python')
python = python_module.find_installation(
  'python3', required : true)

docs = custom_target('libspeechprovider-doc',
  input: [ source_toml, speech_provider_gir[0] ],
  output: 'libspeechprovider',
  command: [
    gidocgen,
    'generate',
    gidocgen_common_args,
    '--config=@INPUT0@',
    '--output-dir=@OUTPUT@',
    '--content-dir=@0@'.format(meson.current_build_dir()),
    '@INPUT1@',
  ],
  build_by_default: true,
  depends: [ urlmap_doc, logo_file ]
  # install: true,
  # install_dir: docs_dir,
)

gdbus_codegen_cmd = find_program('gdbus-codegen')
spiel_iface_docs = custom_target('dbus-docs',
  output : 'generated-org.freedesktop.Speech.Provider.md',
  input : join_paths(interface_src_dir, 'org.freedesktop.Speech.Provider.xml'),
  build_by_default: true,
  depends: docs,
  command : [gdbus_codegen_cmd, '--generate-md', 'generated',
             '--output-directory', meson.current_build_dir() / 'libspeechprovider',
             join_paths(interface_src_dir, 'org.freedesktop.Speech.Provider.xml')])
