gir = find_program('gir')
fs = import('fs')
python_module = import('python')
python = python_module.find_installation('python3')

gir_dirs = ['-d', fs.parent(speech_provider_gir[0].full_path())]
res = run_command(python, '-c', 'import os; print(os.environ.get("XDG_DATA_DIRS"))', check: true)
xdg_data_paths = res.stdout().strip().split(':')
foreach xdg_data_path : xdg_data_paths
  gir_dirs += ['-d', xdg_data_path / 'gir-1.0']
endforeach

subdir('libspeechprovider-sys')

subdir('src')

fs.copyfile('Cargo.toml')
gir_toml = fs.copyfile('Gir.toml')

custom_target('libspeechprovider-rs',
  input: gir_toml,
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