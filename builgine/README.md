# Builgine

Builgine is the build and resource orchestration subsystem of MixieLand.

## Pull backend

MixieLang parses resource declarations for Git source or prebuilt artifacts. Binary pulls require a SHA-256 digest.

The Rust API exposes:
- validate_pull(&PullSpec) — validates the declaration.
- execute_pull(&PullPlan, destination) — clones Git source or streams an HTTPS binary to disk and verifies SHA-256 before accepting it.
- build_source(&PullPlan, source_dir, confirm_trusted_source) — invokes the fixed make backend only after explicit trust confirmation.

Source retrieval requires Git. Building requires Make and the source tree's build dependencies. A Makefile is executable code; only build source you trust. The interpreter does not run builds automatically when it reads a pull statement. Binary artifacts are not executed by the pull backend.

The Linux kernel workflow is a retrieval/build foundation, not a turnkey kernel configuration. A real kernel build still needs an architecture, kernel configuration, toolchain, host dependencies, and a deliberate output/install target.
