/**
 * ansi-to-html-ts - Convert CP437 ANSI art strings to HTML with custom web components
 *
 * This library converts strings representing CP437 encoded text with optional
 * ANSI escape sequences into HTML fragments using custom <ans-KF> web components.
 *
 * @packageDocumentation
 */

export { ansiToHtml, type ConvertOptions } from './converter';
export { CGA_COLORS, colorToHex } from './colors';
export { CP437_TO_UNICODE } from './cp437';

// NOTE: CSS and JavaScript for the `ans-*` web components are provided
// as static assets in the repository `wwwroot/` and should be consumed
// by web projects via the public assets (e.g. `/style.css` and
// `/ansi-display.js`). The generator functions were removed so the
// library does not emit or inject runtime styles/scripts.
