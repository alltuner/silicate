# Installation

## From PyPI

```bash
uv add silicate
```

Or as a standalone script dependency:

```bash
uv run --with silicate python my_script.py
```

!!! note "Pre-built wheels"
    Pre-built wheels are available for:

    - macOS (arm64, x86_64)
    - Linux (x86_64, aarch64)
    - Windows (x86_64)

    If a wheel isn't available for your platform, uv will build from source (requires a Rust toolchain).

## From source

Building from source requires:

- **Rust toolchain** (1.70+) — install via [rustup](https://rustup.rs)
- **Python** (3.9+)
- **System fonts** — the default font is Hack (bundled), but custom fonts require system font libraries

```bash
# Clone with submodules (Silicon is vendored as a git submodule)
git clone --recurse-submodules https://github.com/alltuner/silicate.git
cd silicate

# Install
uv sync
```

### Development setup

```bash
git clone --recurse-submodules https://github.com/alltuner/silicate.git
cd silicate

# Create venv and build in development mode
uv venv
VIRTUAL_ENV=.venv uvx maturin develop

# Verify
uv run python -c "import silicate; print(silicate.list_themes())"
```

!!! tip "Faster iteration"
    During development, `maturin develop` compiles in debug mode by default (faster builds, slower runtime). Add `--release` for optimized builds:

    ```bash
    VIRTUAL_ENV=.venv uvx maturin develop --release
    ```

## System dependencies

### macOS

No extra dependencies needed — font rendering uses CoreText (built-in).

### Linux

You may need fontconfig and freetype development libraries:

=== "Debian / Ubuntu"

    ```bash
    sudo apt-get install pkg-config libfontconfig1-dev libfreetype-dev
    ```

=== "Fedora / RHEL"

    ```bash
    sudo dnf install fontconfig-devel freetype-devel
    ```

=== "Arch"

    ```bash
    sudo pacman -S fontconfig freetype2
    ```

### Windows

No extra dependencies needed — font rendering uses DirectWrite (built-in).
