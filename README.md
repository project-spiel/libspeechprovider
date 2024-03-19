<img alt="libspiel logo" align="right" src="https://raw.githubusercontent.com/project-spiel/libspeechprovider/main/spiel-logo.svg">

# Speech Provider Resources

[![ Build & Test ](https://github.com/project-spiel/libspeechprovider/actions/workflows/ci.yml/badge.svg)](https://github.com/project-spiel/libspeechprovider/actions/workflows/ci.yml) [![ Docs & Website ](https://github.com/project-spiel/libspeechprovider/actions/workflows/website.yml/badge.svg)](https://github.com/project-spiel/libspeechprovider/actions/workflows/website.yml)

## Overview

This project includes the resources needed for building a Spiel speech provider.

### org.freedesktop.Speech.Provider
This is a [loose specification](https://project-spiel.github.io/libspeechprovider/generated-org.freedesktop.Speech.Provider.html) of what methods and properties a D-Bus speech provider should implement in order to be discoverable and usable by client applications. It consists of simple speech methods such as `Synthesize()`, and a `Voices` property.

The interface for a typical speech provider implementation could be written in a couple hundred lines of code. The specification is designed to be simple and straightforward. The really neat thing about this kind of speech provider is that it can be distributed as a Flatpak or Snap without any system prerequisites and be completely self contained.

### Provider Library (libspeechprovider)
The [speech provider library](https://project-spiel.github.io/libspeechprovider/) is designed to provide some utility for creating speech providers. Specifically it offers a stream writer that can be used to send audio data interleaved with speech progress events (word, sentance, ssml mark, etc.)

Language bindings are available through GObject Introspection. So this should work for any application, be it in C/C++, Python, Rust, ECMAscript, or Lua.

## Building

We use the Meson build system for building this project.

```sh
# Some common options
meson setup build
meson compile -C build
```

## Documentation

There is [auto-generated API reference](https://project-spiel.github.io/libspeechprovider/) along with a [documented D-Bus interface](https://project-spiel.github.io/libspeechprovider/generated-org.freedesktop.Speech.Provider.html) for speech providers.