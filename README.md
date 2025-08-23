# Catalyst IDE

<h4 align="center">Lightning-fast code editor written in pure Rust</h4>

<div align="center">
  <a href="https://github.com/codeyousef/catalyst/actions/workflows/ci.yml" target="_blank">
    <img src="https://github.com/codeyousef/catalyst/actions/workflows/ci.yml/badge.svg" />
  </a>
</div>
<br/>

> **Catalyst is a fork of [Lapce](https://github.com/lapce/lapce)** - enhanced with performance improvements and an extensible plugin architecture for modern development workflows.

Catalyst IDE is written in pure Rust, with a UI in [Floem](https://github.com/lapce/floem). It is designed with [Rope Science](https://xi-editor.io/docs/rope_science_00.html) from the [Xi-Editor](https://github.com/xi-editor/xi-editor), enabling lightning-fast computation, and leverages [wgpu](https://github.com/gfx-rs/wgpu) for rendering.

![](https://github.com/catalyst-ide/catalyst/blob/master/extra/images/screenshot.png?raw=true)

## 🚀 What Makes Catalyst Different

Catalyst builds upon the excellent Lapce foundation with focused improvements:

### ⚡ Performance Optimizations
- **Faster startup times** - Optimized initialization sequence
- **Reduced memory usage** - Efficient resource management
- **Smaller binary size** - Streamlined build process
- **Improved responsiveness** - Enhanced UI performance

### 🔌 Extensible Architecture
- **Enhanced plugin system** - Support for external integrations
- **Flexible assistant interface** - Generic AI/tool integration capability
- **Modular design** - Clean separation of concerns
- **Future-ready** - Built for extensibility and growth

## Features

* **Built-in LSP ([Language Server Protocol](https://microsoft.github.io/language-server-protocol/)) support** for intelligent code features
* **Modal editing support** as first class citizen (Vim-like, and toggleable)
* **Built-in remote development** support inspired by [VSCode Remote Development](https://code.visualstudio.com/docs/remote/remote-overview)
* **WASI Plugin system** - Plugins can be written in programming languages that compile to [WASI](https://wasi.dev/) format (C, Rust, [AssemblyScript](https://www.assemblyscript.org/))
* **Built-in terminal** for workspace command execution
* **Extensible architecture** - Support for external integrations and assistant providers

## Installation

You can find pre-built releases for Windows, Linux and macOS [here](https://github.com/codeyousef/catalyst/releases), or [installing with a package manager](docs/installing-with-package-manager.md).
If you'd like to compile from source, you can find the [guide](docs/building-from-source.md).

## Contributing

Guidelines for contributing to Catalyst can be found in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Acknowledgments

Catalyst is built upon the excellent foundation provided by [Lapce](https://github.com/lapce/lapce). We're grateful to the Lapce team and community for creating such a performant and extensible code editor. 

Key improvements from Lapce:
- Performance optimizations for faster startup and lower memory usage
- Enhanced plugin architecture with extensible assistant interfaces
- Streamlined build process and resource usage
- Improved modular design for better maintainability

## License

Catalyst is released under the Apache License Version 2, which is an open source license. You may contribute to this project, or use the code as you please as long as you adhere to its conditions. You can find a copy of the license text here: [`LICENSE`](LICENSE).