# Builgine

Builgine is the build orchestration and artifact-management subsystem planned for MixieLand.

Initial responsibilities:
- Resolve and validate MixieLang pull declarations.
- Distinguish source repositories from prebuilt binary artifacts.
- Require SHA-256 checksums for prebuilt downloads.
- Record source revisions and requested build systems.
- Later fetch pinned source, stage artifacts, invoke supported build backends, and package outputs.

Current status: the Rust code provides a validated pull-plan API only. It does not yet execute remote build scripts or install binaries. Those operations need explicit trust, sandboxing, platform-compatibility, and artifact-verification controls. The first planned backend is Linux kernel source via Make, plus verified prebuilt kernel images.
