# MixieLand

**MixieLang — simple, yet so elegant in power.**

MixieLand is the development suite. MixieLang is its programming language, and Builgine is the build and resource orchestration subsystem.

## Quick setup

Build the CLI, then launch the interactive setup menu:

```sh
cargo build --release
./target/release/mixie setup
```

The setup menu uses the standard `dialog` terminal utility when installed. It offers dependency checks, optional prerequisite installation, a release build, and a hello-world test. On Termux, install the menu utility first with `pkg install dialog`. If `dialog` is unavailable, MixieLang prints the platform-specific install hint and provides a basic text fallback.

Detected environments currently include Termux, Debian/Ubuntu, Fedora, Arch, Alpine, openSUSE, and macOS with Homebrew. Package installation is only started after you select it in the menu and confirm. Windows and unknown environments show manual setup guidance.

## Requirements

- Rust stable toolchain (Cargo included)
- Git for source pulls
- Make for source builds
- `dialog` for the full terminal setup interface

## Build and run

```sh
cargo build --release
cargo run --release -- -f examples/hello.mx
cargo run --release -- setup
```

The setup menu does not run automatically, change shell startup files, or install packages without confirmation.
