# FerroTemplate

[![Template CI](https://github.com/scialpicoody/FerroTemplate/actions/workflows/template.yml/badge.svg)](https://github.com/scialpicoody/FerroTemplate/actions/workflows/template.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![FerroUI](https://img.shields.io/badge/framework-FerroUI%200.4.0--preview.1-7C3AED.svg)](https://github.com/wieslawsoltes/FerroUI)

A [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) template for
[FerroUI](https://github.com/wieslawsoltes/FerroUI) applications — a cross-platform UI
framework for Rust (a port of Avalonia UI). One prompt-driven template that scaffolds a
complete, CI-verified desktop application in your choice of UI style and theme.

| You choose | You get |
|---|---|
| **UI style** — `rsx` (default), `builders`, `xaml` | `rsx!` + `#[component]`, the builder chain of `ferroui-view`, or XAML compiled by `ferroui-build` |
| **Theme** — `winui` (default), `fluent`, `simple` | The theme the application starts with, wired into dependencies and code |
| **FerroUI version** — `=0.4.0-preview.1` (default) | Every FerroUI crate pinned to the same exact version |
| **Description** | The package description |

## Quick start

```sh
cargo install cargo-generate        # once
cargo generate --git https://github.com/scialpicoody/FerroTemplate.git
cd <your-project>
cargo run                           # a window with "Hello, World!" and a click counter
```

## Ways to generate

The `--git <url>` form works with any git remote — HTTPS, SSH, GitHub, GitLab,
self-hosted — so it is the recommended spelling. The `gh:` shorthand below is its
equivalent for GitHub (`gh:owner/repo` = `--git https://github.com/owner/repo.git`).

**Interactive** (answer the four prompts, Enter accepts the default):

```sh
cargo generate --git https://github.com/scialpicoody/FerroTemplate.git
# shorthand: cargo generate gh:scialpicoody/FerroTemplate
# over SSH:  cargo generate --git git@github.com:scialpicoody/FerroTemplate.git
```

**Non-interactive** (skip the prompts; all four placeholders must be defined):

```sh
cargo generate --git https://github.com/scialpicoody/FerroTemplate.git --name my-app \
  -d description="My app" \
  -d ferroui_version="=0.4.0-preview.1" \
  -d style=xaml \
  -d theme=fluent
```

**Pinned to a template release** (tags follow the FerroUI version, see
[Versioning](#versioning)):

```sh
cargo generate --git https://github.com/scialpicoody/FerroTemplate.git --tag v0.4.0-preview.1
```

**From a local clone** (template development):

```sh
git clone https://github.com/scialpicoody/FerroTemplate
cargo generate --path FerroTemplate
```

**As a favorite** (a short alias, after adding this to `$CARGO_HOME/cargo-generate.toml`):

```toml
[favorites.ferroui]
description = "A FerroUI application"
git = "https://github.com/scialpicoody/FerroTemplate"
```

```sh
cargo generate ferroui
# with a pinned version:
cargo generate ferroui --tag v0.4.0-preview.1
```

> **Note**: this repository is deliberately *not* a GitHub "template repository"
> (no *Use this template* button) — its files carry Liquid templating that only
> `cargo generate` expands.

## What a generated project contains

- A single crate with a `lib` + `bin` split: `src/lib.rs` holds the interface, `src/main.rs`
  the entry point, so unit tests run without a window; two example tests included.
- "Hello, World!" with a click counter — a signal that the button text follows — the
  smallest program that shows FerroUI's reactivity.
- Every FerroUI dependency pinned to the same exact version (`=0.4.0-preview.1` by default).
- A `dist` release profile (fat LTO, stripped symbols), `rust-version = "1.89"`, MIT license.
- CI (GitHub Actions): `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` on
  Ubuntu, Windows and macOS.
- A smoke switch: `FERROUI_SMOKE_EXIT_MS=<n>` closes the window after `n` milliseconds.
- Style-specific extras:

| Style | Extras |
|---|---|
| `rsx`, `builders` | The Shift+F12 inspector of a running application (built into debug builds); a commented-out `hot-reload` feature |
| `xaml` | `build.rs` + `App.xaml` + `MainWindow.xaml` compiled by `ferroui-build`; `AppBuilder` entry point; the F12 developer-tools wiring in place, commented out until `ferroui-diagnostics` publishes |

## Requirements

- Rust 1.89 or later.
- The first build downloads a prebuilt Skia and builds HarfBuzz from source (a C++
  compiler is needed); it can take a while.
- Linux: the development packages of fontconfig and FreeType
  (`libfontconfig1-dev`, `libfreetype6-dev`); macOS: the Xcode command line tools;
  Windows: the MSVC build tools.
- Platform details and the browser/iOS/Android hosts: the
  [FerroUI platform documents](https://github.com/wieslawsoltes/FerroUI/tree/main/docs/porting).

## Verification

The template is verified by its own CI ([.github/workflows/template.yml](.github/workflows/template.yml)):
every style x theme combination (3 x 3) is rendered and `cargo check`ed on Ubuntu, and the
default combination (`rsx` + `winui`) is spot-checked on Windows and macOS.

To verify a combination locally:

```sh
cargo generate --path . --force --name ferro-rendered \
  -d description="local check" -d ferroui_version="=0.4.0-preview.1" -d style=rsx -d theme=winui
cargo check --manifest-path ferro-rendered/Cargo.toml
```

## Versioning

Tags follow the FerroUI version the template generates for: `v0.4.0-preview.1` is the first
(`0.4.0` final is not on crates.io yet). When FerroUI releases, the default of
`ferroui_version` in [cargo-generate.toml](cargo-generate.toml) is bumped and a matching tag
is published, so

```sh
cargo generate --git https://github.com/scialpicoody/FerroTemplate.git --tag <version>
```

always reproduces a project for that framework version. See [CHANGELOG.md](CHANGELOG.md).

## Roadmap

- Wave 2: the `mvvm` style (ReactiveUI, view locator, compiled XAML views).
- Theme choices: `material` and `macos`, once `ferroui-themes-material` and
  `ferroui-themes-macos` publish to crates.io (the Liquid branches are already in place).
- The F12 developer tools of the `xaml` style activate when `ferroui-diagnostics` publishes
  (the wiring is in place, commented out).
- Optional hosts: browser (Emscripten), iOS, Android.

## Contributing

Issues and pull requests welcome — see [CONTRIBUTING.md](CONTRIBUTING.md) for the layout of
the template, the local verification loop and the release procedure.

## License

MIT, as FerroUI. Generated projects carry the same license (change it if you wish).

## Acknowledgements

Built on [FerroUI](https://github.com/wieslawsoltes/FerroUI) by
[Wiesław Šoltés](https://github.com/wieslawsoltes) — a Rust port of the
[Avalonia UI](https://github.com/AvaloniaUI/Avalonia) architecture.
