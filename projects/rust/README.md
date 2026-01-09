# ansi-to-html-rs

Convert CP437 ANSI art files to HTML with custom web components.

## Overview

This Rust mono-repo provides tools for converting BBS-style ANSI art files (with Code Page 437 encoding) into HTML that can be displayed in modern web browsers. It supports standard ANSI escape sequences as well as BBS-specific color codes from Synchronet and Renegade BBS software.

## Crates

### lib/ (ansi-to-html-rs)

Core library for converting CP437 byte arrays with ANSI/BBS escape sequences to HTML fragments.

**Features:**
- CP437 to Unicode conversion (all 256 characters including box drawing, symbols, Greek letters)
- ANSI escape sequence support:
  - SGR color codes (30-37, 40-47, 90-97, 100-107)
  - Bold/bright, dim, blink, reverse video
  - Reset, clear screen, cursor forward
  - Save/restore cursor position (collapses text between)
- BBS color code support (optional):
  - **Synchronet Ctrl-A codes**: `^Ar` (red), `^AR` (bright red), `^A1` (blue background), etc.
  - **Renegade pipe codes**: `|00`-`|07` (foreground), `|08`-`|15` (bright), `|16`-`|23` (background)
- Soft line wrapping at column 80 for ANSI content
- HTML escaping for special characters
- UTF-8 input mode (optional): skips CP437 conversion, only converts control characters
- Generates `<ans-kf>` custom elements (k=background, f=foreground in lowercase hex 0-f)

### web/ (ansi-display-rs)

Axum-based web server for testing the ansi-to-html-rs library.

**Features:**
- File upload form accepting `.msg`, `.ans`, and `.txt` files
- Checkbox for UTF-8 input mode (skips CP437 conversion)
- Checkboxes to enable Synchronet and Renegade color code parsing
- Dark-themed UI with live preview
- Dynamic CSS and JavaScript generation for color web components
- Automatic `.env` file loading for configuration
- Configurable port and wwwroot directory via CLI or environment variables

## Installation

```bash
# Clone the repository
git clone https://github.com/bbs-land/bbs-ansi-to-html.git
cd bbs-ansi-to-html/projects/rust

# Build the project
cargo build --release

# Run the test web server
cargo run --bin ansi-display-rs
```

Then open http://127.0.0.1:3000 in your browser.

## Usage

### As a Library

```rust
use ansi_to_html_rs::{convert, convert_with_options, ConvertOptions, generate_css, generate_js};

// Convert standard ANSI art
let ansi_data = b"\x1b[31mRed Text\x1b[0m Normal";
let html = convert(ansi_data);

// Convert with BBS color code support
let options = ConvertOptions {
    synchronet_ctrl_a: true,
    renegade_pipe: true,
    utf8_input: false,
};
let bbs_data = b"|04Red |02Green";
let html = convert_with_options(bbs_data, &options);

// Convert UTF-8 input (skip CP437, only convert control chars)
let utf8_options = ConvertOptions {
    utf8_input: true,
    ..Default::default()
};
let html = convert_with_options(utf8_data, &utf8_options);

// Generate supporting CSS and JavaScript for web components
let css = generate_css();
let js = generate_js();
```

### HTML Output Format

The converter outputs HTML wrapped in a `<pre class="ansi">` element with custom `<ans-kf>` elements for colors:

```html
<pre class="ansi">
  <ans-07>Normal text</ans-07>
  <ans-04>Red text</ans-04>
  <ans-1f>White on blue</ans-1f>
</pre>
```

### Web Server Configuration

#### Command Line Options

```
Usage: ansi-display-rs [OPTIONS]

Options:
  -p, --port <PORT>        Port number to bind against
  -w, --wwwroot <WWWROOT>  Directory for static files (absolute or relative)
  -h, --help               Print help
```

#### Environment Variables

- `HTTP_PORT` or `PORT` - Port number (fallback if CLI not specified)
- `WWWROOT` - Static files directory (used if CLI not specified)

#### .env File Support

The server automatically loads `.env` files on startup. Values do NOT override existing environment variables.

Search order:
1. From current working directory, search upward up to 3 parent directories
2. From executable directory, search upward up to 3 parent directories

Example `.env` file:
```
HTTP_PORT=8080
WWWROOT=/var/www/myapp
```

#### Running Examples

```bash
# Default: port 3000, auto-detect wwwroot
cargo run --bin ansi-display-rs

# Custom port
cargo run --bin ansi-display-rs -- -p 8080

# Custom wwwroot directory
cargo run --bin ansi-display-rs -- -w /path/to/wwwroot

# Using environment variables
HTTP_PORT=8080 cargo run --bin ansi-display-rs
```

## CGA Color Palette

| Code | Name | Hex |
|------|------|-----|
| 0 | Black | #000000 |
| 1 | Blue | #0000AA |
| 2 | Green | #00AA00 |
| 3 | Cyan | #00AAAA |
| 4 | Red | #AA0000 |
| 5 | Magenta | #AA00AA |
| 6 | Brown | #AA5500 |
| 7 | Light Gray | #AAAAAA |
| 8 | Dark Gray | #555555 |
| 9 | Light Blue | #5555FF |
| a | Light Green | #55FF55 |
| b | Light Cyan | #55FFFF |
| c | Light Red | #FF5555 |
| d | Light Magenta | #FF55FF |
| e | Yellow | #FFFF55 |
| f | White | #FFFFFF |

## BBS Color Code Reference

### Synchronet Ctrl-A Codes

| Code | Effect |
|------|--------|
| `^Ak`-`^Aw` | Foreground colors (black, blue, green, cyan, red, magenta, yellow, white) |
| `^AK`-`^AW` | Bright foreground colors |
| `^A0`-`^A7` | Background colors |
| `^AH` | High intensity (bright) |
| `^AN` | Normal (reset) |

### Renegade Pipe Codes

| Code | Effect |
|------|--------|
| `\|00`-`\|07` | Normal foreground colors |
| `\|08`-`\|15` | Bright foreground colors |
| `\|16`-`\|23` | Background colors |

## Test Files

Sample test files are provided in the `../../test-files/` directory (at the repository root):

- `sample.ans` - CP437 ANSI art with box drawing and colors
- `sample-utf8.txt` - UTF-8 test file with Unicode box drawing, block elements, emoji, and international text

## Testing

```bash
cargo test
```

51 tests total (45 library unit tests + 3 env module tests + 3 doc tests).

## License

MIT License - see [LICENSE.md](LICENSE.md)

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.
