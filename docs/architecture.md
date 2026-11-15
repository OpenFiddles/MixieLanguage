# MixieLand Rust architecture

- **MixieLand** is the complete language and development suite.
- **MixieLang** is the programming language and its language tooling.
- **Builgine** is the build orchestration, target configuration, external compiler integration, and packaging system.

The intended pipeline is `Source → Lexer → Parser → AST → Semantic Analysis → Shared IR → Backends`. The interpreter executes the AST today; the native compiler currently lowers the supported numeric subset to C and invokes the host C toolchain.

## Current language slice

- Numeric and string literals.
- Variables with `let name = expression;`.
- Arithmetic: `+`, `-`, `*`, `/`, and unary minus.
- Parenthesized expressions and normal arithmetic precedence.
- `print expression;` including string concatenation.
- Existing `pull { ... }` Builgine directives.

The REPL retains an environment between statements. File execution starts with a fresh environment.

## Compiler targets

The first compiler target is a native host executable. It requires Clang or GCC and produces a binary for the current operating system and CPU architecture.

A bare-metal target is deliberately separate. It needs a freestanding runtime, architecture-specific entry point, memory layout, linker script, boot protocol, and image packaging. Builgine will own those target profiles and boot-image generation rather than pretending a normal host executable is bootable.

Beg, Adv, and AdvN are modes of one language, not separate languages. The existing print prototype is retained as a migration smoke test while the newer syntax becomes canonical.
