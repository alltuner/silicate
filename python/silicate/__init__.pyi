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
) -> bytes:
    """Generate a code image and return PNG bytes.

    Args:
        code: The source code to render.
        language: Language for syntax highlighting (e.g. "python", "rust", "js").
        theme: Color theme name. Use list_themes() to see available themes.
        font: List of (font_name, size) tuples. Defaults to [("Hack", 26.0)].
        show_line_numbers: Whether to display line numbers.
        show_window_controls: Whether to show macOS-style window buttons.
        window_title: Optional title in the window title bar.
        round_corner: Whether to round the image corners.
        background: Background color as hex string (e.g. "#abb8c3").
        shadow_color: Shadow color as hex string.
        shadow_blur_radius: Gaussian blur radius for the shadow.
        pad_horiz: Horizontal padding in pixels.
        pad_vert: Vertical padding in pixels.
        shadow_offset_x: Shadow offset in X axis.
        shadow_offset_y: Shadow offset in Y axis.
        code_pad_right: Padding to the right of the code in pixels.
        highlight_lines: List of 1-based line numbers to highlight.
        tab_width: Number of spaces per tab character.
        line_offset: Starting line number.
        line_pad: Padding between lines in pixels.
    """
    ...

def to_file(
    code: str,
    output: str,
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
) -> None:
    """Generate a code image and save it to a file.

    The output format is determined by the file extension (e.g. .png, .jpg).

    Args:
        code: The source code to render.
        output: Output file path.
        language: Language for syntax highlighting.
        theme: Color theme name. Use list_themes() to see available themes.
        font: List of (font_name, size) tuples. Defaults to [("Hack", 26.0)].
        show_line_numbers: Whether to display line numbers.
        show_window_controls: Whether to show macOS-style window buttons.
        window_title: Optional title in the window title bar.
        round_corner: Whether to round the image corners.
        background: Background color as hex string.
        shadow_color: Shadow color as hex string.
        shadow_blur_radius: Gaussian blur radius for the shadow.
        pad_horiz: Horizontal padding in pixels.
        pad_vert: Vertical padding in pixels.
        shadow_offset_x: Shadow offset in X axis.
        shadow_offset_y: Shadow offset in Y axis.
        code_pad_right: Padding to the right of the code in pixels.
        highlight_lines: List of 1-based line numbers to highlight.
        tab_width: Number of spaces per tab character.
        line_offset: Starting line number.
        line_pad: Padding between lines in pixels.
    """
    ...

def list_themes() -> list[str]:
    """Return a sorted list of available syntax highlighting theme names."""
    ...

def list_languages() -> list[tuple[str, list[str]]]:
    """Return a sorted list of available languages as (name, [extensions]) tuples."""
    ...
