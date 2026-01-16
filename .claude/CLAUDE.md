# bbs-ansi-to-html

A multi-language mono-repo for converting CP437 ANSI art files to HTML with custom web components.

## Project Structure

```
ansi-to-html/
├── LICENSE.md              # MIT License (2026 BBS.land)
├── README.md               # Project overview documentation
├── wwwroot/                # Shared static web assets
│   ├── style.css           # App styles (upload form, layout)
│   ├── ansi-display.css    # ANSI display styles (fonts, pre.ansi)
│   ├── ansi-display.js     # Web component definitions
│   └── fonts/
│       └── Px437_IBM_VGA_8x16.ttf
├── test-files/             # Sample ANSI art files for testing
│   ├── sample.ans          # CP437 ANSI art with box drawing and colors
│   └── sample-utf8.txt     # UTF-8 test file with Unicode
└── projects/
    ├── rust/               # Rust implementation
    │   ├── Cargo.toml      # Workspace configuration
    │   ├── README.md       # Rust project documentation
    │   ├── lib/            # Core library crate (ansi-to-html-rs)
    │   │   ├── Cargo.toml
    │   │   └── src/
    │   │       ├── lib.rs  # Main conversion logic
    │   │       └── cp437.rs # CP437 to Unicode mapping
    │   └── web/            # Web application crate (ansi-display-rs)
    │       ├── Cargo.toml
    │       ├── build.rs    # Copies wwwroot to target directory
    │       └── src/
    │           ├── main.rs # Entry point
    │           ├── config.rs # CLI argument and environment variable handling
    │           ├── env.rs  # .env file loading logic
    │           ├── server.rs # Axum web server and route handlers
    │           └── wwwroot.rs # wwwroot directory resolution logic
    └── typescript/         # TypeScript/Deno implementation
        ├── lib/            # Core library
        │   ├── deno.json   # Library configuration
        │   └── src/
        │       ├── index.ts # Library exports
        │       ├── converter.ts # Main conversion logic
        │       ├── cp437.ts # CP437 to Unicode mapping
        │       └── colors.ts # Color utilities
        └── web/            # Vite + React web application
            ├── deno.json   # Web app configuration
            └── src/
                └── main.tsx # React entry point
```

## CGA Color Palette

| # | Name | Hex |
|---|------|-----|
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

## Rust Implementation (projects/rust/)

### lib/ (ansi-to-html-rs)

Core library for converting CP437 byte arrays with ANSI/BBS escape sequences to HTML.

#### Features

- **CP437 to Unicode conversion**: All 256 characters including box drawing, symbols, Greek letters
- **ANSI escape sequence support**:
  - SGR color codes (30-37, 40-47, 90-97, 100-107)
  - Bold/bright (`ESC[1m`), dim (`ESC[2m`), blink (`ESC[5m`), reverse video (`ESC[7m`)
  - Reset (`ESC[0m`)
  - Clear screen (`ESC[2J`, `ESC[3J`) - emits three newlines
  - Cursor forward (`ESC[C`, `ESC[nC`) - emits n space characters
  - Save/restore cursor position (`ESC[s`/`ESC[u` and `ESC7`/`ESC8`) - collapses text between
- **BBS color code support** (optional, via `ConvertOptions`):
  - **Synchronet Ctrl-A codes**: `^Ar` (red), `^AR` (bright red), `^A1` (blue background), `^AH` (high intensity), `^AN` (reset)
  - **Renegade pipe codes**: `|00`-`|07` (foreground), `|08`-`|15` (bright), `|16`-`|23` (background)
- **Soft line wrapping**: Lines with ANSI sequences wrap at column 80
- **UTF-8 input mode** (optional, via `ConvertOptions`): Treats input as UTF-8 instead of CP437, only converting control characters (< 0x20)
- **SAUCE metadata handling**: Parses SAUCE/COMNT records and displays metadata as `Key: Value` lines (Title, Author, Group, Date, Size, Font, Comment). Content after SAUCE records continues to be processed.
- **HTML output**: Results wrapped in `<pre class="ansi">` with `<ans-kf>` custom elements (lowercase hex)

#### Public API

```rust
use ansi_to_html_rs::{convert, convert_with_options, ConvertOptions, generate_css, generate_js};

// Standard ANSI conversion
let html = convert(ansi_bytes);

// With BBS color codes enabled
let options = ConvertOptions {
    synchronet_ctrl_a: true,
    renegade_pipe: true,
    utf8_input: false,
};
let html = convert_with_options(bbs_bytes, &options);

// With UTF-8 input mode (skip CP437, only convert control chars)
let utf8_options = ConvertOptions {
    utf8_input: true,
    ..Default::default()
};
let html = convert_with_options(utf8_bytes, &utf8_options);

// Generate supporting assets (for dynamic use)
let css = generate_css();
let js = generate_js();
```

### web/ (ansi-display-rs)

Axum-based web server for testing the ansi-to-html-rs library.

#### Features

- File upload form accepting `.msg`, `.ans`, and `.txt` files
- Checkbox for UTF-8 input mode (skips CP437 conversion)
- Checkboxes to enable Synchronet and Renegade color code parsing
- Dark-themed UI with live preview
- Static file serving from `wwwroot/` directory
- DOS VGA fonts included for authentic rendering

#### Running

```bash
cd projects/rust

# Default: port 3000, auto-detect wwwroot
cargo run --bin ansi-display-rs

# Custom port
cargo run --bin ansi-display-rs -- -p 8080

# Custom wwwroot directory
cargo run --bin ansi-display-rs -- -w /path/to/wwwroot
```

### Rust Testing

```bash
cd projects/rust
cargo test
```

51 tests total (45 library unit tests + 3 env module tests + 3 doc tests).

## TypeScript Implementation (projects/typescript/)

### @bbs/ansi-to-html (Library)

TypeScript/Deno library with the same conversion capabilities as the Rust implementation.

#### Usage

```typescript
import { ansiToHtml } from '@bbs/ansi-to-html';

// Convert standard ANSI art
const html = ansiToHtml('\x1b[31mRed Text\x1b[0m Normal');

// Convert with BBS color code support
const html2 = ansiToHtml('|04Red |02Green', {
  renegadePipe: true
});

// Convert UTF-8 input
const html3 = ansiToHtml('Hello 世界', { utf8Input: true });
```

### Web Application

Vite + React web application for testing the TypeScript library.

#### Running

```bash
cd projects/typescript/web
deno task dev
```

### TypeScript Testing

```bash
cd projects/typescript/lib
deno task test
```

## HTML Output Format

Both implementations generate the same HTML format:

```html
<pre class="ansi">
  <ans-07>Normal text</ans-07>
  <ans-04>Red text</ans-04>
  <ans-1f>White on blue</ans-1f>
</pre>
```

## Shared Static Assets (wwwroot/)

The `wwwroot/` directory at the project root is shared across all implementations:

- `style.css` - App styles for the upload form and layout
- `ansi-display.css` - ANSI display styles including `@font-face` for DOS VGA fonts and `pre.ansi` styling
- `ansi-display.js` - JavaScript that defines `ans-kf` web components dynamically (lowercase hex)
- `fonts/` - "Px437 IBM VGA 8x16" TTF font for authentic DOS rendering

## Escape Sequence Reference

- `\e` or `\x1B` - ESC character (0x1B)
- `\r` - Carriage return (suppressed in output)
- `\n` - Newline (preserved in output)

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

## License

MIT License - Copyright (c) 2026 BBS.land
