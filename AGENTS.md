# AGENTS.md

`devforge` is a Rust CLI (clap) that generates project scaffolds from `<prefix>-<name>` conventions. Templates are embedded into the binary at build time with `include_dir`.

## Commands

- Build: `make build` → `bin/devforge` (also `cargo build --release --locked` → `target/release/devforge`)
- Run home: `./bin/devforge` (no subcommand)
- Run: `./bin/devforge create api-catalog --lang go`
- Test all: `make test` / `cargo test --all-features --locked`
- Single test: `cargo test prefix::detect` or `cargo test --test parity`
- Format: `make fmt` (writes), `make fmt-check` (fails if anything is unformatted)
- Lint: `make lint` / `cargo clippy --all-targets --all-features --locked -- -D warnings`; `make vet` is an alias
- Full pre-PR check: `make all` → `clean → fmt-check → lint → build → test` (PR checklist requires it)
- Install: `make install` → `cargo install --path . --locked --force` → `~/.cargo/bin/devforge`
- Uninstall: `make uninstall` → `cargo uninstall devforge`

The local build is `bin/devforge`; the installed binary is `~/.cargo/bin/devforge`. The legacy Go binary was `~/go/bin/cli-devforge`.

## Architecture

- `src/main.rs`: parses clap, dispatches commands (or the no-args home screen), maps errors to `Error: ...` + exit 1; `src/lib.rs` exposes modules for integration tests.
- `src/cli.rs`: clap command/flag definitions; `command` is `Option<Command>` so no args reaches the home screen.
- `src/banner.rs`: home screen banner, gradient styles, version, prefix count, and command hints; uses `anstream`/`anstyle`.
- `src/commands/{create,home,list,version}.rs`: command handlers.
- `src/prefix/registry.rs`: source of truth for prefixes, supported languages, `default_lang`, and `template_dir`; `detect.rs` splits the project name on the **first** `-` (`api-user-service` → prefix `api`, project `user-service`).
- `src/scaffold.rs`: resolves the template dir, builds `Data`, renders every file, and writes the project.
- `src/template/engine.rs`: `Renderer` trait + `GoSubstitutionRenderer` (only `{{.Field}}` with optional whitespace).
- `src/template/embedded.rs`: `include_dir!`, recursive listing, and output path mapping.
- `src/output.rs`: exact `created` lines, post-creation message, and `list` table formatting.
- `src/error.rs`: `thiserror` error enum.
- `build.rs`: injects `DEVFORGE_VERSION`.

## Adding a prefix or template (2 places must stay in sync)

1. Create `templates/<prefix>/<lang>/...` (files end in `.tmpl`).
2. Add an entry (name, description, langs, default_lang, template_dir) in `src/prefix/registry.rs`.

There is no embed directive to update: `include_dir!` embeds everything under `templates/`. The tests `every_registered_language_has_templates` and `every_template_renders_with_known_fields` validate new templates automatically. If a language is advertised without templates, the test fails (current exception: `worker/python`).

Other template rules:

- Only file **contents** are rendered. Output path is the template path with `<prefix>/<lang>/` stripped and `.tmpl` removed; directory/file names are fixed.
- `.gitkeep.tmpl` becomes `.gitkeep`.
- Java templates are intentionally minimal stubs (`Dockerfile` + `src/main/java/.gitkeep`), not full scaffolds.
- Go `ModulePath` defaults to `github.com/example/<project>` when `--module` is empty.
- Available vars: `{{.FullName}} {{.Prefix}} {{.ProjectName}} {{.Lang}} {{.Author}} {{.ModulePath}} {{.License}} {{.Date}} {{.Description}}`.
- The renderer supports only simple substitutions. Unknown fields and control structures (`if`, `range`, pipelines) return an error. Add new fields to `Data` and `field_value` in `src/template/engine.rs`.
- `include_dir` embeds dotfiles, so `.gitignore.tmpl` and `.gitkeep.tmpl` are safe.
- Use `Path` components for template paths (never trim string prefixes with `/` or `\`); this keeps Windows working.

## Parity fixtures

- `tests/fixtures/golden/` was captured from the legacy Go binary by `scripts/capture-golden.sh`. The script is kept for reference and must be run from a checkout of the legacy Go version (for example, tag `v0.4.0`), not against the Rust binary.
- `tests/parity.rs` compares exit code, stdout, generated tree, and the core error message.
- Regenerate fixtures only when behaviour intentionally changes. `worker/python` now reports `no templates found for worker/python` instead of the legacy Go walk error.
- Running `devforge` with no arguments is intentionally divergent: the Rust binary prints the home screen and exits 0, while the legacy Go binary printed help and exited 1. It is not part of the golden fixtures.

## Versioning / releases

- `build.rs` resolves the version in this order: `DEVFORGE_VERSION` → `git describe --tags --always --dirty` → `dev`, then sets `cargo:rustc-env=DEVFORGE_VERSION=...`.
- Release workflows set `DEVFORGE_VERSION` to the tag (`vX.Y.Z`).
- Pushing to `main` triggers `auto-release.yml`, which derives the next semver from commit subjects: `BREAKING CHANGE`/`major:` → major, `feat:`/`feature:` → minor, everything else → patch; first release is `0.1.0`.
- `release-build.yml` is the reusable matrix: Linux amd64/arm64 (arm64 via `cross`), macOS amd64/arm64, Windows amd64; it publishes assets + `checksums.txt`.
- Flow: `feature/*` → PR → `develop` → PR → `main`.

## Gotchas

- `Cargo.lock` is committed; CI and release commands use `--locked`.
- CI checks formatting with `cargo fmt --all -- --check`; `make fmt` writes changes locally.
- `include_dir` paths are relative to the embedded root; use `TEMPLATES.get_dir("prefix/lang")`, not `get_dir("prefix").get_dir("lang")`.
- `worker/python` is advertised but has no templates; `create worker-x --lang python` exits 1.
- `.Date` is available to templates but currently unused.
- The home banner is UTF-8 block art. `anstream` strips ANSI when piped or when `NO_COLOR` is set; it respects `CLICOLOR`/`CLICOLOR_FORCE` and supports Windows ANSI. The glyphs themselves still require a UTF-8-capable terminal.
