# Silicate

**Python bindings for [Silicon](https://github.com/Aloxaf/silicon)'s renderer — create beautiful images of your source code, powered by Rust.**

Unlike wrapper approaches that shell out to a CLI, Silicate uses [PyO3](https://pyo3.rs) to call Silicon's Rust library directly, giving you native performance with a clean Python API.

## Installation

```bash
uv add silicate
```

**Build from source** (requires Rust toolchain):

```bash
git clone --recurse-submodules https://github.com/alltuner/silicate.git
cd silicate
uv sync
```

## Usage

```python
import silicate

# Generate PNG bytes
png_bytes = silicate.generate(
    "print('Hello, World!')",
    language="python",
    theme="Dracula",
)

# Save directly to file
silicate.to_file(
    "fn main() { println!(\"Hello!\"); }",
    "output.png",
    language="rs",
    theme="Nord",
    window_title="main.rs",
)

# Explore available themes and languages
print(silicate.list_themes())
print(silicate.list_languages())
```

### Options

All parameters after `code` (and `output` for `to_file`) are keyword-only:

| Parameter | Default | Description |
|---|---|---|
| `language` | `"python"` | Syntax highlighting language |
| `theme` | `"Dracula"` | Color theme (use `list_themes()`) |
| `font` | `None` | Font list as `[("Name", size)]`, defaults to Hack 26pt |
| `show_line_numbers` | `True` | Display line numbers |
| `show_window_controls` | `True` | macOS-style window buttons |
| `window_title` | `None` | Title bar text |
| `round_corner` | `True` | Rounded image corners |
| `background` | `"#abb8c3"` | Background color (hex) |
| `shadow_color` | `"#707070"` | Shadow color (hex) |
| `shadow_blur_radius` | `50.0` | Shadow blur radius |
| `pad_horiz` | `80` | Horizontal padding (px) |
| `pad_vert` | `100` | Vertical padding (px) |
| `highlight_lines` | `None` | 1-based line numbers to highlight |
| `tab_width` | `4` | Spaces per tab |
| `line_offset` | `1` | Starting line number |
| `line_pad` | `2` | Line spacing (px) |

## Development

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/alltuner/silicate.git
cd silicate

# Create venv and build
uv venv
VIRTUAL_ENV=.venv uvx maturin develop

# Test
uv run python -c "import silicate; print(silicate.list_themes())"
```

## License

MIT
