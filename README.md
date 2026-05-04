<h1 align="center">Silicate</h1>

<p align="center">
  <strong>Python bindings for <a href="https://github.com/Aloxaf/silicon">Silicon</a>'s renderer.</strong><br>
  Create beautiful images of your source code, powered by Rust.
</p>

<p align="center">
  <a href="https://alltuner.com/sponsor">Sponsor</a>
</p>

<p align="center">
  <img src="https://img.shields.io/pypi/v/silicate?color=5B2333" alt="PyPI">
  <img src="https://img.shields.io/github/license/alltuner/silicate?color=5B2333" alt="License">
  <img src="https://img.shields.io/github/stars/alltuner/silicate?color=5B2333" alt="Stars">
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/alltuner/silicate/main/docs/assets/hero-example.png" alt="Silicate example: Python code rendered with the Dracula theme" width="700">
</p>

---

## Get Started

```bash
uv add silicate
```

```python
import silicate

png_bytes = silicate.generate(
    "print('Hello, World!')",
    language="python",
    theme="Dracula",
)
```

---

## What is Silicate?

Silicate is a Python binding for [Silicon](https://github.com/Aloxaf/silicon), the Rust tool that turns source code into beautifully rendered PNGs. Unlike wrapper approaches that shell out to a CLI, Silicate uses [PyO3](https://pyo3.rs) to call Silicon's Rust library directly, giving you native performance with a clean Python API.

### Features

- **Native performance** — no subprocess, no temp files; the renderer runs in-process via PyO3.
- **24+ built-in themes** — Dracula, Nord, Monokai, GitHub, Solarized, and more.
- **Full Silicon control** — every renderer option exposed as a keyword argument.
- **PNG bytes or file** — `generate()` returns bytes, `to_file()` writes directly.

## Usage

```python
import silicate

# PNG bytes
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

### Theme gallery

<table>
  <tr>
    <td align="center"><strong>Dracula</strong><br><img src="https://raw.githubusercontent.com/alltuner/silicate/main/docs/assets/hero-example.png" width="400"></td>
    <td align="center"><strong>Nord</strong><br><img src="https://raw.githubusercontent.com/alltuner/silicate/main/docs/assets/example-nord.png" width="400"></td>
  </tr>
  <tr>
    <td align="center"><strong>Monokai Extended</strong><br><img src="https://raw.githubusercontent.com/alltuner/silicate/main/docs/assets/example-monokai.png" width="400"></td>
    <td align="center"><strong>GitHub</strong><br><img src="https://raw.githubusercontent.com/alltuner/silicate/main/docs/assets/example-github.png" width="400"></td>
  </tr>
  <tr>
    <td align="center"><strong>Solarized Dark</strong><br><img src="https://raw.githubusercontent.com/alltuner/silicate/main/docs/assets/example-solarized.png" width="400"></td>
    <td align="center"><strong>Your code here</strong><br><img src="https://raw.githubusercontent.com/alltuner/silicate/main/docs/assets/example-usage.png" width="400"></td>
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

[MIT](LICENSE)

## Support the project

Silicate is an open source project built by [David Poblador i Garcia](https://davidpoblador.com/) through [All Tuner Labs](https://www.alltuner.com/).

If this project was useful to you, [consider supporting its development](https://alltuner.com/sponsor).

---

<p align="center">
  Built by <a href="https://davidpoblador.com">David Poblador i Garcia</a> with the support of <a href="https://alltuner.com">All Tuner Labs</a>.<br>
  Made with ❤️ in Poblenou, Barcelona.
</p>
