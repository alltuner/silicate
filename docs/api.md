# API Reference

## `generate`

```python
def generate(
    code: str,
    *,
    language: str = "python",
    theme: str = "Dracula",
    font: list[tuple[str, float]] | None = None,
    show_line_numbers: bool = True,
    show_window_controls: bool = True,
    window_title: str | None = None,
    round_corner: bool = True,
    background: str = "#abb8c3",
    shadow_color: str = "#707070",
    shadow_blur_radius: float = 50.0,
    pad_horiz: int = 80,
    pad_vert: int = 100,
    shadow_offset_x: int = 0,
    shadow_offset_y: int = 0,
    code_pad_right: int = 25,
    highlight_lines: list[int] | None = None,
    tab_width: int = 4,
    line_offset: int = 1,
    line_pad: int = 2,
) -> bytes
```

Generate a code image and return raw PNG bytes.

**Parameters:**

| Parameter | Type | Default | Description |
|---|---|---|---|
| `code` | `str` | *(required)* | Source code to render |
| `language` | `str` | `"python"` | Language for syntax highlighting. Accepts language names (`"python"`, `"rust"`) or file extensions (`"py"`, `"rs"`). |
| `theme` | `str` | `"Dracula"` | Color theme. Use [`list_themes()`](#list_themes) to see options. |
| `font` | `list[tuple[str, float]] \| None` | `None` | Font list as `[(name, size)]` tuples. Falls back to bundled Hack 26pt. |
| `show_line_numbers` | `bool` | `True` | Display line numbers |
| `show_window_controls` | `bool` | `True` | Show macOS-style window buttons |
| `window_title` | `str \| None` | `None` | Title bar text |
| `round_corner` | `bool` | `True` | Round image corners |
| `background` | `str` | `"#abb8c3"` | Background color as hex (`#RGB`, `#RRGGBB`, or `#RRGGBBAA`) |
| `shadow_color` | `str` | `"#707070"` | Shadow color as hex |
| `shadow_blur_radius` | `float` | `50.0` | Gaussian blur radius for shadow |
| `pad_horiz` | `int` | `80` | Horizontal padding in pixels |
| `pad_vert` | `int` | `100` | Vertical padding in pixels |
| `shadow_offset_x` | `int` | `0` | Shadow offset in X axis |
| `shadow_offset_y` | `int` | `0` | Shadow offset in Y axis |
| `code_pad_right` | `int` | `25` | Padding to the right of the code in pixels |
| `highlight_lines` | `list[int] \| None` | `None` | 1-based line numbers to highlight |
| `tab_width` | `int` | `4` | Spaces per tab character |
| `line_offset` | `int` | `1` | Starting line number |
| `line_pad` | `int` | `2` | Spacing between lines in pixels |

**Returns:** `bytes` — Raw PNG image data.

**Raises:** `ValueError` — If the language, theme, or color format is invalid.

**Example:**

```python
import silicate

png = silicate.generate(
    "console.log('hello')",
    language="js",
    theme="Nord",
)

# Use the bytes directly (e.g., in a web response)
# or write to file
with open("output.png", "wb") as f:
    f.write(png)
```

---

## `to_file`

```python
def to_file(
    code: str,
    output: str,
    *,
    # ... same keyword arguments as generate()
) -> None
```

Generate a code image and save it directly to a file.

Accepts the same parameters as [`generate()`](#generate), plus:

| Parameter | Type | Description |
|---|---|---|
| `output` | `str` | Output file path. Format is determined by extension (`.png`, `.jpg`, etc.) |

**Returns:** `None`

**Raises:** `ValueError` — If parameters are invalid or the file cannot be written.

**Example:**

```python
silicate.to_file(
    "SELECT * FROM users WHERE active = true;",
    "query.png",
    language="sql",
    theme="GitHub",
    show_window_controls=False,
)
```

---

## `list_themes`

```python
def list_themes() -> list[str]
```

Return a sorted list of all available syntax highlighting theme names.

**Returns:** `list[str]` — Theme names that can be passed to the `theme` parameter.

**Example:**

```python
>>> silicate.list_themes()
['1337', 'Coldark-Cold', 'Coldark-Dark', 'DarkNeon', 'Dracula', ...]
```

---

## `list_languages`

```python
def list_languages() -> list[tuple[str, list[str]]]
```

Return a sorted list of all available languages as `(name, [extensions])` tuples.

**Returns:** `list[tuple[str, list[str]]]` — Each entry is `(language_name, [file_extensions])`.

**Example:**

```python
>>> silicate.list_languages()[:3]
[('ActionScript', ['as']),
 ('Apache Conf', ['envvars', 'htaccess', ...]),
 ('AppleScript', ['applescript', 'script editor'])]
```

The `language` parameter in `generate()` and `to_file()` accepts either the language name or any of its file extensions.
