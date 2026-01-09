# bbs-ansi-to-html

Convert CP437 ANSI art files to HTML with custom web components.

**[Live Demo](https://bbs-land.github.io/bbs-ansi-to-html/)** (TypeScript Version)

## Overview

This mono-repo provides tools for converting BBS-style ANSI art files (with Code Page 437 encoding) into HTML that can be displayed in modern web browsers. It supports standard ANSI escape sequences as well as BBS-specific color codes from Synchronet and Renegade BBS software.

Available in two implementations:
- **[Rust](projects/rust/)** - High-performance library and Axum-based web server
- **[TypeScript](projects/typescript/)** - Deno/TypeScript library with Vite + React web app

## Features

- **CP437 to Unicode conversion** - All 256 characters including box drawing, symbols, Greek letters
- **ANSI escape sequences** - SGR color codes, bold/bright, dim, blink, reverse video, cursor movement
- **BBS color codes** (optional):
  - Synchronet Ctrl-A codes
  - Renegade pipe codes
- **Soft line wrapping** at column 80 for ANSI content
- **UTF-8 input mode** - Skip CP437 conversion for modern text
- **Custom web components** - Output uses `<ans-kf>` elements for styling

## Quick Start

### Rust

```bash
cd projects/rust
cargo run --bin ansi-display-rs
```

Then open http://127.0.0.1:3000 in your browser.

### TypeScript

```bash
cd projects/typescript/web
deno task dev
```

Then open http://localhost:5173 in your browser.

## Project Structure

```
ansi-to-html/
├── projects/
│   ├── rust/               # Rust implementation
│   │   ├── lib/            # Core library crate (ansi-to-html-rs)
│   │   └── web/            # Axum web server (ansi-display-rs)
│   └── typescript/         # TypeScript implementation
│       ├── lib/            # Core library (@bbs/ansi-to-html)
│       └── web/            # Vite + React web app
├── wwwroot/                # Shared static assets (fonts, CSS, JS)
└── test-files/             # Sample ANSI art files
```

## HTML Output Format

Both implementations generate the same HTML format using custom web components:

```html
<pre class="ansi">
  <ans-07>Normal text</ans-07>
  <ans-04>Red text</ans-04>
  <ans-1f>White on blue</ans-1f>
</pre>
```

The tag name `ans-KF` uses lowercase hex digits where K is the background color (0-f) and F is the foreground color (0-f).

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

## Documentation

- [Rust Implementation](projects/rust/README.md) - Detailed Rust API documentation
- [TypeScript Implementation](projects/typescript/README.md) - TypeScript library usage

## License

MIT License - Copyright (c) 2026 BBS.land

See [LICENSE.md](LICENSE.md) for details.
