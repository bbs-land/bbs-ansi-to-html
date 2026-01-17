import { assertEquals, assertStringIncludes } from 'jsr:@std/assert';
import { ansiToHtml, type ConvertOptions } from './converter.ts';

Deno.test('ansiToHtml - converts basic text', () => {
  const result = ansiToHtml('Hello');
  assertStringIncludes(result, '<pre class="ansi">');
  assertStringIncludes(result, '<ans-07>');
  assertStringIncludes(result, 'Hello');
  assertStringIncludes(result, '</ans-07>');
  assertStringIncludes(result, '</pre>');
});

Deno.test('ansiToHtml - escapes HTML special characters', () => {
  const result = ansiToHtml('<script>&</script>');
  assertStringIncludes(result, '&lt;script&gt;&amp;&lt;/script&gt;');
  // Test double quote
  const result2 = ansiToHtml('"quoted"');
  assertStringIncludes(result2, '&quot;quoted&quot;');
  // Test apostrophe
  const result3 = ansiToHtml("it's here");
  assertStringIncludes(result3, "it&apos;s here");
});

Deno.test('ansiToHtml - handles color changes', () => {
  // ESC[31m sets red foreground
  const input = '\x1b[31mRed';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-04>'); // Red foreground on black
});

Deno.test('ansiToHtml - handles clear screen', () => {
  // ESC[2J clears screen
  const input = 'Before\x1b[2JAfter';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '\n\n\n');
});

Deno.test('ansiToHtml - preserves newlines', () => {
  const result = ansiToHtml('Line1\nLine2');
  assertStringIncludes(result, 'Line1\nLine2');
});

Deno.test('ansiToHtml - suppresses carriage returns', () => {
  const result = ansiToHtml('Line1\r\nLine2');
  assertEquals(result.includes('\r'), false);
  assertStringIncludes(result, 'Line1\nLine2');
});

Deno.test('ansiToHtml - handles bright foreground colors', () => {
  // ESC[91m = bright red
  const input = '\x1b[91mBright Red';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-0c>'); // Light Red on black
});

Deno.test('ansiToHtml - handles bold making bright', () => {
  // ESC[1m makes foreground bright, ESC[34m blue -> light blue
  const input = '\x1b[1;34mBold Blue';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-09>'); // Light Blue (9) on black
});

Deno.test('ansiToHtml - handles reset colors', () => {
  // ESC[31m red, then ESC[0m reset
  const input = '\x1b[31mRed\x1b[0mNormal';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-04>Red</ans-04>');
  assertStringIncludes(result, '<ans-07>Normal');
});

Deno.test('ansiToHtml - handles cursor forward', () => {
  // ESC[5C moves cursor forward 5 positions (emits 5 spaces)
  const input = 'A\x1b[5CB';
  const result = ansiToHtml(input);
  assertStringIncludes(result, 'A     B');
});

Deno.test('ansiToHtml - handles save/restore position collapse', () => {
  // ESC[s saves position, text should be collapsed, ESC[u restores
  const input = 'Before\x1b[sHidden\x1b[uAfter';
  const result = ansiToHtml(input);
  assertStringIncludes(result, 'Before');
  assertStringIncludes(result, 'After');
  assertEquals(result.includes('Hidden'), false);
});

Deno.test('ansiToHtml - converts CP437 box drawing characters', () => {
  // 0xDA = top-left corner, 0xC4 = horizontal, 0xBF = top-right corner
  const input = String.fromCharCode(0xDA, 0xC4, 0xC4, 0xBF);
  const result = ansiToHtml(input);
  assertStringIncludes(result, '┌');
  assertStringIncludes(result, '─');
  assertStringIncludes(result, '┐');
});

Deno.test('ansiToHtml - converts CP437 special characters', () => {
  // 0x01 = smiley, 0x02 = black smiley, 0x03 = heart
  const input = String.fromCharCode(0x01, 0x02, 0x03);
  const result = ansiToHtml(input);
  assertStringIncludes(result, '☺');
  assertStringIncludes(result, '☻');
  assertStringIncludes(result, '♥');
});

Deno.test('ansiToHtml - handles soft return at column 80', () => {
  // Create a line with ANSI escape that's longer than 80 chars
  let input = '\x1b[31m'; // Red color
  for (let i = 0; i < 85; i++) {
    input += 'X';
  }
  const result = ansiToHtml(input);
  // Should have a newline injected after column 80
  const lines = result.split('\n');
  const xLine = lines.find((s) => s.includes('XXXX'));
  assertEquals(xLine !== undefined, true);
  assertEquals((xLine!.match(/X/g) || []).length, 80);
});

Deno.test('ansiToHtml - does not add soft return without ANSI', () => {
  // Without ANSI, no soft return should happen
  let input = '';
  for (let i = 0; i < 85; i++) {
    input += 'X';
  }
  const result = ansiToHtml(input);
  assertEquals(result.includes('\n'), false);
});

// Synchronet Ctrl-A codes tests
Deno.test('Synchronet - handles foreground colors (lowercase)', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + r (lowercase) = red foreground
  const input = '\x01rRed Text';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-04>'); // Red on black
});

Deno.test('Synchronet - handles background colors (uppercase)', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + R (uppercase) = red background
  const input = '\x01RRed BG';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-47>'); // Red bg (4), Light Gray fg (7)
});

Deno.test('Synchronet - handles background color (digit)', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + 1 = blue background
  const input = '\x011Blue BG';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-17>'); // Blue bg, Light Gray fg
});

Deno.test('Synchronet - handles high intensity foreground', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + b (blue fg) + Ctrl-A + h (high intensity) = bright blue
  const input = '\x01b\x01hBright Blue';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-09>'); // Light Blue on black
});

Deno.test('Synchronet - handles high intensity background', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + B (blue bg) + Ctrl-A + i (blink/high intensity bg) = bright blue bg
  const input = '\x01B\x01iBright Blue BG';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-97>'); // Light Blue bg (9), Light Gray fg (7)
});

Deno.test('Synchronet - handles normal reset', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + r (red fg) then Ctrl-A + n = reset to normal
  const input = '\x01rRed\x01nNormal';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-04>Red</ans-04>');
  assertStringIncludes(result, '<ans-07>Normal');
});

Deno.test('Synchronet - is disabled by default', () => {
  // Without option, Ctrl-A should be treated as CP437 character (smiley)
  const input = '\x01rText';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '☺'); // CP437 0x01 = smiley face
  assertStringIncludes(result, 'rText');
});

Deno.test('Synchronet - preserves intensity when changing color', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Set high intensity first, then change color - intensity should be preserved
  const input = '\x01h\x01bBright Blue';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-09>'); // Light Blue (high intensity preserved)
});

Deno.test('Synchronet - combined foreground and background', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + w (white fg) + Ctrl-A + B (blue bg)
  const input = '\x01w\x01BWhite on Blue';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-17>'); // Blue bg (1), Light Gray fg (7)
});

Deno.test('Synchronet - high intensity is idempotent', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Applying high intensity multiple times should have same effect as once
  const input = '\x01b\x01h\x01hDouble High';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-09>'); // Light Blue (9), not something weird
});

Deno.test('Synchronet - blink is idempotent', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Applying blink/high bg multiple times should have same effect as once
  const input = '\x01B\x01i\x01iDouble Blink BG';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-97>'); // Light Blue bg (9), Light Gray fg (7)
});

// Renegade pipe codes tests
Deno.test('Renegade - handles foreground colors', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // |04 = red foreground
  const input = '|04Red Text';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '<ans-04>'); // Red on black
});

Deno.test('Renegade - handles bright foreground', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // |12 = bright red (Light Red)
  const input = '|12Bright Red';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '<ans-0c>'); // Light Red on black
});

Deno.test('Renegade - handles background color', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // |17 = blue background
  const input = '|17Blue BG';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '<ans-17>'); // Blue bg, Light Gray fg
});

Deno.test('Renegade - is disabled by default', () => {
  // Without option, pipe should be passed through
  const input = '|04Text';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '|04Text');
});

Deno.test('Renegade - passes through incomplete codes', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // |0X is not a valid code (X is not a digit)
  const input = '|0XText';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '|0XText');
});

Deno.test('Renegade - passes through literal pipe', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // Single | followed by non-digit should be passed through
  const input = '|Hello';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '|Hello');
});

// UTF-8 input mode tests
Deno.test('UTF-8 - passes through UTF-8 text', () => {
  const utf8Options: ConvertOptions = { utf8Input: true };
  const input = 'Hello, 世界!';
  const result = ansiToHtml(input, utf8Options);
  assertStringIncludes(result, 'Hello, 世界!');
});

Deno.test('UTF-8 - still converts control characters', () => {
  const utf8Options: ConvertOptions = { utf8Input: true };
  // Control char 0x01 (smiley in CP437) should still be converted
  const input = '\x01 Hello';
  const result = ansiToHtml(input, utf8Options);
  assertStringIncludes(result, '☺');
  assertStringIncludes(result, 'Hello');
});

Deno.test('UTF-8 - handles ANSI codes in UTF-8 mode', () => {
  const utf8Options: ConvertOptions = { utf8Input: true };
  // ANSI codes should still work in UTF-8 mode
  const input = '\x1b[31mRed 日本語\x1b[0m';
  const result = ansiToHtml(input, utf8Options);
  assertStringIncludes(result, '<ans-04>'); // Red
  assertStringIncludes(result, '日本語');
});

Deno.test('UTF-8 - handles Renegade codes with UTF-8 text', () => {
  const options: ConvertOptions = { utf8Input: true, renegadePipe: true };
  const input = '|04Red |02Grün';
  const result = ansiToHtml(input, options);
  assertStringIncludes(result, '<ans-04>'); // Red
  assertStringIncludes(result, '<ans-02>'); // Green
  assertStringIncludes(result, 'Grün'); // German umlaut preserved
});

// SAUCE metadata parsing tests

// Helper to create a valid 128-byte SAUCE record
function createSauceRecord(fields: {
  title?: string;
  author?: string;
  group?: string;
  date?: string;
  width?: number;
  height?: number;
  font?: string;
}): string {
  const pad = (s: string, len: number) => s.padEnd(len, ' ').slice(0, len);
  const padNull = (s: string, len: number) => s.padEnd(len, '\0').slice(0, len);

  let record = 'SAUCE00';                                    // 7 bytes
  record += pad(fields.title || '', 35);                     // 35 bytes
  record += pad(fields.author || '', 20);                    // 20 bytes
  record += pad(fields.group || '', 20);                     // 20 bytes
  record += pad(fields.date || '', 8);                       // 8 bytes
  record += '\0\0\0\0';                                      // 4 bytes filesize
  record += '\x01\x01';                                      // 2 bytes datatype, filetype
  // TInfo1-4: width (2), height (2), zeros (4)
  const w = fields.width || 0;
  const h = fields.height || 0;
  record += String.fromCharCode(w & 0xFF, (w >> 8) & 0xFF);
  record += String.fromCharCode(h & 0xFF, (h >> 8) & 0xFF);
  record += '\0\0\0\0';                                      // 4 bytes tinfo3-4
  record += '\0';                                            // 1 byte comments
  record += '\0';                                            // 1 byte tflags
  record += padNull(fields.font || '', 22);                  // 22 bytes tinfos
  return record;
}

Deno.test('SUB without SAUCE - stops processing', () => {
  // SUB without valid SAUCE record - content after SUB is ignored
  const input = 'Visible\x1aRandom garbage after SUB';
  const result = ansiToHtml(input);
  assertStringIncludes(result, 'Visible');
  assertEquals(result.includes('Random'), false);
  assertEquals(result.includes('garbage'), false);
});

Deno.test('SAUCE record - parsed and displayed', () => {
  const sauce = createSauceRecord({
    title: 'Test Title',
    author: 'Test Author',
    group: 'Test Group',
    date: '20240115',
    width: 80,
    height: 25,
    font: 'IBM VGA',
  });
  const input = 'Content before SAUCE\x1a' + sauce;
  const result = ansiToHtml(input);
  assertStringIncludes(result, 'Content before SAUCE');
  assertStringIncludes(result, 'Title: Test Title');
  assertStringIncludes(result, 'Author: Test Author');
  assertStringIncludes(result, 'Group: Test Group');
  assertStringIncludes(result, 'Date: 2024-01-15');
  assertStringIncludes(result, 'Size: 80x25');
  assertStringIncludes(result, 'Font: IBM VGA');
});

Deno.test('SAUCE with COMNT block', () => {
  // Create COMNT block (5 bytes header + 64-byte comment line)
  const comment = 'This is a comment line for the ANSI art.'.padEnd(64, ' ');
  const comnt = 'COMNT' + comment;
  const sauce = createSauceRecord({
    title: 'Artwork Title',
    author: 'Artist',
  });
  const input = 'Art content\x1a' + comnt + sauce;
  const result = ansiToHtml(input);
  assertStringIncludes(result, 'Art content');
  assertStringIncludes(result, 'Title: Artwork Title');
  assertStringIncludes(result, 'Author: Artist');
  assertStringIncludes(result, 'Comment: This is a comment line for the ANSI art.');
});

Deno.test('Content after SAUCE - continues parsing', () => {
  const sauce = createSauceRecord({ title: 'Title' });
  const input = 'Before SAUCE\x1a' + sauce + 'Content after SAUCE record';
  const result = ansiToHtml(input);
  assertStringIncludes(result, 'Before SAUCE');
  assertStringIncludes(result, 'Title: Title');
  assertStringIncludes(result, 'Content after SAUCE record');
});

Deno.test('SAUCE UTF-8 mode', () => {
  const utf8Options: ConvertOptions = { utf8Input: true };
  const sauce = createSauceRecord({ title: 'UTF-8 Test' });
  const input = 'Hello UTF-8 é\x1a' + sauce;
  const result = ansiToHtml(input, utf8Options);
  assertStringIncludes(result, 'Hello UTF-8 é');
  assertStringIncludes(result, 'Title: UTF-8 Test');
});

// CSS and JS are provided as static assets in the repository `wwwroot/`.
// Tests for generation were removed because the library no longer emits
// CSS/JS at runtime; web projects should load `/style.css` and
// `/ansi-display.js` from their public directory.

// ========== 256-color and RGB support tests ==========

Deno.test('256-color - handles foreground', () => {
  // ESC[38;5;196m = 256-color foreground, color 196 (bright red in cube)
  const input = '\x1b[38;5;196mRed 256';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-256 fg="196" bg="bg-0">');
  assertStringIncludes(result, 'Red 256');
  assertStringIncludes(result, '</ans-256>');
});

Deno.test('256-color - handles background', () => {
  // ESC[48;5;21m = 256-color background, color 21 (blue in cube)
  const input = '\x1b[48;5;21mBlue BG';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-256 fg="fg-7" bg="21">');
  assertStringIncludes(result, 'Blue BG');
});

Deno.test('256-color - handles both fg and bg', () => {
  // ESC[38;5;226;48;5;21m = yellow fg (226) on blue bg (21)
  const input = '\x1b[38;5;226;48;5;21mYellow on Blue';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-256 fg="226" bg="21">');
});

Deno.test('RGB - handles foreground', () => {
  // ESC[38;2;255;128;0m = RGB foreground (orange)
  const input = '\x1b[38;2;255;128;0mOrange';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-rgb fg="255,128,0" bg="bg-0">');
  assertStringIncludes(result, 'Orange');
  assertStringIncludes(result, '</ans-rgb>');
});

Deno.test('RGB - handles background', () => {
  // ESC[48;2;0;64;128m = RGB background (dark blue)
  const input = '\x1b[48;2;0;64;128mDark Blue BG';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-rgb fg="fg-7" bg="0,64,128">');
  assertStringIncludes(result, 'Dark Blue BG');
});

Deno.test('RGB - handles both fg and bg', () => {
  // ESC[38;2;255;255;0;48;2;128;0;128m = yellow fg on purple bg
  const input = '\x1b[38;2;255;255;0;48;2;128;0;128mYellow on Purple';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-rgb fg="255,255,0" bg="128,0,128">');
});

Deno.test('Extended color - reset to CGA', () => {
  // Start with 256-color, then reset to default
  const input = '\x1b[38;5;196mRed\x1b[0mNormal';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-256 fg="196"');
  assertStringIncludes(result, 'Red');
  assertStringIncludes(result, '</ans-256>');
  assertStringIncludes(result, '<ans-07>Normal');
});

Deno.test('Extended color - switch CGA to 256', () => {
  // Start with CGA red, then switch to 256-color
  const input = '\x1b[31mCGA Red\x1b[38;5;196m256 Red';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-04>CGA Red</ans-04>');
  assertStringIncludes(result, '<ans-256 fg="196"');
  assertStringIncludes(result, '256 Red');
});

Deno.test('Extended color - switch 256 to RGB', () => {
  // Start with 256-color, then switch to RGB
  const input = '\x1b[38;5;196m256\x1b[38;2;255;0;0mRGB';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-256');
  assertStringIncludes(result, '256');
  assertStringIncludes(result, '<ans-rgb fg="255,0,0"');
  assertStringIncludes(result, 'RGB');
});

Deno.test('256-color - handles CGA range', () => {
  // 256-color palette indices 0-15 are the standard CGA colors
  // Test index 4 (blue in 256-color palette)
  const input = '\x1b[38;5;4mBlue';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-256 fg="4"');
});

Deno.test('256-color - handles grayscale', () => {
  // Test grayscale colors (232-255)
  const input = '\x1b[38;5;240mGray';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '<ans-256 fg="240"');
});

// ========== Renegade escaped pipe tests ==========

Deno.test('Renegade - escaped pipe outputs literal pipe', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // || should output a single | and continue
  const input = '||Hello';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '|Hello');
});

Deno.test('Renegade - escaped pipe followed by digits', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // ||04 should output |04 (literal pipe followed by 04)
  const input = '||04Red';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '|04Red');
});

Deno.test('Renegade - high intensity background colors', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // |24 = dark gray background (high intensity black)
  const input = '|24Dark Gray BG';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '<ans-87>'); // Dark Gray bg (8), Light Gray fg (7)
});

Deno.test('Renegade - high intensity background colors range', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // |31 = white background (high intensity)
  const input = '|31White BG';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '<ans-f7>'); // White bg (f), Light Gray fg (7)
});

Deno.test('Renegade - combined high intensity bg with fg', () => {
  const renegadeOptions: ConvertOptions = { renegadePipe: true };
  // |00 = black fg, |28 = light red background
  const input = '|00|28Black on Light Red';
  const result = ansiToHtml(input, renegadeOptions);
  assertStringIncludes(result, '<ans-c0>'); // Light Red bg (c), Black fg (0)
});
