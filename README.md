# SigmaOS-workspace

Cargo workspace consolidation for SigmaOS core components.

The root `Cargo.toml` unifies independently maintained component
repositories (one repo per component) into a single build graph.
`crates/` contains symlinks to the canonical component repos in `~`.

## Members

Kernel layer: `kernel`, `vfs`, `scheduler`, `driver_framework`
System layer: `init`, `boot_manager`, `posix`, `sys`, `memory_manager`
Userland layer: `coreutils`, `initramfs`

`crates/interrupt` and `crates/allocators` are excluded: their manifests
declare sibling path deps that cannot resolve through symlinks; they
remain buildable standalone.

## Usage

```sh
cargo metadata --no-deps   # resolve the member graph
cargo check --workspace    # typecheck all core members
cargo test -p sigmaos_vfs  # test one member
```

`src/lib.rs` exposes `CORE_MEMBERS` and layer classification for tooling.
