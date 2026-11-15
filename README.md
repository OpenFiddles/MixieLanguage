# MixieLand

**MixieLang — simple, yet so elegant in power.**

MixieLand is the development suite. MixieLang is its programming language, and Builgine is the build and resource orchestration subsystem.

## Interactive interpreter

Build the CLI and start the REPL:

```sh
cargo build --release
./target/release/mixie repl
```

Try:

```mixie
let base = 6;
let answer = base * 7;
print answer;
print "Hello from MixieLand!";
```

The REPL keeps variables between entered statements. Commands: `:help`, `:vars`, `:reset`, and `:quit`. Each statement currently ends with a semicolon.

## Run a source file

```sh
cargo run --release -- -f examples/hello.mx
```

## Compile a native executable

Install a C toolchain (Clang or GCC), then:

```sh
cargo run --release -- compile examples/hello.mx -o hello
./hello
```

MixieLang currently emits a native host executable through the installed C compiler. This is **not yet a bare-metal kernel**. The freestanding target, boot entry point, linker script, architecture-specific startup, and bootable image packaging are a separate Builgine milestone; a host binary cannot simply be loaded directly by bare hardware.

## Setup

```sh
cargo run --release -- setup
```

The setup menu uses the standard `dialog` terminal utility when installed. It offers dependency checks, optional prerequisite installation, a release build, and a hello-world test. On Termux, install the menu utility first with `pkg install dialog`. If `dialog` is unavailable, MixieLang prints the platform-specific install hint and provides a basic text fallback.

Package installation is only started after you select it in the menu and confirm. The setup menu does not run automatically or change shell startup files.
