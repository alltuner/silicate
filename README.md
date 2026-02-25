# Silicate

**Python bindings for [Silicon](https://github.com/Aloxaf/silicon)'s renderer — create beautiful images of your source code, powered by Rust.**

<p align="center">
  <img src="docs/assets/hero-example.png" alt="Silicate example - Python code rendered with Dracula theme" width="700">
</p>

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

## Theme gallery

Silicate ships with 24+ built-in themes. Here are a few:

<table>
  <tr>
    <td align="center"><strong>Dracula</strong><br><img src="docs/assets/hero-example.png" width="400"></td>
    <td align="center"><strong>Nord</strong><br><img src="docs/assets/example-nord.png" width="400"></td>
  </tr>
  <tr>
    <td align="center"><strong>Monokai Extended</strong><br><img src="docs/assets/example-monokai.png" width="400"></td>
    <td align="center"><strong>GitHub</strong><br><img src="docs/assets/example-github.png" width="400"></td>
  </tr>
  <tr>
    <td align="center"><strong>Solarized Dark</strong><br><img src="docs/assets/example-solarized.png" width="400"></td>
    <td align="center"><strong>Your code here</strong><br><img src="docs/assets/example-usage.png" width="400"></td>
  </tr>
</table>

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
| `shadow_offset_x` | `0` | Shadow offset in X axis |
| `shadow_offset_y` | `0` | Shadow offset in Y axis |
| `code_pad_right` | `25` | Right padding inside code window (px) |
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

---

Built by [All Tuner Labs](https://alltuner.com)
