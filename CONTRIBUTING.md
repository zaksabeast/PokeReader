# Contributing to PokeReader

Thanks for taking the time to contribute. This document covers the build
environment, coding rules and the pull request process.

## Before you start

- PokeReader is a 3gx plugin for the Nintendo 3DS. It runs inside the game
  process and is distributed under the GNU General Public License v3. See
  [LICENSE](LICENSE).
- Open an issue first for large changes, especially changes that touch game
  memory addresses or RNG behaviour.
- Never include copyrighted game files or ROMs in issues, pull requests or
  releases.

## Prerequisites

- [devkitPro](https://devkitpro.org/wiki/Getting_Started) with devkitARM
  (`DEVKITARM` must be set).
- [3gxtool](https://gitlab.com/thepixellizeross/3gxtool).
- Rust with the pinned toolchain from
  [`reader_core/rust-toolchain.toml`](reader_core/rust-toolchain.toml) and the
  `armv6k-nintendo-3ds` target.
- `make` and a POSIX-like shell (Linux, macOS, MSYS2 or WSL on Windows).

The CI container `zaksabeast/devkitarm-rust` contains everything needed.

## Building and testing

```sh
make         # builds out/default.3gx
make test    # host unit tests
make lint    # clippy for the 3DS target
make format  # rustfmt
```

`make test` runs the host test suite with the `test_stubs` feature, which
replaces the 3DS host functions with stubs. Runtime-only behaviour (framebuffer,
HID shared memory) is not covered by those tests; validate it in Citra or on
hardware when changing it.

If you use `cargo` directly from `reader_core/`:

```sh
cargo test --features test_stubs
```

## Code layout

- `3gx/` — C plugin entry point, ARM hook and the bridge to the game
  (`main.c`, `pnp.c`, `hid.c`).
- `reader_core/src/` — Rust core (`no_std`): memory readers, RNG, overlay
  drawing, menus and the input system.
- `reader_core/src/input/` — configuration, bindings, dispatcher and the
  Controls menu.

The C layer owns the pause loop (`handle_freeze` in `3gx/sources/main.c`) and
asks Rust whether to pause, step or resume. Keep platform access in `pnp/` and
keep pure logic testable on the host.

## Style

- Run `make format` before opening a pull request.
- Run `make lint`; new warnings in files you touched should be fixed.
- Follow the existing naming and module style. Prefer small, focused functions
  and explicit error handling over panics.
- Do not add `unwrap()` in code paths that can run on the console unless the
  value is a compile-time constant.
- Device input must go through the `input` module; do not add new hardcoded
  `is_just_pressed` checks for user actions.

## Commit messages

Use short, imperative subjects, optionally with a scope:

```
input: add configurable bindings
input: persist configuration to the SD card
docs: document input isolation limits
```

Keep commits focused. Do not mix formatting-only changes with behaviour changes.

## Pull requests

1. Fork the repository and create a feature branch from `main`.
2. Make sure `make test`, `make lint` and `make build` pass.
3. Describe:
   - the problem,
   - the solution,
   - the tests you ran,
   - what you validated in Citra or on hardware,
   - anything that is still unverified.
4. Do not merge your own pull request.
5. Do not force-push a branch under review.

For changes that affect RNG calculations or memory addresses, include the game
and version used to verify the change. Behaviour must remain identical to the
previous implementation unless the change is intentional and documented.

## Reporting issues

Include:

- PokeReader version (shown in the overlay),
- game and update version,
- Luma3DS or Citra version,
- the exact steps to reproduce,
- the contents of `PokeReader-input.cfg` when the issue is input related.

## License

By contributing you agree that your contribution is licensed under the GNU
General Public License v3, the same license as this project.
