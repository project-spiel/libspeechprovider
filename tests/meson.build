service_dir = join_paths(meson.current_build_dir(), 'services')

test_env = environment()
test_env.prepend('GI_TYPELIB_PATH',
  join_paths(meson.project_build_root(), 'libspeechprovider'), separator: ':')
test_env.prepend('LD_LIBRARY_PATH',
  join_paths(meson.project_build_root(), 'libspeechprovider'), separator: ':')
test_env.set('G_DEBUG', 'fatal-warnings')

python_module = import('python')
python = python_module.find_installation(
  'python3', required : false, modules: ['tap'])

tests = ['test_provider_lib.py']

test_deps = speech_provider_deps + [
  speech_provider_lib_dep,
]

if python.found() and get_option('introspection')
  foreach test_name : tests
    test(
      test_name, python,
      args : [
        files(test_name)
      ],
      env : test_env,
      protocol : 'tap'
    )
  endforeach
endif
