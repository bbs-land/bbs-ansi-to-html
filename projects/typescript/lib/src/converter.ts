/**
 * ANSI to HTML converter
 *
 * Converts CP437 encoded strings with ANSI/BBS escape sequences to HTML
 * using custom <ans-KF> web components.
 */

import { CP437_TO_UNICODE } from './cp437.ts';
import { colorToHex } from './colors.ts';

/**
 * Options for controlling conversion behavior.
 */
export interface ConvertOptions {
  /** Enable Synchronet Ctrl-A color codes (Ctrl-A + character) */
  synchronetCtrlA?: boolean;
  /** Enable Renegade BBS pipe codes (|00 through |23) */
  renegadePipe?: boolean;
  /** Treat input as UTF-8 instead of CP437 (only convert control chars < 0x20) */
  utf8Input?: boolean;
}

/**
 * SAUCE record data (Standard Architecture for Universal Comment Extensions)
 */
interface SauceRecord {
  title: string;
  author: string;
  group: string;
  date: string;
  width: number;
  height: number;
  comments: string[];
  font: string;
}

/**
 * Decode a CP437 byte field to string, trimming trailing spaces/nulls
 */
function decodeField(input: string, start: number, length: number): string {
  let result = '';
  for (let i = start; i < start + length && i < input.length; i++) {
    const code = input.charCodeAt(i);
    result += CP437_TO_UNICODE[code] ?? String.fromCharCode(code);
  }
  return result.replace(/[\s\0]+$/, '');
}

/**
 * Parse SAUCE record from input string starting at given position
 */
function parseSauceRecord(input: string, saucePos: number, comntPos: number | null): SauceRecord | null {
  if (saucePos + 128 > input.length) return null;

  const id = input.slice(saucePos, saucePos + 5);
  if (id !== 'SAUCE') return null;

  const record: SauceRecord = {
    title: decodeField(input, saucePos + 7, 35),
    author: decodeField(input, saucePos + 42, 20),
    group: decodeField(input, saucePos + 62, 20),
    date: decodeField(input, saucePos + 82, 8),
    width: input.charCodeAt(saucePos + 96) | (input.charCodeAt(saucePos + 97) << 8),
    height: input.charCodeAt(saucePos + 98) | (input.charCodeAt(saucePos + 99) << 8),
    comments: [],
    font: decodeField(input, saucePos + 106, 22),
  };

  // Parse COMNT block if present
  if (comntPos !== null && comntPos < saucePos) {
    const comntId = input.slice(comntPos, comntPos + 5);
    if (comntId === 'COMNT') {
      const commentData = input.slice(comntPos + 5, saucePos);
      // Each comment line is 64 characters
      for (let i = 0; i < commentData.length; i += 64) {
        const line = decodeField(commentData, i, 64);
        if (line) {
          record.comments.push(line);
        }
      }
    }
  }

  return record;
}

/**
 * Format SAUCE record as "Key: Value\n" lines
 */
function formatSauceOutput(sauce: SauceRecord): string {
  let output = '';

  if (sauce.title) {
    output += `Title: ${sauce.title}\n`;
  }
  if (sauce.author) {
    output += `Author: ${sauce.author}\n`;
  }
  if (sauce.group) {
    output += `Group: ${sauce.group}\n`;
  }
  if (sauce.date) {
    // Format date from CCYYMMDD to CCYY-MM-DD if valid
    if (sauce.date.length === 8 && /^\d{8}$/.test(sauce.date)) {
      output += `Date: ${sauce.date.slice(0, 4)}-${sauce.date.slice(4, 6)}-${sauce.date.slice(6, 8)}\n`;
    } else {
      output += `Date: ${sauce.date}\n`;
    }
  }
  if (sauce.width > 0 || sauce.height > 0) {
    output += `Size: ${sauce.width}x${sauce.height}\n`;
  }
  if (sauce.font) {
    output += `Font: ${sauce.font}\n`;
  }
  for (const comment of sauce.comments) {
    output += `Comment: ${comment}\n`;
  }

  return output;
}

/**
 * Find SAUCE record position and COMNT block in input
 * Returns [saucePos, comntPos, afterSaucePos] or [null, null, null]
 */
function findSaucePositions(input: string): [number | null, number | null, number | null] {
  // Search for SAUCE00 marker
  const saucePos = input.lastIndexOf('SAUCE00');
  if (saucePos === -1) {
    return [null, null, null];
  }

  // Look for COMNT before SAUCE
  const searchStart = Math.max(0, saucePos - (64 * 256 + 5));
  const searchArea = input.slice(searchStart, saucePos);
  const comntRelPos = searchArea.lastIndexOf('COMNT');
  const comntPos = comntRelPos !== -1 ? searchStart + comntRelPos : null;

  // Position after SAUCE record
  const afterSaucePos = saucePos + 128;

  return [saucePos, comntPos, afterSaucePos];
}

/**
 * Parser state for ANSI escape sequences.
 */
const enum ParseState {
  Normal,
  Escape,
  Csi,
  SynchronetCtrlA,
  RenegadePipe1,
  RenegadePipe2,
}

/**
 * Map ANSI color code (0-7) to CGA color code.
 */
function ansiToCga(ansiColor: number): number {
  switch (ansiColor) {
    case 0: return 0; // Black
    case 1: return 4; // Red
    case 2: return 2; // Green
    case 3: return 6; // Brown/Yellow
    case 4: return 1; // Blue
    case 5: return 5; // Magenta
    case 6: return 3; // Cyan
    case 7: return 7; // White/Light Gray
    default: return 7;
  }
}

/**
 * Map bright ANSI color code (0-7) to CGA bright color code.
 */
function ansiBrightToCga(ansiColor: number): number {
  switch (ansiColor) {
    case 0: return 8;  // Dark Gray
    case 1: return 12; // Light Red
    case 2: return 10; // Light Green
    case 3: return 14; // Yellow
    case 4: return 9;  // Light Blue
    case 5: return 13; // Light Magenta
    case 6: return 11; // Light Cyan
    case 7: return 15; // White
    default: return 15;
  }
}

/**
 * Converter class that maintains state during conversion.
 */
class Converter {
  private foreground = 7;  // Light Gray
  private background = 0;  // Black
  private output = '';
  private currentColumn = 0;
  private lineHasAnsi = false;
  private savePositionActive = false;
  private parseState: ParseState = ParseState.Normal;
  private csiParams = '';
  private renegadeFirstDigit = 0;
  private options: Required<ConvertOptions>;

  constructor(options: ConvertOptions = {}) {
    this.options = {
      synchronetCtrlA: options.synchronetCtrlA ?? false,
      renegadePipe: options.renegadePipe ?? false,
      utf8Input: options.utf8Input ?? false,
    };
  }

  private openTag(): void {
    const bg = colorToHex(this.background);
    const fg = colorToHex(this.foreground);
    this.output += `<ans-${bg}${fg}>`;
  }

  private closeTag(): void {
    const bg = colorToHex(this.background);
    const fg = colorToHex(this.foreground);
    this.output += `</ans-${bg}${fg}>`;
  }

  private switchColor(newBg: number, newFg: number): void {
    if (newBg !== this.background || newFg !== this.foreground) {
      this.closeTag();
      this.background = newBg;
      this.foreground = newFg;
      this.openTag();
    }
  }

  private emitChar(ch: string): void {
    if (this.savePositionActive) {
      return;
    }

    // Check for soft return at column 80
    if (this.lineHasAnsi && this.currentColumn >= 80 && ch !== '\n') {
      this.output += '\n';
      this.currentColumn = 0;
    }

    switch (ch) {
      case '<':
        this.output += '&lt;';
        this.currentColumn++;
        break;
      case '>':
        this.output += '&gt;';
        this.currentColumn++;
        break;
      case '&':
        this.output += '&amp;';
        this.currentColumn++;
        break;
      case '"':
        this.output += '&quot;';
        this.currentColumn++;
        break;
      case "'":
        this.output += '&apos;';
        this.currentColumn++;
        break;
      case '\n':
        this.output += '\n';
        this.currentColumn = 0;
        this.lineHasAnsi = false;
        break;
      case '\r':
        // Suppress carriage returns
        break;
      default:
        this.output += ch;
        this.currentColumn++;
        break;
    }
  }

  private processSgr(params: string): void {
    const codes = params === '' ? [0] : params.split(';').map(s => parseInt(s, 10) || 0);

    let newFg = this.foreground;
    let newBg = this.background;

    for (const code of codes) {
      switch (code) {
        case 0:
          // Reset
          newFg = 7;
          newBg = 0;
          break;
        case 1:
          // Bold/Bright - set high bit on foreground
          newFg |= 0x08;
          break;
        case 2:
        case 22:
          // Dim / Normal intensity - clear high bit
          newFg &= 0x07;
          break;
        case 5:
        case 6:
          // Blink - set high bit on background (CGA style)
          newBg |= 0x08;
          break;
        case 25:
          // Blink off
          newBg &= 0x07;
          break;
        case 7:
          // Reverse video
          [newFg, newBg] = [newBg, newFg];
          break;
        case 30: case 31: case 32: case 33:
        case 34: case 35: case 36: case 37:
          // Standard foreground colors
          newFg = (newFg & 0x08) | ansiToCga(code - 30);
          break;
        case 39:
          // Default foreground
          newFg = 7;
          break;
        case 40: case 41: case 42: case 43:
        case 44: case 45: case 46: case 47:
          // Standard background colors
          newBg = (newBg & 0x08) | ansiToCga(code - 40);
          break;
        case 49:
          // Default background
          newBg = 0;
          break;
        case 90: case 91: case 92: case 93:
        case 94: case 95: case 96: case 97:
          // Bright foreground colors
          newFg = ansiBrightToCga(code - 90);
          break;
        case 100: case 101: case 102: case 103:
        case 104: case 105: case 106: case 107:
          // Bright background colors
          newBg = ansiBrightToCga(code - 100);
          break;
      }
    }

    this.switchColor(newBg, newFg);
  }

  private processCsi(params: string, command: string): void {
    this.lineHasAnsi = true;

    switch (command) {
      case 'm':
        // SGR - Select Graphic Rendition
        this.processSgr(params);
        break;
      case 'J': {
        // ED - Erase Display
        const n = parseInt(params, 10) || 0;
        if (n === 2 || n === 3) {
          // Clear screen - inject three line feeds
          this.emitChar('\n');
          this.emitChar('\n');
          this.emitChar('\n');
        }
        break;
      }
      case 's':
        // SCP - Save Cursor Position
        this.savePositionActive = true;
        break;
      case 'u':
        // RCP - Restore Cursor Position
        this.savePositionActive = false;
        break;
      case 'H':
      case 'f':
        // CUP - Cursor Position (often used with clear screen)
        break;
      case 'C': {
        // CUF - Cursor Forward: emit n space characters (default 1)
        const n = Math.max(1, parseInt(params, 10) || 1);
        for (let i = 0; i < n; i++) {
          this.emitChar(' ');
        }
        break;
      }
      case 'A':
      case 'B':
      case 'D':
        // Other cursor movement (up, down, back) - ignored for static conversion
        break;
      case 'K':
        // EL - Erase in Line - ignored
        break;
    }
  }

  private processSynchronetCode(code: number): void {
    this.lineHasAnsi = true;
    let newFg = this.foreground;
    let newBg = this.background;

    // Using character codes for comparison
    switch (code) {
      // Lowercase = normal intensity foreground
      case 0x6B: newFg = 0; break;  // 'k' - Black
      case 0x62: newFg = 1; break;  // 'b' - Blue
      case 0x67: newFg = 2; break;  // 'g' - Green
      case 0x63: newFg = 3; break;  // 'c' - Cyan
      case 0x72: newFg = 4; break;  // 'r' - Red
      case 0x6D: newFg = 5; break;  // 'm' - Magenta
      case 0x79: newFg = 6; break;  // 'y' - Brown/Yellow
      case 0x77: newFg = 7; break;  // 'w' - White/Light Gray

      // Uppercase = high intensity foreground
      case 0x4B: newFg = 8; break;  // 'K' - Dark Gray
      case 0x42: newFg = 9; break;  // 'B' - Light Blue
      case 0x47: newFg = 10; break; // 'G' - Light Green
      case 0x43: newFg = 11; break; // 'C' - Light Cyan
      case 0x52: newFg = 12; break; // 'R' - Light Red
      case 0x4D: newFg = 13; break; // 'M' - Light Magenta
      case 0x59: newFg = 14; break; // 'Y' - Yellow
      case 0x57: newFg = 15; break; // 'W' - White

      // Background colors (0-7)
      case 0x30: newBg = 0; break;  // '0' - Black
      case 0x31: newBg = 1; break;  // '1' - Blue
      case 0x32: newBg = 2; break;  // '2' - Green
      case 0x33: newBg = 3; break;  // '3' - Cyan
      case 0x34: newBg = 4; break;  // '4' - Red
      case 0x35: newBg = 5; break;  // '5' - Magenta
      case 0x36: newBg = 6; break;  // '6' - Brown
      case 0x37: newBg = 7; break;  // '7' - White/Light Gray

      // Special codes
      case 0x48: // 'H'
      case 0x68: // 'h'
        newFg |= 0x08; // High intensity
        break;
      case 0x49: // 'I'
      case 0x69: // 'i'
        newBg |= 0x08; // Blink/high intensity background
        break;
      case 0x4E: // 'N'
      case 0x6E: // 'n'
        // Normal - reset to default
        newFg = 7;
        newBg = 0;
        break;
      case 0x2D: // '-'
        newFg &= 0x07; // Remove high intensity from foreground
        break;
      case 0x5F: // '_'
        newBg &= 0x07; // Remove blink from background
        break;
    }

    this.switchColor(newBg, newFg);
  }

  private processRenegadeCode(code: number): void {
    this.lineHasAnsi = true;
    let newFg = this.foreground;
    let newBg = this.background;

    if (code >= 0 && code <= 15) {
      // Foreground colors 0-15
      newFg = code;
    } else if (code >= 16 && code <= 23) {
      // Background colors 16-23
      newBg = code - 16;
    }

    this.switchColor(newBg, newFg);
  }

  private processCharCode(charCode: number): void {
    switch (this.parseState) {
      case ParseState.Normal:
        if (charCode === 0x1B) {
          this.parseState = ParseState.Escape;
        } else if (this.options.synchronetCtrlA && charCode === 0x01) {
          this.parseState = ParseState.SynchronetCtrlA;
        } else if (this.options.renegadePipe && charCode === 0x7C) { // '|'
          this.parseState = ParseState.RenegadePipe1;
        } else if (charCode === 0x0A) { // '\n'
          this.emitChar('\n');
        } else if (charCode === 0x0D) { // '\r'
          // Suppress carriage returns
        } else if (charCode < 0x20 || charCode >= 0x7F) {
          // Convert CP437 to Unicode
          const unicodeChar = CP437_TO_UNICODE[charCode] ?? '\uFFFD';
          this.emitChar(unicodeChar);
        } else {
          this.emitChar(String.fromCharCode(charCode));
        }
        break;

      case ParseState.Escape:
        if (charCode === 0x5B) { // '['
          this.parseState = ParseState.Csi;
          this.csiParams = '';
        } else if (charCode === 0x37) { // '7' - Save cursor position (DEC)
          this.savePositionActive = true;
          this.lineHasAnsi = true;
          this.parseState = ParseState.Normal;
        } else if (charCode === 0x38) { // '8' - Restore cursor position (DEC)
          this.savePositionActive = false;
          this.lineHasAnsi = true;
          this.parseState = ParseState.Normal;
        } else {
          this.parseState = ParseState.Normal;
        }
        break;

      case ParseState.Csi:
        if ((charCode >= 0x30 && charCode <= 0x39) || charCode === 0x3B) {
          // Digit or semicolon
          this.csiParams += String.fromCharCode(charCode);
        } else if (charCode >= 0x40 && charCode <= 0x7E) {
          // Final byte of CSI sequence
          this.processCsi(this.csiParams, String.fromCharCode(charCode));
          this.parseState = ParseState.Normal;
        } else {
          this.parseState = ParseState.Normal;
        }
        break;

      case ParseState.SynchronetCtrlA:
        this.processSynchronetCode(charCode);
        this.parseState = ParseState.Normal;
        break;

      case ParseState.RenegadePipe1:
        if (charCode >= 0x30 && charCode <= 0x39) {
          this.renegadeFirstDigit = charCode - 0x30;
          this.parseState = ParseState.RenegadePipe2;
        } else {
          this.emitChar('|');
          this.parseState = ParseState.Normal;
          this.processCharCode(charCode);
        }
        break;

      case ParseState.RenegadePipe2:
        if (charCode >= 0x30 && charCode <= 0x39) {
          const code = this.renegadeFirstDigit * 10 + (charCode - 0x30);
          if (code <= 23) {
            this.processRenegadeCode(code);
          }
          // If code > 23, just ignore the sequence
        } else {
          this.emitChar('|');
          this.emitChar(String.fromCharCode(0x30 + this.renegadeFirstDigit));
          this.parseState = ParseState.Normal;
          this.processCharCode(charCode);
          return;
        }
        this.parseState = ParseState.Normal;
        break;
    }
  }

  private processUtf8Char(ch: string): void {
    const code = ch.charCodeAt(0);

    switch (this.parseState) {
      case ParseState.Normal:
        if (code === 0x1B) {
          this.parseState = ParseState.Escape;
        } else if (this.options.synchronetCtrlA && code === 0x01) {
          this.parseState = ParseState.SynchronetCtrlA;
        } else if (this.options.renegadePipe && ch === '|') {
          this.parseState = ParseState.RenegadePipe1;
        } else if (ch === '\n') {
          this.emitChar('\n');
        } else if (ch === '\r') {
          // Suppress carriage returns
        } else if (code < 0x20) {
          // Convert low-byte control characters using CP437 mapping
          const unicodeChar = CP437_TO_UNICODE[code] ?? '\uFFFD';
          this.emitChar(unicodeChar);
        } else {
          // Pass through all other UTF-8 characters as-is
          this.emitChar(ch);
        }
        break;

      case ParseState.Escape:
        if (ch === '[') {
          this.parseState = ParseState.Csi;
          this.csiParams = '';
        } else if (ch === '7') {
          this.savePositionActive = true;
          this.lineHasAnsi = true;
          this.parseState = ParseState.Normal;
        } else if (ch === '8') {
          this.savePositionActive = false;
          this.lineHasAnsi = true;
          this.parseState = ParseState.Normal;
        } else {
          this.parseState = ParseState.Normal;
        }
        break;

      case ParseState.Csi:
        if ((code >= 0x30 && code <= 0x39) || ch === ';') {
          this.csiParams += ch;
        } else if (code >= 0x40 && code <= 0x7E) {
          this.processCsi(this.csiParams, ch);
          this.parseState = ParseState.Normal;
        } else {
          this.parseState = ParseState.Normal;
        }
        break;

      case ParseState.SynchronetCtrlA:
        if (code <= 0xFF) {
          this.processSynchronetCode(code);
        }
        this.parseState = ParseState.Normal;
        break;

      case ParseState.RenegadePipe1:
        if (code >= 0x30 && code <= 0x39) {
          this.renegadeFirstDigit = code - 0x30;
          this.parseState = ParseState.RenegadePipe2;
        } else {
          this.emitChar('|');
          this.parseState = ParseState.Normal;
          this.processUtf8Char(ch);
        }
        break;

      case ParseState.RenegadePipe2:
        if (code >= 0x30 && code <= 0x39) {
          const pipeCode = this.renegadeFirstDigit * 10 + (code - 0x30);
          if (pipeCode <= 23) {
            this.processRenegadeCode(pipeCode);
          }
        } else {
          this.emitChar('|');
          this.emitChar(String.fromCharCode(0x30 + this.renegadeFirstDigit));
          this.parseState = ParseState.Normal;
          this.processUtf8Char(ch);
          return;
        }
        this.parseState = ParseState.Normal;
        break;
    }
  }

  /**
   * Convert input string to HTML.
   * Input is treated as CP437 where each character's charCode is the byte value.
   */
  convert(input: string): string {
    this.output = '<pre class="ansi">';
    this.openTag();

    // Find SUB marker and SAUCE positions
    let subPos = -1;
    for (let i = 0; i < input.length; i++) {
      if (input.charCodeAt(i) === 0x1A) {
        subPos = i;
        break;
      }
    }
    const [saucePos, comntPos, afterSaucePos] = findSaucePositions(input);

    // Determine content end position
    const contentEnd = subPos !== -1 ? subPos :
                       comntPos !== null ? comntPos :
                       saucePos !== null ? saucePos :
                       input.length;

    // Process content before SUB/SAUCE
    for (let i = 0; i < contentEnd; i++) {
      this.processCharCode(input.charCodeAt(i));
    }

    // If SAUCE record exists, parse and output it
    if (saucePos !== null) {
      const sauce = parseSauceRecord(input, saucePos, comntPos);
      if (sauce) {
        const sauceOutput = formatSauceOutput(sauce);
        if (sauceOutput) {
          // Add newline before SAUCE metadata
          this.emitChar('\n');
          for (const ch of sauceOutput) {
            this.emitChar(ch);
          }
        }
      }

      // Check for content after SAUCE record
      if (afterSaucePos !== null && afterSaucePos < input.length) {
        const remaining = input.slice(afterSaucePos);
        // Check if there's meaningful content (not just nulls/SUBs)
        let hasMeaningfulContent = false;
        for (let i = 0; i < remaining.length; i++) {
          const code = remaining.charCodeAt(i);
          if (code !== 0 && code !== 0x1A) {
            hasMeaningfulContent = true;
            break;
          }
        }

        if (hasMeaningfulContent) {
          // Add newline separator before continuing content
          this.emitChar('\n');
          for (let i = 0; i < remaining.length; i++) {
            const charCode = remaining.charCodeAt(i);
            if (charCode === 0x1A) {
              // Another SUB - stop processing
              break;
            }
            this.processCharCode(charCode);
          }
        }
      }
    }

    this.closeTag();
    this.output += '</pre>';

    return this.output;
  }

  /**
   * Convert UTF-8 input string to HTML.
   * Only control characters < 0x20 are converted using CP437 mapping.
   */
  convertUtf8(input: string): string {
    this.output = '<pre class="ansi">';
    this.openTag();

    // Find SUB marker and SAUCE positions
    const subPos = input.indexOf('\x1A');
    const [saucePos, comntPos, afterSaucePos] = findSaucePositions(input);

    // Determine content end position
    const contentEnd = subPos !== -1 ? subPos :
                       comntPos !== null ? comntPos :
                       saucePos !== null ? saucePos :
                       input.length;

    // Process content before SUB/SAUCE
    for (const ch of input.slice(0, contentEnd)) {
      this.processUtf8Char(ch);
    }

    // If SAUCE record exists, parse and output it
    if (saucePos !== null) {
      const sauce = parseSauceRecord(input, saucePos, comntPos);
      if (sauce) {
        const sauceOutput = formatSauceOutput(sauce);
        if (sauceOutput) {
          // Add newline before SAUCE metadata
          this.emitChar('\n');
          for (const ch of sauceOutput) {
            this.emitChar(ch);
          }
        }
      }

      // Check for content after SAUCE record
      if (afterSaucePos !== null && afterSaucePos < input.length) {
        const remaining = input.slice(afterSaucePos);
        // Check if there's meaningful content (not just nulls/SUBs)
        let hasMeaningfulContent = false;
        for (const ch of remaining) {
          const code = ch.charCodeAt(0);
          if (code !== 0 && code !== 0x1A) {
            hasMeaningfulContent = true;
            break;
          }
        }

        if (hasMeaningfulContent) {
          // Add newline separator before continuing content
          this.emitChar('\n');
          for (const ch of remaining) {
            if (ch === '\x1A') {
              break;
            }
            this.processUtf8Char(ch);
          }
        }
      }
    }

    this.closeTag();
    this.output += '</pre>';

    return this.output;
  }
}

/**
 * Convert a CP437 encoded string with ANSI/BBS escape sequences to HTML.
 *
 * The input string should have each character's charCode representing the
 * CP437 byte value (0-255). For UTF-8 input, set options.utf8Input to true.
 *
 * @param input - The input string to convert
 * @param options - Optional conversion options
 * @returns HTML string with custom <ans-KF> elements
 *
 * @example
 * ```ts
 * import { ansiToHtml } from 'ansi-to-html-ts';
 *
 * // Convert standard ANSI art
 * const html = ansiToHtml('\x1b[31mRed Text\x1b[0m Normal');
 *
 * // Convert with BBS color code support
 * const html2 = ansiToHtml('|04Red |02Green', {
 *   renegadePipe: true
 * });
 *
 * // Convert UTF-8 input
 * const html3 = ansiToHtml('Hello 世界', { utf8Input: true });
 * ```
 */
export function ansiToHtml(input: string, options: ConvertOptions = {}): string {
  const converter = new Converter(options);
  if (options.utf8Input) {
    return converter.convertUtf8(input);
  }
  return converter.convert(input);
}
