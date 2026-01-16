# ansi-to-html-ts

TypeScript implementation for converting CP437 ANSI art to HTML with custom web components.

## Overview

This directory contains two packages:

- **lib/** - The `@bbs/ansi-to-html` library for converting ANSI art to HTML
- **web/** - A React + Vite web application for testing the library

Both packages use Deno as the runtime.

## Quick Start

### Library

```bash
cd lib
deno task test    # Run tests
deno task check   # Type check
deno task fmt     # Format code
deno task lint    # Lint code
```

### Web Application

```bash
cd web
deno task dev     # Start dev server (default: http://localhost:3100)
deno task build   # Build for production
deno task preview # Preview production build
```

## Web Application Features

- File upload form accepting `.msg`, `.ans`, and `.txt` files
- Checkbox for UTF-8 input mode (skips CP437 conversion)
- Checkboxes to enable Synchronet and Renegade color code parsing
- Dark-themed UI with live preview
- Same wwwroot lookup logic as the Rust version

## Configuration

### Environment Variables

- `HTTP_PORT` or `PORT` - Server port (default: 3100)
- `WWWROOT` - Static files directory

### wwwroot Lookup Order

1. `WWWROOT` environment variable (resolved relative to cwd)
2. Search from cwd upward (up to 3 directories) for `wwwroot/`
3. Search from config file directory upward for `wwwroot/`
4. Fallback to `/var/www/html`

## Project Structure

```
ansi-to-html-ts/
├── lib/                    # @bbs/ansi-to-html library
│   ├── deno.json
│   ├── README.md
│   └── src/
│       ├── index.ts        # Main exports
│       ├── converter.ts    # ansiToHtml() function
│       ├── cp437.ts        # CP437 to Unicode mapping
│       ├── colors.ts       # CGA color definitions
│       ├── css.ts          # generateCss() function
│       ├── js.ts           # generateJs() function
│       └── converter_test.ts
│
└── web/                    # React web application
    ├── deno.json
    ├── vite.config.ts
    ├── index.html
    └── src/
        ├── main.tsx
        ├── App.tsx
        ├── index.css
        └── components/
            └── AnsiPreview.tsx
```

## See Also

- [Library README](lib/README.md) - Detailed library documentation
- [Main Project README](../README.md) - Full project documentation
