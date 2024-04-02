custom_target('libspeechprovider-sys',
  input: 'Gir.toml',
  output: '.',
  command: [
    gir,
    '-c',
    '@INPUT@',
    '-o',
    '@OUTPUT@'
  ] + gir_dirs,
  depends: [speech_provider_gir],
  build_by_default: true,
  install: false,
)