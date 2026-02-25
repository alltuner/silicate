use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use silicon::assets::HighlightingAssets;
use silicon::formatter::ImageFormatterBuilder;
use silicon::utils::{Background, ShadowAdder, ToRgba};
use std::path::Path;
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

static ASSETS: LazyLock<HighlightingAssets> = LazyLock::new(HighlightingAssets::new);

fn parse_color(hex: &str) -> PyResult<image::Rgba<u8>> {
    hex.to_rgba()
        .map_err(|e| PyValueError::new_err(format!("Invalid color '{}': {}", hex, e)))
}

fn render(
    code: &str,
    language: &str,
    theme_name: &str,
    font: Option<Vec<(String, f32)>>,
    show_line_numbers: bool,
    show_window_controls: bool,
    window_title: Option<String>,
    round_corner: bool,
    background: &str,
    shadow_color: &str,
    shadow_blur_radius: f32,
    pad_horiz: u32,
    pad_vert: u32,
    highlight_lines: Option<Vec<u32>>,
    tab_width: u8,
    line_offset: u32,
    line_pad: u32,
) -> PyResult<image::RgbaImage> {
    let ha = &*ASSETS;

    let syntax = ha
        .syntax_set
        .find_syntax_by_token(language)
        .ok_or_else(|| {
            PyValueError::new_err(format!(
                "Unknown language: '{}'. Use list_languages() to see available languages.",
                language
            ))
        })?;

    let theme = ha.theme_set.themes.get(theme_name).ok_or_else(|| {
        PyValueError::new_err(format!(
            "Unknown theme: '{}'. Use list_themes() to see available themes.",
            theme_name
        ))
    })?;

    let mut h = HighlightLines::new(syntax, theme);
    let highlight: Vec<Vec<_>> = LinesWithEndings::from(code)
        .map(|line| {
            h.highlight_line(line, &ha.syntax_set)
                .map_err(|e| PyValueError::new_err(format!("Highlighting error: {}", e)))
        })
        .collect::<PyResult<_>>()?;

    let bg = parse_color(background)?;
    let shadow = parse_color(shadow_color)?;

    let shadow_adder = ShadowAdder::new()
        .background(Background::Solid(bg))
        .shadow_color(shadow)
        .blur_radius(shadow_blur_radius)
        .pad_horiz(pad_horiz)
        .pad_vert(pad_vert);

    let font_spec = font.unwrap_or_else(|| vec![("Hack".to_string(), 26.0)]);
    let font_refs: Vec<(&str, f32)> = font_spec.iter().map(|(n, s)| (n.as_str(), *s)).collect();

    let mut formatter = ImageFormatterBuilder::new()
        .font(font_refs)
        .line_number(show_line_numbers)
        .line_offset(line_offset)
        .line_pad(line_pad)
        .window_controls(show_window_controls)
        .window_title(window_title)
        .round_corner(round_corner)
        .shadow_adder(shadow_adder)
        .highlight_lines(highlight_lines.unwrap_or_default())
        .tab_width(tab_width)
        .build()
        .map_err(|e| PyValueError::new_err(format!("Font error: {}", e)))?;

    let image = formatter.format(&highlight, theme);
    Ok(image)
}

fn encode_png(image: &image::RgbaImage) -> PyResult<Vec<u8>> {
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    encoder
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            image::ColorType::Rgba8,
        )
        .map_err(|e| PyValueError::new_err(format!("PNG encoding failed: {}", e)))?;
    Ok(buf)
}

/// Generate a code image and return PNG bytes.
#[pyfunction]
#[pyo3(signature = (
    code,
    *,
    language = "python",
    theme = "Dracula",
    font = None,
    show_line_numbers = true,
    show_window_controls = true,
    window_title = None,
    round_corner = true,
    background = "#abb8c3",
    shadow_color = "#707070",
    shadow_blur_radius = 50.0,
    pad_horiz = 80,
    pad_vert = 100,
    highlight_lines = None,
    tab_width = 4,
    line_offset = 1,
    line_pad = 2,
))]
fn generate<'py>(
    py: Python<'py>,
    code: &str,
    language: &str,
    theme: &str,
    font: Option<Vec<(String, f32)>>,
    show_line_numbers: bool,
    show_window_controls: bool,
    window_title: Option<String>,
    round_corner: bool,
    background: &str,
    shadow_color: &str,
    shadow_blur_radius: f32,
    pad_horiz: u32,
    pad_vert: u32,
    highlight_lines: Option<Vec<u32>>,
    tab_width: u8,
    line_offset: u32,
    line_pad: u32,
) -> PyResult<Bound<'py, PyBytes>> {
    let image = render(
        code,
        language,
        theme,
        font,
        show_line_numbers,
        show_window_controls,
        window_title,
        round_corner,
        background,
        shadow_color,
        shadow_blur_radius,
        pad_horiz,
        pad_vert,
        highlight_lines,
        tab_width,
        line_offset,
        line_pad,
    )?;
    let bytes = encode_png(&image)?;
    Ok(PyBytes::new(py, &bytes))
}

/// Generate a code image and save it to a file.
///
/// The output format is determined by the file extension (e.g. .png, .jpg).
#[pyfunction]
#[pyo3(signature = (
    code,
    output,
    *,
    language = "python",
    theme = "Dracula",
    font = None,
    show_line_numbers = true,
    show_window_controls = true,
    window_title = None,
    round_corner = true,
    background = "#abb8c3",
    shadow_color = "#707070",
    shadow_blur_radius = 50.0,
    pad_horiz = 80,
    pad_vert = 100,
    highlight_lines = None,
    tab_width = 4,
    line_offset = 1,
    line_pad = 2,
))]
fn to_file(
    code: &str,
    output: &str,
    language: &str,
    theme: &str,
    font: Option<Vec<(String, f32)>>,
    show_line_numbers: bool,
    show_window_controls: bool,
    window_title: Option<String>,
    round_corner: bool,
    background: &str,
    shadow_color: &str,
    shadow_blur_radius: f32,
    pad_horiz: u32,
    pad_vert: u32,
    highlight_lines: Option<Vec<u32>>,
    tab_width: u8,
    line_offset: u32,
    line_pad: u32,
) -> PyResult<()> {
    let image = render(
        code,
        language,
        theme,
        font,
        show_line_numbers,
        show_window_controls,
        window_title,
        round_corner,
        background,
        shadow_color,
        shadow_blur_radius,
        pad_horiz,
        pad_vert,
        highlight_lines,
        tab_width,
        line_offset,
        line_pad,
    )?;

    image
        .save(Path::new(output))
        .map_err(|e| PyValueError::new_err(format!("Failed to save image to '{}': {}", output, e)))?;

    Ok(())
}

/// Return a list of available syntax highlighting theme names.
#[pyfunction]
fn list_themes() -> Vec<String> {
    let ha = &*ASSETS;
    let mut themes: Vec<String> = ha.theme_set.themes.keys().cloned().collect();
    themes.sort();
    themes
}

/// Return a list of available languages as (name, [extensions]) tuples.
#[pyfunction]
fn list_languages() -> Vec<(String, Vec<String>)> {
    let ha = &*ASSETS;
    let mut langs: Vec<(String, Vec<String>)> = ha
        .syntax_set
        .syntaxes()
        .iter()
        .map(|s| (s.name.clone(), s.file_extensions.clone()))
        .collect();
    langs.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    langs
}

#[pymodule]
fn _silicate(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate, m)?)?;
    m.add_function(wrap_pyfunction!(to_file, m)?)?;
    m.add_function(wrap_pyfunction!(list_themes, m)?)?;
    m.add_function(wrap_pyfunction!(list_languages, m)?)?;
    Ok(())
}
