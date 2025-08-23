<h1 align="center">
  <a href="https://catalyst-ide.dev" target="_blank">
  <img src="extra/images/logo.png" width=200 height=200/><br>
  Catalyst IDE
  </a>
</h1>

<h4 align="center">Lightning-fast AI-powered IDE with Claude integration</h4>

<div align="center">
  <a href="https://github.com/catalyst-ide/catalyst/actions/workflows/ci.yml" target="_blank">
    <img src="https://github.com/catalyst-ide/catalyst/actions/workflows/ci.yml/badge.svg" />
  </a>
  <a href="https://discord.gg/n8tGJ6Rn6D" target="_blank">
    <img src="https://img.shields.io/discord/946858761413328946?logo=discord" />
  </a>
  <a href="https://docs.catalyst-ide.dev" target="_blank">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.catalyst-ide.dev&color=blue" alt="Catalyst Docs">
  </a>
</div>
<br/>

> **🔥 Catalyst IDE is a fork of [Lapce](https://github.com/lapce/lapce)** - enhanced with Claude AI integration and 15+ pre-integrated MCP servers for supercharged development workflows.

Catalyst IDE is written in pure Rust, with a UI in [Floem](https://github.com/lapce/floem). It is designed with [Rope Science](https://xi-editor.io/docs/rope_science_00.html) from the [Xi-Editor](https://github.com/xi-editor/xi-editor), enabling lightning-fast computation, and leverages [wgpu](https://github.com/gfx-rs/wgpu) for rendering.

![](https://github.com/catalyst-ide/catalyst/blob/master/extra/images/screenshot.png?raw=true)

## 🚀 What Makes Catalyst Different

Catalyst extends the excellent Lapce foundation with AI-first development tools:

### 🤖 Claude AI Integration
- **Native Claude sidebar** - Chat with Claude directly in your IDE
- **Code-aware conversations** - Claude understands your current project context
- **Intelligent code assistance** - Generate, refactor, and debug code with AI help
- **Multi-language support** - Works seamlessly across all programming languages

### 🔌 15+ Pre-integrated MCP Servers
Model Context Protocol (MCP) servers provide Claude with powerful capabilities:
- **filesystem** - Secure file operations with permission management
- **git** - Local Git repository operations
- **github** - GitHub API integration for repos, issues, PRs
- **docker** - Container and image management
- **sentry** - Production error monitoring and debugging
- **socket** - Security analysis for dependencies
- **semgrep** - Static code analysis for vulnerabilities
- **jam** - Debug recordings with video and logs
- **puppeteer** & **playwright** - Browser automation for testing
- **postgresql** - Read-only database queries and schema inspection
- **mindsdb** - Unified interface to vector databases
- **google-drive** - File search and management
- **zapier** - Connect to 8,000+ applications
- **pipedream** - Access to thousands of APIs

### ⚡ Performance-First
- **< 500ms cold start** - Get coding faster
- **< 200ms warm start** - Near-instant project switching  
- **< 40MB idle memory** - Lightweight resource usage
- **< 5MB binary size** - Minimal disk footprint

## Features

* **🤖 AI-powered development** with native Claude integration
* **🔌 15+ MCP servers** providing Claude with powerful tools and APIs
* Built-in LSP ([Language Server Protocol](https://microsoft.github.io/language-server-protocol/)) support for intelligent code features
* Modal editing support as first class citizen (Vim-like, and toggleable)
* Built-in remote development support inspired by [VSCode Remote Development](https://code.visualstudio.com/docs/remote/remote-overview)
* Plugins can be written in programming languages that compile to [WASI](https://wasi.dev/) format (C, Rust, [AssemblyScript](https://www.assemblyscript.org/))
* Built-in terminal for workspace command execution

## Installation

You can find pre-built releases for Windows, Linux and macOS [here](https://github.com/catalyst-ide/catalyst/releases), or [installing with a package manager](docs/installing-with-package-manager.md).
If you'd like to compile from source, you can find the [guide](docs/building-from-source.md).

## Getting Started with Claude AI

1. **Get Claude API access** - Sign up at [claude.ai](https://claude.ai)
2. **Configure API key** - Add your Claude API key in Catalyst settings
3. **Open Claude sidebar** - Use `Ctrl+Shift+A` (or `Cmd+Shift+A` on macOS)
4. **Start coding with AI** - Ask Claude to help with your code!

## Contributing

Guidelines for contributing to Catalyst can be found in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Acknowledgments

Catalyst is built upon the excellent foundation provided by [Lapce](https://github.com/lapce/lapce). We're grateful to the Lapce team and community for creating such a performant and extensible code editor. 

Key differences from Lapce:
- Native Claude AI integration with sidebar chat
- 15+ pre-integrated MCP servers for enhanced AI capabilities  
- Performance optimizations for faster startup and lower memory usage
- Focus on AI-powered development workflows

## Feedback & Contact

The most popular place for Catalyst developers and users is on the [Discord server](https://discord.gg/n8tGJ6Rn6D).

Or, join the discussion on [Reddit](https://www.reddit.com/r/catalyst/) where we are just getting started.

## License

Catalyst is released under the Apache License Version 2, which is an open source license. You may contribute to this project, or use the code as you please as long as you adhere to its conditions. You can find a copy of the license text here: [`LICENSE`](LICENSE).