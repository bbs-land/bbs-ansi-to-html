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
Deno.test('Synchronet - handles foreground colors', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + r = red foreground
  const input = '\x01rRed Text';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-04>'); // Red on black
});

Deno.test('Synchronet - handles bright foreground', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + R = bright red (Light Red)
  const input = '\x01RBright Red';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-0c>'); // Light Red on black
});

Deno.test('Synchronet - handles background color', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + 1 = blue background
  const input = '\x011Blue BG';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-17>'); // Blue bg, Light Gray fg
});

Deno.test('Synchronet - handles normal reset', () => {
  const syncOptions: ConvertOptions = { synchronetCtrlA: true };
  // Ctrl-A + N = reset to normal
  const input = '\x01RRed\x01NNormal';
  const result = ansiToHtml(input, syncOptions);
  assertStringIncludes(result, '<ans-0c>Red</ans-0c>');
  assertStringIncludes(result, '<ans-07>Normal');
});

Deno.test('Synchronet - is disabled by default', () => {
  // Without option, Ctrl-A should be treated as CP437 character (smiley)
  const input = '\x01rText';
  const result = ansiToHtml(input);
  assertStringIncludes(result, '☺'); // CP437 0x01 = smiley face
  assertStringIncludes(result, 'rText');
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

// CSS and JS are provided as static assets in the repository `wwwroot/`.
// Tests for generation were removed because the library no longer emits
// CSS/JS at runtime; web projects should load `/style.css` and
// `/ansi-display.js` from their public directory.
