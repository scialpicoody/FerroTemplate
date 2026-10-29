# FerroTemplate

A [`cargo generate`](https://github.com/cargo-generate/cargo-generate) template for
[FerroUI](https://github.com/wieslawsoltes/FerroUI) applications: a cross-platform UI
framework for Rust (a port of Avalonia). One prompt-driven template, four decisions:

| Placeholder | Choices (default first) | What it decides |
|---|---|---|
| style | `rsx`, `builders`, `xaml` | How the UI is written: the `rsx!` macro with `#[component]`, the builder chain of `ferroui-view`, or XAML compiled by `build.rs` (`ferroui-build`) |
| theme | `winui`, `fluent`, `simple` | The theme the application starts with (WinUI 3 is the default of the FerroUI samples). The material and macos themes join the choices once their crates publish to crates.io |
| ferroui_version | `=0.4.0-preview.1` | Pinned exactly for every FerroUI crate, per the framework's version scheme |
| description | free text | The package description |

## Usage

```sh
cargo generate gh:scialpicoody/FerroTemplate
```

Answer the prompts, `cd` into the generated project, `cargo run`.

## What a generated project contains

- A single crate with a `lib` + `bin` split (`src/lib.rs` holds the interface, `src/main.rs`
  the entry point), so unit tests run without a window; two example tests included.
- "Hello, World!" with a click counter (a signal that the text follows) — the smallest
  program that shows FerroUI's reactivity.
- Pinned FerroUI dependencies, a `dist` release profile (fat LTO, stripped symbols),
  `rust-version = "1.89"`, MIT.
- CI (GitHub Actions): `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` on
  Ubuntu, Windows and macOS.
- A smoke switch: `FERROUI_SMOKE_EXIT_MS=<n>` closes the window after `n` milliseconds.
- The XAML style additionally wires the developer tools on F12 in debug builds
  (`ferroui-diagnostics`); the declarative styles carry the built-in Shift+F12 inspector and
  a commented-out hot-reload feature.

## Template development

The template is verified by its own CI (`.github/workflows/template.yml`): every style x
theme combination (15) is rendered and `cargo check`ed on Ubuntu, and the default
combination (`rsx` + `winui`) is spot-checked on Windows and macOS.

Local verification of one combination:

```sh
cargo generate --path . --force --name ferro-rendered \
  -d description="local check" -d ferroui_version="=0.4.0-preview.1" -d style=rsx -d theme=winui
cargo check --manifest-path ferro-rendered/Cargo.toml
```

### Versioning

Tags follow the FerroUI version the template generates for: `v0.4.0-preview.1` is the first (0.4.0 final is not on crates.io yet). When
FerroUI releases, bump the default of `ferroui_version` in `cargo-generate.toml` and tag the
template to match, so `cargo generate gh:scialpicoody/FerroTemplate --tag v0.4.0-preview.1` always
reproduces a project for that framework version.

### Roadmap

- Wave 2: the `mvvm` style (ReactiveUI, view locator, compiled XAML views).
- Theme choices: `material` and `macos`, once `ferroui-themes-material` and
  `ferroui-themes-macos` publish to crates.io (the liquid branches are already in place).
- The developer tools (F12) wiring of the xaml style activates when `ferroui-diagnostics`
  publishes (it is in place, commented out).
- Optional hosts: browser (Emscripten), iOS, Android.

## License

MIT, as FerroUI.
