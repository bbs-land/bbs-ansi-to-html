//! ansi-to-html-rs - Convert CP437 ANSI art files to HTML with custom web components
//!
//! This library converts byte arrays representing Code Page 437 (CP437) text with optional
//! ANSI escape sequences into HTML fragments using custom `<ans-KF>` web components.
//!
//! ## Features
//!
//! - **CP437 to Unicode conversion**: All 256 CP437 characters are mapped to their Unicode
//!   equivalents, including box drawing characters, special symbols, and extended Latin.
//!
//! - **ANSI escape sequence support**:
//!   - SGR (Select Graphic Rendition) color codes: 30-37, 40-47, 90-97, 100-107
//!   - Bold/bright mode (ESC[1m) - sets high intensity on foreground
//!   - Dim mode (ESC[2m, ESC[22m) - clears high intensity
//!   - Blink (ESC[5m, ESC[6m) - sets high intensity on background (CGA style)
//!   - Reverse video (ESC[7m)
//!   - Reset (ESC[0m)
//!   - Clear screen (ESC[2J, ESC[3J) - emits three newlines
//!   - Cursor forward (ESC[C, ESC[nC) - emits n space characters (default 1)
//!   - Save/restore cursor position (ESC[s/ESC[u and ESC7/ESC8) - collapses text between
//!
//! - **BBS color code support** (optional):
//!   - **Synchronet Ctrl-A codes**: Ctrl-A followed by color character
//!     - Foreground: K(black), R(red), G(green), Y(yellow), B(blue), M(magenta), C(cyan), W(white)
//!     - Bright: Uppercase letters for high-intensity versions
//!     - Background: 0-7 for background colors
//!     - H(high intensity), N(normal/reset)
//!   - **Renegade pipe codes**: `|00` through `|23`
//!     - `|00`-`|07`: Normal foreground colors
//!     - `|08`-`|15`: High-intensity foreground colors
//!     - `|16`-`|23`: Background colors
//!
//! - **HTML output**: Results are wrapped in `<pre class="ansi">` with `<ans-KF>` custom
//!   elements where K is the background color (0-F) and F is the foreground color (0-F).
//!
//! - **Soft returns**: Lines containing ANSI/BBS sequences automatically wrap at column 80.
//!
//! - **Character handling**:
//!   - Carriage returns (`\r`) are suppressed
//!   - Newlines (`\n`) are preserved
//!   - HTML special characters (`<`, `>`, `&`, `"`) are escaped
//!
//! ## CGA Color Palette
//!
//! | Code | Name | Hex |
//! |------|------|-----|
//! | 0 | Black | #000000 |
//! | 1 | Blue | #0000AA |
//! | 2 | Green | #00AA00 |
//! | 3 | Cyan | #00AAAA |
//! | 4 | Red | #AA0000 |
//! | 5 | Magenta | #AA00AA |
//! | 6 | Brown | #AA5500 |
//! | 7 | Light Gray | #AAAAAA |
//! | 8 | Dark Gray | #555555 |
//! | 9 | Light Blue | #5555FF |
//! | A | Light Green | #55FF55 |
//! | B | Light Cyan | #55FFFF |
//! | C | Light Red | #FF5555 |
//! | D | Light Magenta | #FF55FF |
//! | E | Yellow | #FFFF55 |
//! | F | White | #FFFFFF |
//!
//! ## Example
//!
//! ```rust
//! use ansi_to_html_rs::{convert, convert_with_options, ConvertOptions, generate_css, generate_js};
//!
//! // Convert ANSI art to HTML (standard ANSI only)
//! let ansi_data = b"\x1b[31mRed Text\x1b[0m Normal";
//! let html = convert(ansi_data);
//!
//! // Convert with BBS color code support
//! let options = ConvertOptions {
//!     synchronet_ctrl_a: true,
//!     renegade_pipe: true,
//!     utf8_input: false,
//! };
//! let bbs_data = b"|04Red |02Green";
//! let html = convert_with_options(bbs_data, &options);
//!
//! // Generate supporting CSS and JavaScript
//! let css = generate_css();
//! let js = generate_js();
//! ```

mod cp437;

use cp437::CP437_TO_UNICODE;

/// CGA color hex values
pub const CGA_COLORS: [&str; 16] = [
    "#000000", // 0 - Black
    "#0000AA", // 1 - Blue
    "#00AA00", // 2 - Green
    "#00AAAA", // 3 - Cyan
    "#AA0000", // 4 - Red
    "#AA00AA", // 5 - Magenta
    "#AA5500", // 6 - Brown
    "#AAAAAA", // 7 - Light Gray
    "#555555", // 8 - Dark Gray
    "#5555FF", // 9 - Light Blue
    "#55FF55", // A - Light Green
    "#55FFFF", // B - Light Cyan
    "#FF5555", // C - Light Red
    "#FF55FF", // D - Light Magenta
    "#FFFF55", // E - Yellow
    "#FFFFFF", // F - White
];

/// Options for controlling conversion behavior
#[derive(Debug, Clone, Copy, Default)]
pub struct ConvertOptions {
    /// Enable Synchronet Ctrl-A color codes (Ctrl-A + character)
    pub synchronet_ctrl_a: bool,
    /// Enable Renegade BBS pipe codes (|00 through |23)
    pub renegade_pipe: bool,
    /// Treat input as UTF-8 instead of CP437 (only convert control chars < 0x20)
    pub utf8_input: bool,
}

/// Parser state for ANSI escape sequences
#[derive(Debug, Clone, Copy, PartialEq)]
enum ParseState {
    Normal,
    Escape,
    Csi,
    /// Synchronet Ctrl-A code (waiting for color character)
    SynchronetCtrlA,
    /// Renegade pipe code (waiting for first digit)
    RenegadePipe1,
    /// Renegade pipe code (waiting for second digit)
    RenegadePipe2(u8),
}

/// Converter state
struct Converter {
    foreground: u8,
    background: u8,
    output: String,
    current_column: u32,
    line_has_ansi: bool,
    save_position_active: bool,
    parse_state: ParseState,
    csi_params: String,
    options: ConvertOptions,
}

impl Converter {
    fn new(options: ConvertOptions) -> Self {
        Self {
            foreground: 7,  // Light Gray
            background: 0,  // Black
            output: String::new(),
            current_column: 0,
            line_has_ansi: false,
            save_position_active: false,
            parse_state: ParseState::Normal,
            csi_params: String::new(),
            options,
        }
    }

    fn color_to_hex(color: u8) -> char {
        match color {
            0..=9 => (b'0' + color) as char,
            10..=15 => (b'a' + color - 10) as char,
            _ => '0',
        }
    }

    fn open_tag(&mut self) {
        let bg = Self::color_to_hex(self.background);
        let fg = Self::color_to_hex(self.foreground);
        self.output.push_str(&format!("<ans-{}{}>", bg, fg));
    }

    fn close_tag(&mut self) {
        let bg = Self::color_to_hex(self.background);
        let fg = Self::color_to_hex(self.foreground);
        self.output.push_str(&format!("</ans-{}{}>", bg, fg));
    }

    fn switch_color(&mut self, new_bg: u8, new_fg: u8) {
        if new_bg != self.background || new_fg != self.foreground {
            self.close_tag();
            self.background = new_bg;
            self.foreground = new_fg;
            self.open_tag();
        }
    }

    fn emit_char(&mut self, ch: char) {
        if self.save_position_active {
            return;
        }

        // Check for soft return at column 80
        if self.line_has_ansi && self.current_column >= 80 && ch != '\n' {
            self.output.push('\n');
            self.current_column = 0;
        }

        match ch {
            '<' => self.output.push_str("&lt;"),
            '>' => self.output.push_str("&gt;"),
            '&' => self.output.push_str("&amp;"),
            '"' => self.output.push_str("&quot;"),
            '\n' => {
                self.output.push('\n');
                self.current_column = 0;
                self.line_has_ansi = false;
            }
            '\r' => {
                // Suppress carriage returns
            }
            _ => {
                self.output.push(ch);
                self.current_column += 1;
            }
        }
    }

    fn process_sgr(&mut self, params: &str) {
        // SGR (Select Graphic Rendition) - handles color codes
        let params: Vec<u8> = if params.is_empty() {
            vec![0]
        } else {
            params
                .split(';')
                .filter_map(|s| s.parse().ok())
                .collect()
        };

        let mut new_fg = self.foreground;
        let mut new_bg = self.background;

        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => {
                    // Reset
                    new_fg = 7;
                    new_bg = 0;
                }
                1 => {
                    // Bold/Bright - set high bit on foreground
                    new_fg |= 0x08;
                }
                2 | 22 => {
                    // Dim / Normal intensity - clear high bit
                    new_fg &= 0x07;
                }
                5 | 6 => {
                    // Blink - set high bit on background (in CGA terms)
                    new_bg |= 0x08;
                }
                25 => {
                    // Blink off
                    new_bg &= 0x07;
                }
                7 => {
                    // Reverse video
                    std::mem::swap(&mut new_fg, &mut new_bg);
                }
                30..=37 => {
                    // Standard foreground colors
                    let color = params[i] - 30;
                    // Map ANSI colors to CGA: 0=black, 1=red, 2=green, 3=brown, 4=blue, 5=magenta, 6=cyan, 7=white
                    let cga_color = match color {
                        0 => 0, // Black
                        1 => 4, // Red
                        2 => 2, // Green
                        3 => 6, // Brown/Yellow
                        4 => 1, // Blue
                        5 => 5, // Magenta
                        6 => 3, // Cyan
                        7 => 7, // White/Light Gray
                        _ => 7,
                    };
                    new_fg = (new_fg & 0x08) | cga_color;
                }
                39 => {
                    // Default foreground
                    new_fg = 7;
                }
                40..=47 => {
                    // Standard background colors
                    let color = params[i] - 40;
                    let cga_color = match color {
                        0 => 0,
                        1 => 4,
                        2 => 2,
                        3 => 6,
                        4 => 1,
                        5 => 5,
                        6 => 3,
                        7 => 7,
                        _ => 0,
                    };
                    new_bg = (new_bg & 0x08) | cga_color;
                }
                49 => {
                    // Default background
                    new_bg = 0;
                }
                90..=97 => {
                    // Bright foreground colors
                    let color = params[i] - 90;
                    let cga_color = match color {
                        0 => 8,  // Dark Gray
                        1 => 12, // Light Red
                        2 => 10, // Light Green
                        3 => 14, // Yellow
                        4 => 9,  // Light Blue
                        5 => 13, // Light Magenta
                        6 => 11, // Light Cyan
                        7 => 15, // White
                        _ => 15,
                    };
                    new_fg = cga_color;
                }
                100..=107 => {
                    // Bright background colors
                    let color = params[i] - 100;
                    let cga_color = match color {
                        0 => 8,
                        1 => 12,
                        2 => 10,
                        3 => 14,
                        4 => 9,
                        5 => 13,
                        6 => 11,
                        7 => 15,
                        _ => 8,
                    };
                    new_bg = cga_color;
                }
                _ => {}
            }
            i += 1;
        }

        self.switch_color(new_bg, new_fg);
    }

    fn process_csi(&mut self, params: &str, command: char) {
        self.line_has_ansi = true;

        match command {
            'm' => {
                // SGR - Select Graphic Rendition
                self.process_sgr(params);
            }
            'J' => {
                // ED - Erase Display
                let n: u8 = params.parse().unwrap_or(0);
                if n == 2 || n == 3 {
                    // Clear screen - inject three line feeds
                    self.emit_char('\n');
                    self.emit_char('\n');
                    self.emit_char('\n');
                }
            }
            's' => {
                // SCP - Save Cursor Position
                self.save_position_active = true;
            }
            'u' => {
                // RCP - Restore Cursor Position
                self.save_position_active = false;
            }
            'H' | 'f' => {
                // CUP - Cursor Position (often used with clear screen)
            }
            'C' => {
                // CUF - Cursor Forward: emit n space characters (default 1)
                let n: u32 = params.parse().unwrap_or(1).max(1);
                for _ in 0..n {
                    self.emit_char(' ');
                }
            }
            'A' | 'B' | 'D' => {
                // Other cursor movement (up, down, back) - ignored for static conversion
            }
            'K' => {
                // EL - Erase in Line - ignored
            }
            _ => {
                // Other CSI sequences - ignored
            }
        }
    }

    /// Process Synchronet Ctrl-A color code
    fn process_synchronet_code(&mut self, code: u8) {
        self.line_has_ansi = true;
        let mut new_fg = self.foreground;
        let mut new_bg = self.background;

        match code {
            // Lowercase = normal intensity foreground
            b'k' => new_fg = 0,  // Black
            b'b' => new_fg = 1,  // Blue
            b'g' => new_fg = 2,  // Green
            b'c' => new_fg = 3,  // Cyan
            b'r' => new_fg = 4,  // Red
            b'm' => new_fg = 5,  // Magenta
            b'y' => new_fg = 6,  // Brown/Yellow
            b'w' => new_fg = 7,  // White/Light Gray

            // Uppercase = high intensity foreground
            b'K' => new_fg = 8,  // Dark Gray
            b'B' => new_fg = 9,  // Light Blue
            b'G' => new_fg = 10, // Light Green
            b'C' => new_fg = 11, // Light Cyan
            b'R' => new_fg = 12, // Light Red
            b'M' => new_fg = 13, // Light Magenta
            b'Y' => new_fg = 14, // Yellow
            b'W' => new_fg = 15, // White

            // Background colors (0-7)
            b'0' => new_bg = 0,  // Black
            b'1' => new_bg = 1,  // Blue
            b'2' => new_bg = 2,  // Green
            b'3' => new_bg = 3,  // Cyan
            b'4' => new_bg = 4,  // Red
            b'5' => new_bg = 5,  // Magenta
            b'6' => new_bg = 6,  // Brown
            b'7' => new_bg = 7,  // White/Light Gray

            // Special codes
            b'H' | b'h' => new_fg |= 0x08,  // High intensity
            b'I' | b'i' => new_bg |= 0x08,  // Blink/high intensity background
            b'N' | b'n' => {
                // Normal - reset to default
                new_fg = 7;
                new_bg = 0;
            }
            b'-' => new_fg &= 0x07, // Remove high intensity from foreground
            b'_' => new_bg &= 0x07, // Remove blink from background

            _ => {} // Unknown code, ignore
        }

        self.switch_color(new_bg, new_fg);
    }

    /// Process Renegade pipe color code (0-23)
    fn process_renegade_code(&mut self, code: u8) {
        self.line_has_ansi = true;
        let mut new_fg = self.foreground;
        let mut new_bg = self.background;

        match code {
            // Foreground colors 0-7 (normal intensity)
            0 => new_fg = 0,  // Black
            1 => new_fg = 1,  // Blue
            2 => new_fg = 2,  // Green
            3 => new_fg = 3,  // Cyan
            4 => new_fg = 4,  // Red
            5 => new_fg = 5,  // Magenta
            6 => new_fg = 6,  // Brown
            7 => new_fg = 7,  // Light Gray

            // Foreground colors 8-15 (high intensity)
            8 => new_fg = 8,   // Dark Gray
            9 => new_fg = 9,   // Light Blue
            10 => new_fg = 10, // Light Green
            11 => new_fg = 11, // Light Cyan
            12 => new_fg = 12, // Light Red
            13 => new_fg = 13, // Light Magenta
            14 => new_fg = 14, // Yellow
            15 => new_fg = 15, // White

            // Background colors 16-23
            16 => new_bg = 0, // Black
            17 => new_bg = 1, // Blue
            18 => new_bg = 2, // Green
            19 => new_bg = 3, // Cyan
            20 => new_bg = 4, // Red
            21 => new_bg = 5, // Magenta
            22 => new_bg = 6, // Brown
            23 => new_bg = 7, // Light Gray

            _ => {} // Invalid code, ignore
        }

        self.switch_color(new_bg, new_fg);
    }

    fn process_byte(&mut self, byte: u8) {
        match self.parse_state {
            ParseState::Normal => {
                if byte == 0x1B {
                    self.parse_state = ParseState::Escape;
                } else if self.options.synchronet_ctrl_a && byte == 0x01 {
                    // Ctrl-A for Synchronet codes
                    self.parse_state = ParseState::SynchronetCtrlA;
                } else if self.options.renegade_pipe && byte == b'|' {
                    // Pipe for Renegade codes
                    self.parse_state = ParseState::RenegadePipe1;
                } else if byte == b'\n' {
                    self.emit_char('\n');
                } else if byte == b'\r' {
                    // Suppress carriage returns
                } else if byte < 0x20 || byte >= 0x7F {
                    // Convert CP437 to Unicode
                    let unicode_char = CP437_TO_UNICODE[byte as usize];
                    self.emit_char(unicode_char);
                } else {
                    self.emit_char(byte as char);
                }
            }
            ParseState::Escape => {
                match byte {
                    b'[' => {
                        self.parse_state = ParseState::Csi;
                        self.csi_params.clear();
                    }
                    b'7' => {
                        // \e7 - Save cursor position (DEC)
                        self.save_position_active = true;
                        self.line_has_ansi = true;
                        self.parse_state = ParseState::Normal;
                    }
                    b'8' => {
                        // \e8 - Restore cursor position (DEC)
                        self.save_position_active = false;
                        self.line_has_ansi = true;
                        self.parse_state = ParseState::Normal;
                    }
                    _ => {
                        // Unknown escape sequence, return to normal
                        self.parse_state = ParseState::Normal;
                    }
                }
            }
            ParseState::Csi => {
                if byte.is_ascii_digit() || byte == b';' {
                    self.csi_params.push(byte as char);
                } else if byte >= 0x40 && byte <= 0x7E {
                    // Final byte of CSI sequence
                    let params = std::mem::take(&mut self.csi_params);
                    self.process_csi(&params, byte as char);
                    self.parse_state = ParseState::Normal;
                } else {
                    // Invalid CSI sequence
                    self.parse_state = ParseState::Normal;
                }
            }
            ParseState::SynchronetCtrlA => {
                self.process_synchronet_code(byte);
                self.parse_state = ParseState::Normal;
            }
            ParseState::RenegadePipe1 => {
                if byte.is_ascii_digit() {
                    self.parse_state = ParseState::RenegadePipe2(byte - b'0');
                } else {
                    // Not a valid pipe code, emit the pipe and this character
                    self.emit_char('|');
                    self.parse_state = ParseState::Normal;
                    // Re-process this byte in normal state
                    self.process_byte(byte);
                }
            }
            ParseState::RenegadePipe2(first_digit) => {
                if byte.is_ascii_digit() {
                    let code = first_digit * 10 + (byte - b'0');
                    if code <= 23 {
                        self.process_renegade_code(code);
                    }
                    // If code > 23, just ignore the sequence
                } else {
                    // Not a valid second digit, emit pipe + first digit and re-process
                    self.emit_char('|');
                    self.emit_char((b'0' + first_digit) as char);
                    self.parse_state = ParseState::Normal;
                    self.process_byte(byte);
                    return;
                }
                self.parse_state = ParseState::Normal;
            }
        }
    }

    fn convert(&mut self, input: &[u8]) -> String {
        self.output.push_str("<pre class=\"ansi\">");
        self.open_tag();

        for &byte in input {
            self.process_byte(byte);
        }

        self.close_tag();
        self.output.push_str("</pre>");

        std::mem::take(&mut self.output)
    }

    fn convert_utf8(&mut self, input: &[u8]) -> String {
        self.output.push_str("<pre class=\"ansi\">");
        self.open_tag();

        // Parse as UTF-8, falling back to replacement char for invalid sequences
        let text = String::from_utf8_lossy(input);

        for ch in text.chars() {
            self.process_utf8_char(ch);
        }

        self.close_tag();
        self.output.push_str("</pre>");

        std::mem::take(&mut self.output)
    }

    fn process_utf8_char(&mut self, ch: char) {
        let code = ch as u32;

        match self.parse_state {
            ParseState::Normal => {
                if code == 0x1B {
                    self.parse_state = ParseState::Escape;
                } else if self.options.synchronet_ctrl_a && code == 0x01 {
                    self.parse_state = ParseState::SynchronetCtrlA;
                } else if self.options.renegade_pipe && ch == '|' {
                    self.parse_state = ParseState::RenegadePipe1;
                } else if ch == '\n' {
                    self.emit_char('\n');
                } else if ch == '\r' {
                    // Suppress carriage returns
                } else if code < 0x20 {
                    // Convert low-byte control characters using CP437 mapping
                    let unicode_char = CP437_TO_UNICODE[code as usize];
                    self.emit_char(unicode_char);
                } else {
                    // Pass through all other UTF-8 characters as-is
                    self.emit_char(ch);
                }
            }
            ParseState::Escape => {
                match ch {
                    '[' => {
                        self.parse_state = ParseState::Csi;
                        self.csi_params.clear();
                    }
                    '7' => {
                        self.save_position_active = true;
                        self.line_has_ansi = true;
                        self.parse_state = ParseState::Normal;
                    }
                    '8' => {
                        self.save_position_active = false;
                        self.line_has_ansi = true;
                        self.parse_state = ParseState::Normal;
                    }
                    _ => {
                        self.parse_state = ParseState::Normal;
                    }
                }
            }
            ParseState::Csi => {
                if ch.is_ascii_digit() || ch == ';' {
                    self.csi_params.push(ch);
                } else if code >= 0x40 && code <= 0x7E {
                    let params = std::mem::take(&mut self.csi_params);
                    self.process_csi(&params, ch);
                    self.parse_state = ParseState::Normal;
                } else {
                    self.parse_state = ParseState::Normal;
                }
            }
            ParseState::SynchronetCtrlA => {
                if code <= 0xFF {
                    self.process_synchronet_code(code as u8);
                }
                self.parse_state = ParseState::Normal;
            }
            ParseState::RenegadePipe1 => {
                if ch.is_ascii_digit() {
                    self.parse_state = ParseState::RenegadePipe2(ch as u8 - b'0');
                } else {
                    self.emit_char('|');
                    self.parse_state = ParseState::Normal;
                    self.process_utf8_char(ch);
                }
            }
            ParseState::RenegadePipe2(first_digit) => {
                if ch.is_ascii_digit() {
                    let code = first_digit * 10 + (ch as u8 - b'0');
                    if code <= 23 {
                        self.process_renegade_code(code);
                    }
                } else {
                    self.emit_char('|');
                    self.emit_char((b'0' + first_digit) as char);
                    self.parse_state = ParseState::Normal;
                    self.process_utf8_char(ch);
                    return;
                }
                self.parse_state = ParseState::Normal;
            }
        }
    }
}

/// Convert a CP437 byte array with ANSI escape sequences to an HTML fragment.
///
/// This function uses default options (no BBS color code support).
/// For BBS color code support, use [`convert_with_options`].
///
/// # Arguments
/// * `input` - A byte slice representing CP437 encoded text with optional ANSI escape sequences
///
/// # Returns
/// An HTML string wrapped in a `<pre class="ansi">` element with `<ans-KF>` custom elements
/// for color styling.
///
/// # Example
/// ```
/// use ansi_to_html_rs::convert;
///
/// let input = b"Hello, World!";
/// let html = convert(input);
/// assert!(html.contains("<pre class=\"ansi\">"));
/// assert!(html.contains("<ans-07>"));
/// ```
pub fn convert(input: &[u8]) -> String {
    convert_with_options(input, &ConvertOptions::default())
}

/// Convert a CP437 byte array with ANSI/BBS escape sequences to an HTML fragment.
///
/// # Arguments
/// * `input` - A byte slice representing CP437 encoded text with optional escape sequences
/// * `options` - Conversion options controlling which BBS color code formats to process
///
/// # Returns
/// An HTML string wrapped in a `<pre class="ansi">` element with `<ans-KF>` custom elements
/// for color styling.
///
/// # Example
/// ```
/// use ansi_to_html_rs::{convert_with_options, ConvertOptions};
///
/// let options = ConvertOptions {
///     synchronet_ctrl_a: false,
///     renegade_pipe: true,
///     utf8_input: false,
/// };
/// let input = b"|04Red |02Green";
/// let html = convert_with_options(input, &options);
/// assert!(html.contains("<ans-04>")); // Red
/// assert!(html.contains("<ans-02>")); // Green
/// ```
pub fn convert_with_options(input: &[u8], options: &ConvertOptions) -> String {
    let mut converter = Converter::new(*options);
    if options.utf8_input {
        converter.convert_utf8(input)
    } else {
        converter.convert(input)
    }
}

/// Generate CSS for the ans-KF web components.
///
/// This returns CSS custom property definitions for all 256 color combinations.
pub fn generate_css() -> String {
    let mut css = String::from(
        r#":root {
  --ans-font-family: "IBM VGA 8x16", "Perfect DOS VGA 437", "Px437 IBM VGA8", monospace;
  --ans-font-size: 16px;
  --ans-line-height: 1;
}

pre.ansi {
  font-family: var(--ans-font-family);
  font-size: var(--ans-font-size);
  line-height: var(--ans-line-height);
  background-color: #000000;
  padding: 0;
  margin: 0;
  white-space: pre;
}

"#,
    );

    // Generate styles for each color combination
    for bg in 0..16u8 {
        for fg in 0..16u8 {
            let bg_hex = Converter::color_to_hex(bg);
            let fg_hex = Converter::color_to_hex(fg);
            css.push_str(&format!(
                "ans-{}{} {{ background-color: {}; color: {}; }}\n",
                bg_hex, fg_hex, CGA_COLORS[bg as usize], CGA_COLORS[fg as usize]
            ));
        }
    }

    css
}

/// Generate JavaScript for defining ans-KF web components.
///
/// This returns JavaScript code that defines custom elements for all 256 color combinations.
pub fn generate_js() -> String {
    let js = String::from(
        r##"// ANSI color web components
(function() {
  const colors = [
    "#000000", "#0000AA", "#00AA00", "#00AAAA",
    "#AA0000", "#AA00AA", "#AA5500", "#AAAAAA",
    "#555555", "#5555FF", "#55FF55", "#55FFFF",
    "#FF5555", "#FF55FF", "#FFFF55", "#FFFFFF"
  ];

  const hexChars = "0123456789ABCDEF";

  for (let bg = 0; bg < 16; bg++) {
    for (let fg = 0; fg < 16; fg++) {
      const tagName = `ans-${hexChars[bg]}${hexChars[fg]}`;

      if (!customElements.get(tagName.toLowerCase())) {
        const bgColor = colors[bg];
        const fgColor = colors[fg];

        class AnsElement extends HTMLElement {
          constructor() {
            super();
          }

          connectedCallback() {
            this.style.backgroundColor = bgColor;
            this.style.color = fgColor;
            this.style.display = "inline";
          }
        }

        customElements.define(tagName.toLowerCase(), AnsElement);
      }
    }
  }
})();
"##,
    );

    js
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_text() {
        let result = convert(b"Hello");
        assert!(result.contains("<pre class=\"ansi\">"));
        assert!(result.contains("<ans-07>"));
        assert!(result.contains("Hello"));
        assert!(result.contains("</ans-07>"));
        assert!(result.contains("</pre>"));
    }

    #[test]
    fn test_html_escaping() {
        let result = convert(b"<script>&</script>");
        assert!(result.contains("&lt;script&gt;&amp;&lt;/script&gt;"));
    }

    #[test]
    fn test_color_change() {
        // ESC[31m sets red foreground
        let input = b"\x1b[31mRed";
        let result = convert(input);
        assert!(result.contains("<ans-04>")); // Red foreground on black
    }

    #[test]
    fn test_clear_screen() {
        // ESC[2J clears screen
        let input = b"Before\x1b[2JAfter";
        let result = convert(input);
        // Should have three newlines for clear screen
        assert!(result.contains("\n\n\n"));
    }

    #[test]
    fn test_newline_preserved() {
        let result = convert(b"Line1\nLine2");
        assert!(result.contains("Line1\nLine2"));
    }

    #[test]
    fn test_carriage_return_suppressed() {
        let result = convert(b"Line1\r\nLine2");
        assert!(!result.contains('\r'));
        assert!(result.contains("Line1\nLine2"));
    }

    #[test]
    fn test_generate_css() {
        let css = generate_css();
        assert!(css.contains("ans-07"));
        assert!(css.contains("#AAAAAA")); // Light gray
    }

    #[test]
    fn test_generate_js() {
        let js = generate_js();
        assert!(js.contains("customElements.define"));
    }

    #[test]
    fn test_soft_return_at_column_80() {
        // Create a line with ANSI escape that's longer than 80 chars
        let mut input = vec![0x1b, b'[', b'3', b'1', b'm']; // Red color
        // Add 85 'X' characters - should trigger soft return after 80
        for _ in 0..85 {
            input.push(b'X');
        }
        let result = convert(&input);
        // Should have a newline injected after column 80
        let x_count_before_newline = result
            .split('\n')
            .find(|s| s.contains("XXXX"))
            .map(|s| s.matches('X').count())
            .unwrap_or(0);
        assert_eq!(x_count_before_newline, 80);
    }

    #[test]
    fn test_no_soft_return_without_ansi() {
        // Without ANSI, no soft return should happen
        let input: Vec<u8> = (0..85).map(|_| b'X').collect();
        let result = convert(&input);
        // Should NOT have a newline
        assert!(!result.contains('\n'));
    }

    #[test]
    fn test_save_restore_position_collapse() {
        // ESC[s saves position, text should be collapsed, ESC[u restores
        let input = b"Before\x1b[sHidden\x1b[uAfter";
        let result = convert(input);
        assert!(result.contains("Before"));
        assert!(result.contains("After"));
        assert!(!result.contains("Hidden"));
    }

    #[test]
    fn test_dec_save_restore_position() {
        // \e7 saves position, \e8 restores
        let input = b"Start\x1b7Collapsed\x1b8End";
        let result = convert(input);
        assert!(result.contains("Start"));
        assert!(result.contains("End"));
        assert!(!result.contains("Collapsed"));
    }

    #[test]
    fn test_cp437_box_drawing() {
        // Test box drawing characters (0xDA = top-left corner)
        let input = [0xDA, 0xC4, 0xC4, 0xBF]; // ┌──┐
        let result = convert(&input);
        assert!(result.contains('┌'));
        assert!(result.contains('─'));
        assert!(result.contains('┐'));
    }

    #[test]
    fn test_cp437_special_chars() {
        // Test smiley faces and hearts
        let input = [0x01, 0x02, 0x03]; // ☺☻♥
        let result = convert(&input);
        assert!(result.contains('☺'));
        assert!(result.contains('☻'));
        assert!(result.contains('♥'));
    }

    #[test]
    fn test_bright_foreground_colors() {
        // ESC[91m = bright red
        let input = b"\x1b[91mBright Red";
        let result = convert(input);
        assert!(result.contains("<ans-0c>")); // Light Red on black
    }

    #[test]
    fn test_bright_background_colors() {
        // ESC[101m = bright red background
        let input = b"\x1b[101mBright BG";
        let result = convert(input);
        assert!(result.contains("<ans-c7>")); // Light Red bg, Light Gray fg
    }

    #[test]
    fn test_bold_makes_bright() {
        // ESC[1m makes foreground bright, ESC[34m blue -> light blue
        let input = b"\x1b[1;34mBold Blue";
        let result = convert(input);
        assert!(result.contains("<ans-09>")); // Light Blue (9) on black
    }

    #[test]
    fn test_reset_colors() {
        // ESC[31m red, then ESC[0m reset
        let input = b"\x1b[31mRed\x1b[0mNormal";
        let result = convert(input);
        assert!(result.contains("<ans-04>Red</ans-04>"));
        assert!(result.contains("<ans-07>Normal"));
    }

    #[test]
    fn test_multiple_sgr_params() {
        // ESC[1;31;44m = bold red on blue
        let input = b"\x1b[1;31;44mStyled";
        let result = convert(input);
        assert!(result.contains("<ans-1c>")); // Blue bg (1), Light Red fg (C)
    }

    #[test]
    fn test_full_block_character() {
        // 0xDB = full block
        let input = [0xDB];
        let result = convert(&input);
        assert!(result.contains('█'));
    }

    #[test]
    fn test_shade_characters() {
        // Test shade blocks
        let input = [0xB0, 0xB1, 0xB2]; // ░▒▓
        let result = convert(&input);
        assert!(result.contains('░'));
        assert!(result.contains('▒'));
        assert!(result.contains('▓'));
    }

    #[test]
    fn test_cursor_forward_default() {
        // ESC[C moves cursor forward 1 position (emits 1 space)
        let input = b"A\x1b[CB";
        let result = convert(input);
        assert!(result.contains("A B"));
    }

    #[test]
    fn test_cursor_forward_explicit_one() {
        // ESC[1C moves cursor forward 1 position
        let input = b"A\x1b[1CB";
        let result = convert(input);
        assert!(result.contains("A B"));
    }

    #[test]
    fn test_cursor_forward_multiple() {
        // ESC[5C moves cursor forward 5 positions (emits 5 spaces)
        let input = b"A\x1b[5CB";
        let result = convert(input);
        assert!(result.contains("A     B"));
    }

    #[test]
    fn test_cursor_forward_large() {
        // ESC[10C moves cursor forward 10 positions
        let input = b"X\x1b[10CY";
        let result = convert(input);
        assert!(result.contains("X          Y"));
    }

    #[test]
    fn test_cursor_forward_zero_treated_as_one() {
        // ESC[0C should be treated as ESC[1C per ANSI spec
        let input = b"A\x1b[0CB";
        let result = convert(input);
        assert!(result.contains("A B"));
    }

    // ========== Synchronet Ctrl-A tests ==========

    #[test]
    fn test_synchronet_foreground_colors() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + r = red foreground
        let input = b"\x01rRed Text";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-04>")); // Red on black
    }

    #[test]
    fn test_synchronet_bright_foreground() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + R = bright red (Light Red)
        let input = b"\x01RBright Red";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-0c>")); // Light Red on black
    }

    #[test]
    fn test_synchronet_background_color() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + 1 = blue background
        let input = b"\x011Blue BG";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-17>")); // Blue bg, Light Gray fg
    }

    #[test]
    fn test_synchronet_high_intensity() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + H = high intensity on current color
        let input = b"\x01b\x01HBright Blue";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-09>")); // Light Blue on black
    }

    #[test]
    fn test_synchronet_normal_reset() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + N = reset to normal
        let input = b"\x01RRed\x01NNormal";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-0c>Red</ans-0c>"));
        assert!(result.contains("<ans-07>Normal"));
    }

    #[test]
    fn test_synchronet_disabled_by_default() {
        // Without option, Ctrl-A should be treated as CP437 character (smiley)
        let input = b"\x01rText";
        let result = convert(input);
        assert!(result.contains('☺')); // CP437 0x01 = smiley face
        assert!(result.contains("rText"));
    }

    // ========== Renegade pipe code tests ==========

    #[test]
    fn test_renegade_foreground_colors() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |04 = red foreground
        let input = b"|04Red Text";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-04>")); // Red on black
    }

    #[test]
    fn test_renegade_bright_foreground() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |12 = bright red (Light Red)
        let input = b"|12Bright Red";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-0c>")); // Light Red on black
    }

    #[test]
    fn test_renegade_background_color() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |17 = blue background
        let input = b"|17Blue BG";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-17>")); // Blue bg, Light Gray fg
    }

    #[test]
    fn test_renegade_combined_colors() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |15 = white fg, |20 = red bg
        let input = b"|15|20White on Red";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-4f>")); // Red bg, White fg
    }

    #[test]
    fn test_renegade_disabled_by_default() {
        // Without option, pipe should be passed through
        let input = b"|04Text";
        let result = convert(input);
        assert!(result.contains("|04Text"));
    }

    #[test]
    fn test_renegade_invalid_code_passthrough() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |99 is invalid (>23), should be ignored but not crash
        let input = b"|99Text";
        let result = convert_with_options(input, &options);
        assert!(result.contains("Text"));
    }

    #[test]
    fn test_renegade_incomplete_code_passthrough() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |0X is not a valid code (X is not a digit)
        let input = b"|0XText";
        let result = convert_with_options(input, &options);
        assert!(result.contains("|0XText"));
    }

    #[test]
    fn test_renegade_pipe_literal() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // Single | followed by non-digit should be passed through
        let input = b"|Hello";
        let result = convert_with_options(input, &options);
        assert!(result.contains("|Hello"));
    }

    // ========== Combined options tests ==========

    #[test]
    fn test_both_formats_enabled() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            renegade_pipe: true,
            ..Default::default()
        };
        // Mix of both formats
        let input = b"\x01rSync |09Renegade";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-04>")); // Red from Synchronet
        assert!(result.contains("<ans-09>")); // Light Blue from Renegade
    }

    // ========== UTF-8 input mode tests ==========

    #[test]
    fn test_utf8_input_basic() {
        let options = ConvertOptions {
            utf8_input: true,
            ..Default::default()
        };
        // UTF-8 text with Unicode characters should pass through
        let input = "Hello, 世界!".as_bytes();
        let result = convert_with_options(input, &options);
        assert!(result.contains("Hello, 世界!"));
    }

    #[test]
    fn test_utf8_input_control_chars() {
        let options = ConvertOptions {
            utf8_input: true,
            ..Default::default()
        };
        // Control char 0x01 (smiley in CP437) should still be converted
        let input = b"\x01 Hello";
        let result = convert_with_options(input, &options);
        assert!(result.contains('☺')); // CP437 0x01 = smiley
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_utf8_input_ansi_codes() {
        let options = ConvertOptions {
            utf8_input: true,
            ..Default::default()
        };
        // ANSI codes should still work in UTF-8 mode
        let input = "\x1b[31mRed 日本語\x1b[0m".as_bytes();
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-04>")); // Red
        assert!(result.contains("日本語"));
    }

    #[test]
    fn test_utf8_input_with_renegade() {
        let options = ConvertOptions {
            utf8_input: true,
            renegade_pipe: true,
            ..Default::default()
        };
        // Renegade codes with UTF-8 text
        let input = "|04Red |02Grün".as_bytes();
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-04>")); // Red
        assert!(result.contains("<ans-02>")); // Green
        assert!(result.contains("Grün")); // German umlaut preserved
    }
}
