# Contributing to FerroTemplate

Thanks for considering a contribution. This document explains how the template is put
together, how to verify a change locally, and how a release is cut.

## Layout

```
cargo-generate.toml        Placeholders (style, theme, ferroui_version, description),
                           conditional file selection, template-internal ignore list
Cargo.toml                 Generated manifest (Liquid branches per style)
README.md.liquid           Generated project's README (the `.liquid` suffix is stripped)
build.rs                   XAML style only: compiles App.xaml / MainWindow.xaml
App.xaml, MainWindow.xaml  XAML style only (excluded from the other styles)
src/lib.rs                 Generated interface — one Liquid branch per style
src/main.rs                Generated entry point — XAML (AppBuilder) or launch_themed
src/app.rs,                XAML style only: the application and window classes,
src/main_window.rs         populated from their documents (ferroui-build `populate`)
src/register_types.rs      XAML style only: the crate's markup type table
.github/workflows/ci.yml   Generated project's CI (copied into every generation)
.github/workflows/template.yml   This repository's CI: renders the matrix and checks it
```

Files in the `[template] ignore` list of `cargo-generate.toml` (this README,
`CONTRIBUTING.md`, `CHANGELOG.md`, `template.yml`) exist only for the repository and are
never copied into a generated project.

The Liquid branches already cover the `material` and `macos` themes; their choices are
hidden until the theme crates publish to crates.io.

## Verifying a change

Render every style locally and run the same gates the generated CI runs:

```sh
export CARGO_TARGET_DIR=/tmp/ferro-template-target   # shared across renders
for combo in "rsx winui" "builders fluent" "xaml winui"; do
  set -- $combo
  cargo generate --path . --force --name ferro-$1 \
    -d description="verify" -d ferroui_version="=0.4.0-preview.1" \
    -d style=$1 -d theme=$2
  (cd ferro-$1 && cargo fmt --check && cargo clippy -- -D warnings && cargo test)
done
```

All three gates must pass for every style: the generated CI runs exactly
`cargo fmt --check`, `cargo clippy -- -D warnings` and `cargo test`, so the template output
has to be clean under default rustfmt and clippy. When you edit the Liquid sources, keep
whitespace control in mind: `-%}` trims what follows (useful at end of file and between
branches); `{%-` is not honoured by cargo-generate's Liquid engine.

The full matrix (3 styles x 3 themes) is checked by the repository CI on every push.

## Rules of thumb

- The generated code should stay close to the upstream samples
  (`FerroUI/samples/HelloWorld.*`) — it is the diffable reference users read the
  documentation against.
- Depend only on crates that exist on crates.io at the pinned version; anything
  unpublished (currently `ferroui-diagnostics`, `ferroui-themes-material`,
  `ferroui-themes-macos`) stays commented out or out of the choices, with the wiring in
  place where reasonable.
- Keep the generated project minimal: it is a starting point, not a demo gallery.

## Cutting a release

1. Wait for the FerroUI release the template will target (all crates of the release on
   crates.io at the same exact version).
2. Bump the default of `ferroui_version` in `cargo-generate.toml`, the version in the
   badge and verification snippets of `README.md`, `CONTRIBUTING.md`,
   `.github/workflows/template.yml`, and add a `CHANGELOG.md` entry.
3. Re-render and verify locally (above); let the repository CI go green.
4. Tag and push: `git tag vX.Y.Z && git push origin vX.Y.Z`.
