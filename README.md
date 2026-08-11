# dev_cleaner

A CLI tool to reclaim disk space from old or inactive development projects. It scans a directory for well-known build artifacts, caches, and dependency folders — then deletes them.

## Usage

```
dev_cleaner <PATH> [OPTIONS]
```

### Options

| Flag | Short | Description |
|---|---|---|
| `--dry-run` | `-d` | Scan and report size without deleting anything |
| `--recursive` | `-r` | Scan subdirectories recursively (default: true) |
| `--display-path` | `-p` | Print each matched path and its size before deleting |

### Examples

```sh
# Preview what would be deleted under ~/projects
dev_cleaner ~/projects --dry-run

# Clean and show each matched path
dev_cleaner ~/projects --display-path

# Clean only the top-level folders (non-recursive)
dev_cleaner ~/projects --recursive=false
```

## Configuration (`clean.toml`)

Place a `clean.toml` file in the root of the directory you are scanning to customize behavior.

```toml
[settings]
dry_run = false
recursive = true
display_path = false

[targets]
include = ["node_modules", "target", ".cache"]
exclude = ["dist"]
```

### `[targets].include`

> **Replaces the default match list entirely.** When `include` is set in `clean.toml`, the built-in defaults are ignored and only the patterns you list will be targeted for deletion.

### `[targets].exclude`

Removes specific patterns from whatever match list is active (defaults or your custom `include`). Useful for opting out of a few entries without rewriting the full list.

---

## Default Targets

When no `clean.toml` is present (or `include` is not set), the following patterns are matched:

| Pattern | Description |
|---|---|
| `node_modules` | Node.js dependencies |
| `target` | Rust build output |
| `package-lock.json` | npm lock file |
| `.next` | Next.js build cache |
| `.nuxt` | Nuxt.js build cache |
| `.svelte-kit` | SvelteKit build output |
| `.turbo` | Turborepo cache |
| `.parcel-cache` | Parcel bundler cache |
| `.cache` | Generic cache directory |
| `dist` | Distribution output |
| `build` | Generic build output |
| `yarn.lock` | Yarn lock file |
| `pnpm-lock.yaml` | pnpm lock file |
| `_pycache_` | Python bytecode cache |
| `.pytest_cache` | pytest cache |
| `.mypy_cache` | mypy type checker cache |
| `.ruff_cache` | Ruff linter cache |
| `*.egg-info` | Python egg metadata |
| `.gradle` | Gradle build cache |
| `out` | Generic output directory |
| `*.class` | Java compiled class files |
| `cmake-build-debug` | CMake debug build |
| `cmake-build-release` | CMake release build |
| `.cmake` | CMake cache |
| `.DS_Store` | macOS metadata files |
| `Thumbs.db` | Windows thumbnail cache |
| `.idea` | JetBrains IDE config |
| `*.log` | Log files |
| `coverage` | Test coverage output |
| `.nyc_output` | nyc (Istanbul) coverage output |
| `lcov.info` | LCOV coverage report |
| `.bundle` | Ruby Bundler directory |
| `vendor/bundle` | Ruby vendored gems |
| `tmp` | Temporary files |
| `.rspec_status` | RSpec status file |
| `.yardoc` | YARD documentation cache |
| `pkg` | Generic package output |
| `*.rbc` | Ruby compiled bytecode |
| `luac.out` | Lua compiled output |
| `*.luac` | Lua compiled files |
| `.luarocks` | LuaRocks packages |
| `lua_modules` | Lua modules directory |
| `.Rhistory` | R session history |
| `.RData` | R workspace data |
| `.Rproj.user` | RStudio project user data |
| `*_cache` | Generic cache directories |
| `*_files` | Generic file output directories |
| `.drake` | Drake (R) cache |
| `renv/library` | R renv library |
| `renv/staging` | R renv staging |
| `_build` | Elixir/Mix build output |
| `.elixir_ls` | ElixirLS language server data |
| `.php_cs.cache` | PHP CS Fixer cache |
| `DerivedData` | Xcode derived data |
| `.build` | Swift Package Manager build |
| `*.tmp` | Temporary files |
| `*.bak` | Backup files |
| `*.swp` | Vim swap files |
| `.sass-cache` | Sass/SCSS compiler cache |
| `*.lock` | Generic lock files |

## Installation

```sh
git clone https://github.com/your-username/dev_cleaner
cd dev_cleaner
cargo build --release
```

The binary will be at `target/release/dev_cleaner`.

Or install it directly to your Cargo bin path (available system-wide):

```sh
cargo install --path .
```
