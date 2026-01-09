# @bbs/ansi-to-html

Convert CP437 ANSI art strings to HTML with custom web components.

## Installation

```typescript
import { ansiToHtml } from "jsr:@bbs/ansi-to-html";
```

## Usage

### Basic Conversion

```typescript
import { ansiToHtml } from "@bbs/ansi-to-html";

// Convert ANSI art to HTML
const html = ansiToHtml("\x1b[31mRed Text\x1b[0m Normal");
```

### With Options

```typescript
import { ansiToHtml, type ConvertOptions } from "@bbs/ansi-to-html";

const options: ConvertOptions = {
  synchronetCtrlA: true,  // Enable Synchronet Ctrl-A codes
  renegadePipe: true,     // Enable Renegade pipe codes
  utf8Input: false,       // Treat input as CP437 (default)
};

const html = ansiToHtml("|04Red |02Green", options);
```

### UTF-8 Input Mode

```typescript
const html = ansiToHtml("Hello 世界", { utf8Input: true });
```

### Generate Supporting CSS/JS

```typescript
// The library no longer generates CSS/JS at runtime. The web project's
// static assets (see repository `wwwroot/`) provide `style.css` and
// `ansi-display.js`. If you need a single-file browser bundle that
// exposes `ansiToHtml` as a global function, see the "Build" section below.
```

## API Reference

### `ansiToHtml(input: string, options?: ConvertOptions): string`

Converts a CP437 encoded string with ANSI/BBS escape sequences to HTML.

**Parameters:**
- `input` - The input string where each character's charCode represents the CP437 byte value (0-255). For UTF-8 input, set `options.utf8Input` to `true`.
- `options` - Optional conversion options

**Returns:** HTML string with custom `<ans-KF>` elements wrapped in `<pre class="ansi">`.

### `ConvertOptions`

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `synchronetCtrlA` | `boolean` | `false` | Enable Synchronet Ctrl-A color codes |
| `renegadePipe` | `boolean` | `false` | Enable Renegade BBS pipe codes (`\|00`-`\|23`) |
| `utf8Input` | `boolean` | `false` | Treat input as UTF-8 instead of CP437 |

### `generateCss(): string`

Returns CSS for styling the `ans-KF` web components with all 256 color combinations.

### `generateJs(): string`

Returns JavaScript code that defines custom elements for all 256 `ans-KF` color combinations.

### Other Exports

- `CGA_COLORS: string[]` - Array of 16 CGA color hex values
- `colorToHex(color: number): string` - Convert color code (0-15) to hex char
- `CP437_TO_UNICODE: string[]` - CP437 to Unicode mapping table

## HTML Output Format

The converter outputs HTML wrapped in a `<pre class="ansi">` element with custom `<ans-KF>` elements:

```html
<pre class="ansi">
  <ans-07>Normal text</ans-07>
  <ans-04>Red text</ans-04>
  <ans-1f>White on blue</ans-1f>
</pre>
```

Where `K` is the background color and `F` is the foreground color (lowercase hex 0-f).

## Features

### ANSI Escape Sequences

- SGR color codes (30-37, 40-47, 90-97, 100-107)
- Bold/bright, dim, blink, reverse video
- Reset, clear screen, cursor forward
- Save/restore cursor position (collapses text between)

### BBS Color Codes

**Synchronet Ctrl-A codes** (when `synchronetCtrlA: true`):
- `^Ak`-`^Aw` - Foreground colors (black, blue, green, cyan, red, magenta, yellow, white)
- `^AK`-`^AW` - Bright foreground colors
- `^A0`-`^A7` - Background colors
- `^AH` - High intensity, `^AN` - Normal (reset)

**Renegade pipe codes** (when `renegadePipe: true`):
- `|00`-`|07` - Normal foreground colors
- `|08`-`|15` - Bright foreground colors
- `|16`-`|23` - Background colors

### Other Features

- CP437 to Unicode conversion (all 256 characters)
- Soft line wrapping at column 80 for ANSI content
- HTML escaping for special characters (`<`, `>`, `&`, `"`)
- Carriage return suppression, newline preservation

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

## Development

```bash
deno task test    # Run tests (32 tests)
deno task check   # Type check
deno task fmt     # Format code
deno task lint    # Lint code
```

## Build (single-file ES3 bundle)

This repository includes a small Rollup-based build in `ansi-to-html-ts/lib/`.
It compiles the library to a single ES3-compatible IIFE file and attaches a
global `ansiToHtml` function. To build:

```bash
cd ansi-to-html-ts/lib
npm install
npm run build
# output: dist/ansi-to-html.es3.iife.js
```

Include the generated file in a page to get a global `ansiToHtml` function.


## License

MIT License - see [LICENSE.md](../../LICENSE.md)
