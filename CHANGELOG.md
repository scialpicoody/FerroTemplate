# Changelog

Tags follow the FerroUI version the template generates for.

## v0.4.0-preview.1 — 2026-10-29

Initial release.

- Styles: `rsx` (default, `rsx!` + `#[component]`), `builders`, `xaml` (compiled markup via
  `ferroui-build`, `AppBuilder` entry point).
- Themes: `winui` (default), `fluent`, `simple` — `material` and `macos` join when their
  crates publish to crates.io (Liquid branches already in place).
- FerroUI `=0.4.0-preview.1` pinned exactly for every FerroUI crate.
- Generated projects: `lib` + `bin` split, unit tests, GitHub Actions CI (fmt, clippy, test
  on Ubuntu/Windows/macOS), `dist` release profile, `FERROUI_SMOKE_EXIT_MS` smoke switch,
  platform prerequisites in the README.
- The F12 developer-tools wiring of the `xaml` style is in place and commented out until
  `ferroui-diagnostics` publishes.
