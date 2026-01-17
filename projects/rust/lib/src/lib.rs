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
//! - **HTML output**: Results are wrapped in `<pre class="ansi">` with custom elements:
//!   - `<ans-KF>` - Standard 16-color CGA where K=background, F=foreground (hex 0-F)
//!   - `<ans-256 fg="N" bg="N">` - 256-color mode (N=0-255, or "fg-#"/"bg-#" for CGA fallback)
//!   - `<ans-rgb fg="R,G,B" bg="R,G,B">` - 24-bit RGB mode (or "fg-#"/"bg-#" for CGA fallback)
//!
//! - **Soft returns**: Lines containing ANSI/BBS sequences automatically wrap at column 80.
//!
//! - **SAUCE metadata handling**: Parses SAUCE/COMNT records commonly appended to ANSI art
//!   files and displays metadata as `Key: Value` lines (Title, Author, Group, Date, Size,
//!   Font, Comment). Content after SAUCE records continues to be processed, allowing for
//!   BBS messages that contain ANSI art followed by additional text.
//!
//! - **Character handling**:
//!   - Carriage returns (`\r`) are suppressed
//!   - Newlines (`\n`) are preserved
//!   - HTML special characters (`<`, `>`, `&`, `"`, `'`) are escaped
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

/// SAUCE record data (Standard Architecture for Universal Comment Extensions)
#[derive(Debug, Clone, Default)]
struct SauceRecord {
    title: String,
    author: String,
    group: String,
    date: String,
    width: u16,
    height: u16,
    comments: Vec<String>,
    font: String,
}

impl SauceRecord {
    /// Parse SAUCE record from bytes starting at "SAUCE00"
    fn parse(data: &[u8], comnt_data: Option<&[u8]>) -> Option<Self> {
        if data.len() < 128 || &data[0..5] != b"SAUCE" {
            return None;
        }

        let mut record = SauceRecord::default();

        // Parse fields using CP437 decoding, trimming trailing spaces/nulls
        record.title = Self::decode_field(&data[7..42]);
        record.author = Self::decode_field(&data[42..62]);
        record.group = Self::decode_field(&data[62..82]);
        record.date = Self::decode_field(&data[82..90]);

        // TInfo1 = width, TInfo2 = height (little-endian u16)
        record.width = u16::from_le_bytes([data[96], data[97]]);
        record.height = u16::from_le_bytes([data[98], data[99]]);

        // TInfoS = font name (22 bytes, null-terminated string)
        record.font = Self::decode_field(&data[106..128]);

        // Parse comments if COMNT block provided
        if let Some(comnt) = comnt_data {
            if comnt.len() >= 5 && &comnt[0..5] == b"COMNT" {
                let comment_bytes = &comnt[5..];
                // Each comment line is 64 bytes
                for chunk in comment_bytes.chunks(64) {
                    let line = Self::decode_field(chunk);
                    if !line.is_empty() {
                        record.comments.push(line);
                    }
                }
            }
        }

        Some(record)
    }

    fn decode_field(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|&b| CP437_TO_UNICODE[b as usize])
            .collect::<String>()
            .trim_end_matches(|c: char| c == ' ' || c == '\0')
            .to_string()
    }

    /// Format SAUCE record as "Key: Value\n" lines
    fn format_output(&self) -> String {
        let mut output = String::new();

        if !self.title.is_empty() {
            output.push_str(&format!("Title: {}\n", self.title));
        }
        if !self.author.is_empty() {
            output.push_str(&format!("Author: {}\n", self.author));
        }
        if !self.group.is_empty() {
            output.push_str(&format!("Group: {}\n", self.group));
        }
        if !self.date.is_empty() {
            // Format date from CCYYMMDD to CCYY-MM-DD if valid
            if self.date.len() == 8 && self.date.chars().all(|c| c.is_ascii_digit()) {
                output.push_str(&format!(
                    "Date: {}-{}-{}\n",
                    &self.date[0..4],
                    &self.date[4..6],
                    &self.date[6..8]
                ));
            } else {
                output.push_str(&format!("Date: {}\n", self.date));
            }
        }
        if self.width > 0 || self.height > 0 {
            output.push_str(&format!("Size: {}x{}\n", self.width, self.height));
        }
        if !self.font.is_empty() {
            output.push_str(&format!("Font: {}\n", self.font));
        }
        for comment in &self.comments {
            output.push_str(&format!("Comment: {}\n", comment));
        }

        output
    }
}

/// Find SAUCE record position and COMNT block in data
/// Returns (sauce_start, comnt_start, after_sauce_start)
fn find_sauce_positions(data: &[u8]) -> (Option<usize>, Option<usize>, Option<usize>) {
    // SAUCE is always 128 bytes from the end (if present)
    // COMNT block (if present) is before SAUCE
    let mut sauce_pos = None;
    let mut comnt_pos = None;

    // Search for SAUCE00 marker
    if let Some(pos) = data
        .windows(7)
        .rposition(|w| w == b"SAUCE00")
    {
        sauce_pos = Some(pos);

        // Look for COMNT before SAUCE (within reasonable range)
        let search_start = pos.saturating_sub(64 * 256 + 5); // Max 255 comments * 64 bytes + "COMNT"
        if let Some(rel_pos) = data[search_start..pos]
            .windows(5)
            .rposition(|w| w == b"COMNT")
        {
            comnt_pos = Some(search_start + rel_pos);
        }
    }

    // Calculate position after SAUCE record
    let after_sauce = sauce_pos.map(|pos| pos + 128);

    (sauce_pos, comnt_pos, after_sauce)
}

/// Extended color mode for 256-color and RGB support
#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum ColorMode {
    /// Standard 16-color CGA mode (uses <ans-KF> tags)
    #[default]
    Cga,
    /// 256-color mode (uses <ans-256 fg="N" bg="N"> tags)
    Color256,
    /// 24-bit RGB mode (uses <ans-rgb fg="R,G,B" bg="R,G,B"> tags)
    Rgb,
}

/// Extended color value (for 256-color and RGB modes)
#[derive(Debug, Clone, Copy, PartialEq)]
enum ExtendedColor {
    /// CGA color fallback (0-15)
    Cga(u8),
    /// 256-color palette index (0-255)
    Palette(u8),
    /// 24-bit RGB color
    Rgb(u8, u8, u8),
}

impl Default for ExtendedColor {
    fn default() -> Self {
        ExtendedColor::Cga(7) // Default to light gray
    }
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
    /// Extended foreground color (for 256-color and RGB modes)
    ext_foreground: ExtendedColor,
    /// Extended background color (for 256-color and RGB modes)
    ext_background: ExtendedColor,
    /// Current color mode
    color_mode: ColorMode,
    output: String,
    current_column: u32,
    has_encountered_ansi: bool,
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
            ext_foreground: ExtendedColor::Cga(7),
            ext_background: ExtendedColor::Cga(0),
            color_mode: ColorMode::Cga,
            output: String::new(),
            current_column: 0,
            has_encountered_ansi: false,
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

    /// Format an extended color as a string attribute value
    fn format_ext_color(color: &ExtendedColor, is_foreground: bool) -> String {
        match color {
            ExtendedColor::Cga(c) => {
                let prefix = if is_foreground { "fg" } else { "bg" };
                format!("{}-{}", prefix, Self::color_to_hex(*c))
            }
            ExtendedColor::Palette(n) => n.to_string(),
            ExtendedColor::Rgb(r, g, b) => format!("{},{},{}", r, g, b),
        }
    }

    fn open_tag(&mut self) {
        match self.color_mode {
            ColorMode::Cga => {
                let bg = Self::color_to_hex(self.background);
                let fg = Self::color_to_hex(self.foreground);
                self.output.push_str(&format!("<ans-{}{}>", bg, fg));
            }
            ColorMode::Color256 => {
                let fg = Self::format_ext_color(&self.ext_foreground, true);
                let bg = Self::format_ext_color(&self.ext_background, false);
                self.output.push_str(&format!("<ans-256 fg=\"{}\" bg=\"{}\">", fg, bg));
            }
            ColorMode::Rgb => {
                let fg = Self::format_ext_color(&self.ext_foreground, true);
                let bg = Self::format_ext_color(&self.ext_background, false);
                self.output.push_str(&format!("<ans-rgb fg=\"{}\" bg=\"{}\">", fg, bg));
            }
        }
    }

    fn close_tag(&mut self) {
        match self.color_mode {
            ColorMode::Cga => {
                let bg = Self::color_to_hex(self.background);
                let fg = Self::color_to_hex(self.foreground);
                self.output.push_str(&format!("</ans-{}{}>", bg, fg));
            }
            ColorMode::Color256 => {
                self.output.push_str("</ans-256>");
            }
            ColorMode::Rgb => {
                self.output.push_str("</ans-rgb>");
            }
        }
    }

    /// Check if color state has changed (considering mode and extended colors)
    fn colors_changed(&self, new_mode: ColorMode, new_bg: u8, new_fg: u8,
                      new_ext_bg: ExtendedColor, new_ext_fg: ExtendedColor) -> bool {
        if new_mode != self.color_mode {
            return true;
        }
        match self.color_mode {
            ColorMode::Cga => new_bg != self.background || new_fg != self.foreground,
            ColorMode::Color256 | ColorMode::Rgb => {
                new_ext_bg != self.ext_background || new_ext_fg != self.ext_foreground
            }
        }
    }

    fn switch_color(&mut self, new_bg: u8, new_fg: u8) {
        // Stay in CGA mode
        let new_ext_fg = ExtendedColor::Cga(new_fg);
        let new_ext_bg = ExtendedColor::Cga(new_bg);
        if self.colors_changed(ColorMode::Cga, new_bg, new_fg, new_ext_bg, new_ext_fg) {
            self.close_tag();
            self.color_mode = ColorMode::Cga;
            self.background = new_bg;
            self.foreground = new_fg;
            self.ext_foreground = new_ext_fg;
            self.ext_background = new_ext_bg;
            self.open_tag();
        }
    }

    fn emit_char(&mut self, ch: char) {
        if self.save_position_active {
            return;
        }

        // Check for soft return at column 80 (only for CP437 mode with ANSI sequences)
        if !self.options.utf8_input && self.has_encountered_ansi && self.current_column >= 80 && ch != '\n' {
            self.output.push('\n');
            self.current_column = 0;
        }

        match ch {
            '<' => self.output.push_str("&lt;"),
            '>' => self.output.push_str("&gt;"),
            '&' => self.output.push_str("&amp;"),
            '"' => self.output.push_str("&quot;"),
            '\'' => self.output.push_str("&apos;"),
            '\n' => {
                self.output.push('\n');
                self.current_column = 0;
                // Note: has_encountered_ansi is NOT reset - it's a file-level flag
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

    /// Map ANSI color code (0-7) to CGA color code
    fn ansi_to_cga(ansi_color: u8) -> u8 {
        match ansi_color {
            0 => 0, // Black
            1 => 4, // Red
            2 => 2, // Green
            3 => 6, // Brown/Yellow
            4 => 1, // Blue
            5 => 5, // Magenta
            6 => 3, // Cyan
            7 => 7, // White/Light Gray
            _ => 7,
        }
    }

    /// Map bright ANSI color code (0-7) to CGA bright color code (8-15)
    fn ansi_to_cga_bright(ansi_color: u8) -> u8 {
        match ansi_color {
            0 => 8,  // Dark Gray
            1 => 12, // Light Red
            2 => 10, // Light Green
            3 => 14, // Yellow
            4 => 9,  // Light Blue
            5 => 13, // Light Magenta
            6 => 11, // Light Cyan
            7 => 15, // White
            _ => 15,
        }
    }

    fn process_sgr(&mut self, params: &str) {
        // SGR (Select Graphic Rendition) - handles color codes
        // Parse params as u16 to handle potential values > 255
        let params: Vec<u16> = if params.is_empty() {
            vec![0]
        } else {
            params
                .split(';')
                .filter_map(|s| s.parse().ok())
                .collect()
        };

        // Track pending state changes
        let mut new_fg = self.foreground;
        let mut new_bg = self.background;
        let mut new_mode = self.color_mode;
        let mut new_ext_fg = self.ext_foreground;
        let mut new_ext_bg = self.ext_background;

        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => {
                    // Reset - return to CGA mode with default colors
                    new_fg = 7;
                    new_bg = 0;
                    new_mode = ColorMode::Cga;
                    new_ext_fg = ExtendedColor::Cga(7);
                    new_ext_bg = ExtendedColor::Cga(0);
                }
                1 => {
                    // Bold/Bright - set high bit on foreground
                    new_fg |= 0x08;
                    // Also update extended color if it's CGA
                    if let ExtendedColor::Cga(c) = new_ext_fg {
                        new_ext_fg = ExtendedColor::Cga(c | 0x08);
                    }
                }
                2 | 22 => {
                    // Dim / Normal intensity - clear high bit
                    new_fg &= 0x07;
                    if let ExtendedColor::Cga(c) = new_ext_fg {
                        new_ext_fg = ExtendedColor::Cga(c & 0x07);
                    }
                }
                5 | 6 => {
                    // Blink - set high bit on background (in CGA terms)
                    new_bg |= 0x08;
                    if let ExtendedColor::Cga(c) = new_ext_bg {
                        new_ext_bg = ExtendedColor::Cga(c | 0x08);
                    }
                }
                25 => {
                    // Blink off
                    new_bg &= 0x07;
                    if let ExtendedColor::Cga(c) = new_ext_bg {
                        new_ext_bg = ExtendedColor::Cga(c & 0x07);
                    }
                }
                7 => {
                    // Reverse video
                    std::mem::swap(&mut new_fg, &mut new_bg);
                    std::mem::swap(&mut new_ext_fg, &mut new_ext_bg);
                }
                30..=37 => {
                    // Standard foreground colors - switch to CGA mode
                    let cga_color = Self::ansi_to_cga((params[i] - 30) as u8);
                    new_fg = (new_fg & 0x08) | cga_color;
                    new_mode = ColorMode::Cga;
                    new_ext_fg = ExtendedColor::Cga(new_fg);
                }
                38 => {
                    // Extended foreground color
                    if i + 1 < params.len() {
                        match params[i + 1] {
                            5 => {
                                // 256-color mode: ESC[38;5;Nm
                                if i + 2 < params.len() {
                                    let index = params[i + 2] as u8;
                                    new_mode = ColorMode::Color256;
                                    new_ext_fg = ExtendedColor::Palette(index);
                                    // Preserve background as CGA fallback if it was CGA
                                    if matches!(self.ext_background, ExtendedColor::Cga(_)) {
                                        new_ext_bg = ExtendedColor::Cga(new_bg);
                                    }
                                    i += 3;
                                    continue;
                                }
                            }
                            2 => {
                                // RGB mode: ESC[38;2;R;G;Bm
                                if i + 4 < params.len() {
                                    let r = params[i + 2] as u8;
                                    let g = params[i + 3] as u8;
                                    let b = params[i + 4] as u8;
                                    new_mode = ColorMode::Rgb;
                                    new_ext_fg = ExtendedColor::Rgb(r, g, b);
                                    // Preserve background as CGA fallback if it was CGA
                                    if matches!(self.ext_background, ExtendedColor::Cga(_)) {
                                        new_ext_bg = ExtendedColor::Cga(new_bg);
                                    }
                                    i += 5;
                                    continue;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                39 => {
                    // Default foreground
                    new_fg = 7;
                    new_ext_fg = ExtendedColor::Cga(7);
                    new_mode = ColorMode::Cga;
                }
                40..=47 => {
                    // Standard background colors - switch to CGA mode
                    let cga_color = Self::ansi_to_cga((params[i] - 40) as u8);
                    new_bg = (new_bg & 0x08) | cga_color;
                    new_mode = ColorMode::Cga;
                    new_ext_bg = ExtendedColor::Cga(new_bg);
                }
                48 => {
                    // Extended background color
                    if i + 1 < params.len() {
                        match params[i + 1] {
                            5 => {
                                // 256-color mode: ESC[48;5;Nm
                                if i + 2 < params.len() {
                                    let index = params[i + 2] as u8;
                                    // Use 256-color if fg is already extended, otherwise use current mode
                                    if matches!(new_ext_fg, ExtendedColor::Palette(_)) {
                                        new_mode = ColorMode::Color256;
                                    } else if matches!(new_ext_fg, ExtendedColor::Rgb(_, _, _)) {
                                        // Keep RGB mode if fg is RGB
                                    } else {
                                        new_mode = ColorMode::Color256;
                                    }
                                    new_ext_bg = ExtendedColor::Palette(index);
                                    // Preserve foreground as CGA fallback if it was CGA
                                    if matches!(self.ext_foreground, ExtendedColor::Cga(_)) && matches!(new_ext_fg, ExtendedColor::Cga(_)) {
                                        new_ext_fg = ExtendedColor::Cga(new_fg);
                                    }
                                    i += 3;
                                    continue;
                                }
                            }
                            2 => {
                                // RGB mode: ESC[48;2;R;G;Bm
                                if i + 4 < params.len() {
                                    let r = params[i + 2] as u8;
                                    let g = params[i + 3] as u8;
                                    let b = params[i + 4] as u8;
                                    // Use RGB mode if fg is already RGB, otherwise upgrade
                                    if matches!(new_ext_fg, ExtendedColor::Rgb(_, _, _)) {
                                        new_mode = ColorMode::Rgb;
                                    } else if matches!(new_ext_fg, ExtendedColor::Palette(_)) {
                                        // Keep 256-color mode if fg is 256
                                        new_mode = ColorMode::Color256;
                                    } else {
                                        new_mode = ColorMode::Rgb;
                                    }
                                    new_ext_bg = ExtendedColor::Rgb(r, g, b);
                                    // Preserve foreground as CGA fallback if it was CGA
                                    if matches!(self.ext_foreground, ExtendedColor::Cga(_)) && matches!(new_ext_fg, ExtendedColor::Cga(_)) {
                                        new_ext_fg = ExtendedColor::Cga(new_fg);
                                    }
                                    i += 5;
                                    continue;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                49 => {
                    // Default background
                    new_bg = 0;
                    new_ext_bg = ExtendedColor::Cga(0);
                    new_mode = ColorMode::Cga;
                }
                90..=97 => {
                    // Bright foreground colors - switch to CGA mode
                    let cga_color = Self::ansi_to_cga_bright((params[i] - 90) as u8);
                    new_fg = cga_color;
                    new_mode = ColorMode::Cga;
                    new_ext_fg = ExtendedColor::Cga(new_fg);
                }
                100..=107 => {
                    // Bright background colors - switch to CGA mode
                    let cga_color = Self::ansi_to_cga_bright((params[i] - 100) as u8);
                    new_bg = cga_color;
                    new_mode = ColorMode::Cga;
                    new_ext_bg = ExtendedColor::Cga(new_bg);
                }
                _ => {}
            }
            i += 1;
        }

        // Apply accumulated changes
        if self.colors_changed(new_mode, new_bg, new_fg, new_ext_bg, new_ext_fg) {
            self.close_tag();
            self.color_mode = new_mode;
            self.foreground = new_fg;
            self.background = new_bg;
            self.ext_foreground = new_ext_fg;
            self.ext_background = new_ext_bg;
            self.open_tag();
        }
    }

    fn process_csi(&mut self, params: &str, command: char) {
        self.has_encountered_ansi = true;

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
    ///
    /// Synchronet color codes use:
    /// - Lowercase letters (k,b,g,c,r,m,y,w) for foreground colors (0-7)
    /// - Uppercase letters (K,B,G,C,R,M,Y,W) for background colors (0-7)
    /// - h/H for high intensity modifier on foreground
    /// - i/I for blink/high intensity modifier on background
    fn process_synchronet_code(&mut self, code: u8) {
        self.has_encountered_ansi = true;
        let mut new_fg = self.foreground;
        let mut new_bg = self.background;

        match code {
            // Lowercase = foreground colors (sets base color, preserves intensity)
            b'k' => new_fg = (new_fg & 0x08) | 0,  // Black
            b'b' => new_fg = (new_fg & 0x08) | 1,  // Blue
            b'g' => new_fg = (new_fg & 0x08) | 2,  // Green
            b'c' => new_fg = (new_fg & 0x08) | 3,  // Cyan
            b'r' => new_fg = (new_fg & 0x08) | 4,  // Red
            b'm' => new_fg = (new_fg & 0x08) | 5,  // Magenta
            b'y' => new_fg = (new_fg & 0x08) | 6,  // Brown/Yellow
            b'w' => new_fg = (new_fg & 0x08) | 7,  // White/Light Gray

            // Uppercase = background colors (sets base color, preserves intensity)
            b'K' => new_bg = (new_bg & 0x08) | 0,  // Black
            b'B' => new_bg = (new_bg & 0x08) | 1,  // Blue
            b'G' => new_bg = (new_bg & 0x08) | 2,  // Green
            b'C' => new_bg = (new_bg & 0x08) | 3,  // Cyan
            b'R' => new_bg = (new_bg & 0x08) | 4,  // Red
            b'M' => new_bg = (new_bg & 0x08) | 5,  // Magenta
            b'Y' => new_bg = (new_bg & 0x08) | 6,  // Brown/Yellow
            b'W' => new_bg = (new_bg & 0x08) | 7,  // White/Light Gray

            // Digit codes (0-7) for background colors (legacy, sets base color)
            b'0' => new_bg = (new_bg & 0x08) | 0,  // Black
            b'1' => new_bg = (new_bg & 0x08) | 1,  // Blue
            b'2' => new_bg = (new_bg & 0x08) | 2,  // Green
            b'3' => new_bg = (new_bg & 0x08) | 3,  // Cyan
            b'4' => new_bg = (new_bg & 0x08) | 4,  // Red
            b'5' => new_bg = (new_bg & 0x08) | 5,  // Magenta
            b'6' => new_bg = (new_bg & 0x08) | 6,  // Brown
            b'7' => new_bg = (new_bg & 0x08) | 7,  // White/Light Gray

            // High intensity modifier for foreground
            b'H' | b'h' => new_fg |= 0x08,
            // Blink/high intensity modifier for background
            b'I' | b'i' => new_bg |= 0x08,
            // Normal - reset to default
            b'N' | b'n' => {
                new_fg = 7;
                new_bg = 0;
            }
            // Remove high intensity from foreground
            b'-' => new_fg &= 0x07,
            // Remove blink from background
            b'_' => new_bg &= 0x07,

            _ => {} // Unknown code, ignore
        }

        self.switch_color(new_bg, new_fg);
    }

    /// Process Renegade pipe color code (0-31)
    fn process_renegade_code(&mut self, code: u8) {
        self.has_encountered_ansi = true;
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

            // Background colors 16-23 (normal intensity)
            16 => new_bg = 0, // Black
            17 => new_bg = 1, // Blue
            18 => new_bg = 2, // Green
            19 => new_bg = 3, // Cyan
            20 => new_bg = 4, // Red
            21 => new_bg = 5, // Magenta
            22 => new_bg = 6, // Brown
            23 => new_bg = 7, // Light Gray

            // Background colors 24-31 (high intensity)
            24 => new_bg = 8,  // Dark Gray
            25 => new_bg = 9,  // Light Blue
            26 => new_bg = 10, // Light Green
            27 => new_bg = 11, // Light Cyan
            28 => new_bg = 12, // Light Red
            29 => new_bg = 13, // Light Magenta
            30 => new_bg = 14, // Yellow
            31 => new_bg = 15, // White

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
                        self.has_encountered_ansi = true;
                        self.parse_state = ParseState::Normal;
                    }
                    b'8' => {
                        // \e8 - Restore cursor position (DEC)
                        self.save_position_active = false;
                        self.has_encountered_ansi = true;
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
                } else if byte == b'|' {
                    // Escaped pipe (||), emit literal pipe
                    self.emit_char('|');
                    self.parse_state = ParseState::Normal;
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
                    if code <= 31 {
                        self.process_renegade_code(code);
                    }
                    // If code > 31, just ignore the sequence
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

        // Find SUB marker and SAUCE positions
        let sub_pos = input.iter().position(|&b| b == 0x1A);
        let (sauce_pos, comnt_pos, after_sauce_pos) = find_sauce_positions(input);

        // Determine content end position
        let content_end = sub_pos
            .or(comnt_pos)
            .or(sauce_pos)
            .unwrap_or(input.len());

        // Process content before SUB/SAUCE
        for &byte in &input[..content_end] {
            self.process_byte(byte);
        }

        // If SAUCE record exists, parse and output it
        if let Some(sauce_start) = sauce_pos {
            let comnt_data = comnt_pos.map(|cp| &input[cp..sauce_start]);
            if let Some(sauce) = SauceRecord::parse(&input[sauce_start..], comnt_data) {
                let sauce_output = sauce.format_output();
                if !sauce_output.is_empty() {
                    // Add newline before SAUCE metadata
                    self.emit_char('\n');
                    for ch in sauce_output.chars() {
                        self.emit_char(ch);
                    }
                }
            }

            // Check for content after SAUCE record
            if let Some(after_pos) = after_sauce_pos {
                if after_pos < input.len() {
                    let remaining = &input[after_pos..];
                    if !remaining.is_empty() && remaining.iter().any(|&b| b != 0 && b != 0x1A) {
                        // Add newline separator before continuing content
                        self.emit_char('\n');
                        for &byte in remaining {
                            if byte == 0x1A {
                                // Another SUB - recursively handle nested SAUCE
                                break;
                            }
                            self.process_byte(byte);
                        }
                    }
                }
            }
        }

        self.close_tag();
        self.output.push_str("</pre>");

        std::mem::take(&mut self.output)
    }

    fn convert_utf8(&mut self, input: &[u8]) -> String {
        self.output.push_str("<pre class=\"ansi\">");
        self.open_tag();

        // Find SUB marker and SAUCE positions (work on raw bytes)
        let sub_pos = input.iter().position(|&b| b == 0x1A);
        let (sauce_pos, comnt_pos, after_sauce_pos) = find_sauce_positions(input);

        // Determine content end position
        let content_end = sub_pos
            .or(comnt_pos)
            .or(sauce_pos)
            .unwrap_or(input.len());

        // Parse content as UTF-8
        let content = String::from_utf8_lossy(&input[..content_end]);
        for ch in content.chars() {
            self.process_utf8_char(ch);
        }

        // If SAUCE record exists, parse and output it
        if let Some(sauce_start) = sauce_pos {
            let comnt_data = comnt_pos.map(|cp| &input[cp..sauce_start]);
            if let Some(sauce) = SauceRecord::parse(&input[sauce_start..], comnt_data) {
                let sauce_output = sauce.format_output();
                if !sauce_output.is_empty() {
                    // Add newline before SAUCE metadata
                    self.emit_char('\n');
                    for ch in sauce_output.chars() {
                        self.emit_char(ch);
                    }
                }
            }

            // Check for content after SAUCE record
            if let Some(after_pos) = after_sauce_pos {
                if after_pos < input.len() {
                    let remaining = &input[after_pos..];
                    if !remaining.is_empty() && remaining.iter().any(|&b| b != 0 && b != 0x1A) {
                        // Add newline separator before continuing content
                        self.emit_char('\n');
                        let remaining_text = String::from_utf8_lossy(remaining);
                        for ch in remaining_text.chars() {
                            if ch == '\x1A' {
                                break;
                            }
                            self.process_utf8_char(ch);
                        }
                    }
                }
            }
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
                        self.has_encountered_ansi = true;
                        self.parse_state = ParseState::Normal;
                    }
                    '8' => {
                        self.save_position_active = false;
                        self.has_encountered_ansi = true;
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
                } else if ch == '|' {
                    // Escaped pipe (||), emit literal pipe
                    self.emit_char('|');
                    self.parse_state = ParseState::Normal;
                } else {
                    self.emit_char('|');
                    self.parse_state = ParseState::Normal;
                    self.process_utf8_char(ch);
                }
            }
            ParseState::RenegadePipe2(first_digit) => {
                if ch.is_ascii_digit() {
                    let code = first_digit * 10 + (ch as u8 - b'0');
                    if code <= 31 {
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
        // Test double quote
        let result = convert(b"\"quoted\"");
        assert!(result.contains("&quot;quoted&quot;"));
        // Test apostrophe
        let result = convert(b"it's here");
        assert!(result.contains("it&apos;s here"));
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
        // Ctrl-A + r (lowercase) = red foreground
        let input = b"\x01rRed Text";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-04>")); // Red on black
    }

    #[test]
    fn test_synchronet_background_color_uppercase() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + R (uppercase) = red background
        let input = b"\x01RRed BG";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-47>")); // Red bg (4), Light Gray fg (7)
    }

    #[test]
    fn test_synchronet_background_color_digit() {
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
    fn test_synchronet_high_intensity_foreground() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + b (blue fg) + Ctrl-A + h (high intensity) = bright blue
        let input = b"\x01b\x01hBright Blue";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-09>")); // Light Blue on black
    }

    #[test]
    fn test_synchronet_high_intensity_background() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + B (blue bg) + Ctrl-A + i (blink/high intensity bg) = bright blue bg
        let input = b"\x01B\x01iBright Blue BG";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-97>")); // Light Blue bg (9), Light Gray fg (7)
    }

    #[test]
    fn test_synchronet_normal_reset() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + r (red fg) then Ctrl-A + n = reset to normal
        let input = b"\x01rRed\x01nNormal";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-04>Red</ans-04>"));
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

    #[test]
    fn test_synchronet_preserves_intensity() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Set high intensity first, then change color - intensity should be preserved
        let input = b"\x01h\x01bBright Blue";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-09>")); // Light Blue (high intensity preserved)
    }

    #[test]
    fn test_synchronet_combined_fg_bg() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Ctrl-A + w (white fg) + Ctrl-A + B (blue bg)
        let input = b"\x01w\x01BWhite on Blue";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-17>")); // Blue bg (1), Light Gray fg (7)
    }

    #[test]
    fn test_synchronet_intensity_idempotent() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Applying high intensity multiple times should have same effect as once
        let input = b"\x01b\x01h\x01hDouble High";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-09>")); // Light Blue (9), not something weird
    }

    #[test]
    fn test_synchronet_blink_idempotent() {
        let options = ConvertOptions {
            synchronet_ctrl_a: true,
            ..Default::default()
        };
        // Applying blink/high bg multiple times should have same effect as once
        let input = b"\x01B\x01i\x01iDouble Blink BG";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-97>")); // Light Blue bg (9), Light Gray fg (7)
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

    // ========== SAUCE metadata parsing tests ==========

    #[test]
    fn test_sub_without_sauce_stops_processing() {
        // SUB without valid SAUCE record - content after SUB is ignored
        let input = b"Visible\x1aRandom garbage after SUB";
        let result = convert(input);
        assert!(result.contains("Visible"));
        assert!(!result.contains("Random"));
        assert!(!result.contains("garbage"));
    }

    #[test]
    fn test_sauce_record_parsed_and_displayed() {
        // Create a minimal valid SAUCE record (128 bytes)
        let mut input = b"Content before SAUCE\x1a".to_vec();
        // SAUCE00 header
        input.extend_from_slice(b"SAUCE00");
        // Title (35 bytes) - "Test Title" padded with spaces
        input.extend_from_slice(b"Test Title                         ");
        // Author (20 bytes)
        input.extend_from_slice(b"Test Author         ");
        // Group (20 bytes)
        input.extend_from_slice(b"Test Group          ");
        // Date (8 bytes) - CCYYMMDD
        input.extend_from_slice(b"20240115");
        // FileSize (4 bytes) - little endian
        input.extend_from_slice(&[0, 0, 0, 0]);
        // DataType (1 byte)
        input.push(1);
        // FileType (1 byte)
        input.push(1);
        // TInfo1-4 (8 bytes) - width=80, height=25
        input.extend_from_slice(&[80, 0, 25, 0, 0, 0, 0, 0]);
        // Comments (1 byte)
        input.push(0);
        // TFlags (1 byte)
        input.push(0);
        // TInfoS (22 bytes) - font name
        input.extend_from_slice(b"IBM VGA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");

        let result = convert(&input);
        assert!(result.contains("Content before SAUCE"));
        assert!(result.contains("Title: Test Title"));
        assert!(result.contains("Author: Test Author"));
        assert!(result.contains("Group: Test Group"));
        assert!(result.contains("Date: 2024-01-15"));
        assert!(result.contains("Size: 80x25"));
        assert!(result.contains("Font: IBM VGA"));
    }

    #[test]
    fn test_sauce_with_comnt_block() {
        // Create input with COMNT block before SAUCE
        let mut input = b"Art content\x1a".to_vec();
        // COMNT header + one 64-byte comment line
        input.extend_from_slice(b"COMNT");
        input.extend_from_slice(b"This is a comment line for the ANSI art.                       ");
        // SAUCE00 header
        input.extend_from_slice(b"SAUCE00");
        // Title (35 bytes)
        input.extend_from_slice(b"Artwork Title                      ");
        // Author (20 bytes)
        input.extend_from_slice(b"Artist              ");
        // Group (20 bytes)
        input.extend_from_slice(b"                    ");
        // Date (8 bytes)
        input.extend_from_slice(b"20230701");
        // FileSize (4 bytes)
        input.extend_from_slice(&[0, 0, 0, 0]);
        // DataType, FileType
        input.extend_from_slice(&[1, 1]);
        // TInfo1-4 (8 bytes)
        input.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
        // Comments count (1 byte) - 1 comment
        input.push(1);
        // TFlags (1 byte)
        input.push(0);
        // TInfoS (22 bytes)
        input.extend_from_slice(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");

        let result = convert(&input);
        assert!(result.contains("Art content"));
        assert!(result.contains("Title: Artwork Title"));
        assert!(result.contains("Author: Artist"));
        assert!(result.contains("Comment: This is a comment line for the ANSI art."));
    }

    #[test]
    fn test_content_after_sauce_continues() {
        // Create input with content after SAUCE record
        let mut input = b"Before SAUCE\x1a".to_vec();
        // Minimal SAUCE record (128 bytes)
        input.extend_from_slice(b"SAUCE00");
        input.extend_from_slice(b"Title                              "); // 35
        input.extend_from_slice(b"                    "); // 20 author
        input.extend_from_slice(b"                    "); // 20 group
        input.extend_from_slice(b"        "); // 8 date
        input.extend_from_slice(&[0u8; 4]); // filesize
        input.extend_from_slice(&[0, 0]); // datatype, filetype
        input.extend_from_slice(&[0u8; 8]); // tinfo1-4
        input.push(0); // comments
        input.push(0); // tflags
        input.extend_from_slice(&[0u8; 22]); // tinfos
        // Content after SAUCE
        input.extend_from_slice(b"Content after SAUCE record");

        let result = convert(&input);
        assert!(result.contains("Before SAUCE"));
        assert!(result.contains("Title: Title"));
        assert!(result.contains("Content after SAUCE record"));
    }

    #[test]
    fn test_sauce_utf8_mode() {
        let options = ConvertOptions {
            utf8_input: true,
            ..Default::default()
        };
        // Create input with UTF-8 content and SAUCE
        let mut input = b"Hello UTF-8 \xc3\xa9\x1a".to_vec(); // é in UTF-8
        // Full SAUCE record (128 bytes total)
        // SAUCE00 (7) + Title (35) + Author (20) + Group (20) + Date (8) +
        // FileSize (4) + DataType (1) + FileType (1) + TInfo1-4 (8) +
        // Comments (1) + TFlags (1) + TInfoS (22) = 128
        input.extend_from_slice(b"SAUCE00");                           // 7 bytes
        input.extend_from_slice(b"UTF-8 Test                         "); // 35 bytes
        input.extend_from_slice(&[b' '; 20]);                          // 20 bytes author
        input.extend_from_slice(&[b' '; 20]);                          // 20 bytes group
        input.extend_from_slice(b"        ");                          // 8 bytes date
        input.extend_from_slice(&[0u8; 4]);                            // 4 bytes filesize
        input.extend_from_slice(&[1, 1]);                              // 2 bytes datatype, filetype
        input.extend_from_slice(&[0u8; 8]);                            // 8 bytes tinfo1-4
        input.push(0);                                                 // 1 byte comments
        input.push(0);                                                 // 1 byte tflags
        input.extend_from_slice(&[0u8; 22]);                           // 22 bytes tinfos

        let result = convert_with_options(&input, &options);
        assert!(result.contains("Hello UTF-8 é"));
        assert!(result.contains("Title: UTF-8 Test"));
    }

    // ========== 256-color and RGB support tests ==========

    #[test]
    fn test_256_color_foreground() {
        // ESC[38;5;196m = 256-color foreground, color 196 (bright red in cube)
        let input = b"\x1b[38;5;196mRed 256";
        let result = convert(input);
        assert!(result.contains("<ans-256 fg=\"196\" bg=\"bg-0\">"));
        assert!(result.contains("Red 256"));
        assert!(result.contains("</ans-256>"));
    }

    #[test]
    fn test_256_color_background() {
        // ESC[48;5;21m = 256-color background, color 21 (blue in cube)
        let input = b"\x1b[48;5;21mBlue BG";
        let result = convert(input);
        assert!(result.contains("<ans-256 fg=\"fg-7\" bg=\"21\">"));
        assert!(result.contains("Blue BG"));
    }

    #[test]
    fn test_256_color_both() {
        // ESC[38;5;226;48;5;21m = yellow fg (226) on blue bg (21)
        let input = b"\x1b[38;5;226;48;5;21mYellow on Blue";
        let result = convert(input);
        assert!(result.contains("<ans-256 fg=\"226\" bg=\"21\">"));
    }

    #[test]
    fn test_rgb_foreground() {
        // ESC[38;2;255;128;0m = RGB foreground (orange)
        let input = b"\x1b[38;2;255;128;0mOrange";
        let result = convert(input);
        assert!(result.contains("<ans-rgb fg=\"255,128,0\" bg=\"bg-0\">"));
        assert!(result.contains("Orange"));
        assert!(result.contains("</ans-rgb>"));
    }

    #[test]
    fn test_rgb_background() {
        // ESC[48;2;0;64;128m = RGB background (dark blue)
        let input = b"\x1b[48;2;0;64;128mDark Blue BG";
        let result = convert(input);
        assert!(result.contains("<ans-rgb fg=\"fg-7\" bg=\"0,64,128\">"));
        assert!(result.contains("Dark Blue BG"));
    }

    #[test]
    fn test_rgb_both() {
        // ESC[38;2;255;255;0;48;2;128;0;128m = yellow fg on purple bg
        let input = b"\x1b[38;2;255;255;0;48;2;128;0;128mYellow on Purple";
        let result = convert(input);
        assert!(result.contains("<ans-rgb fg=\"255,255,0\" bg=\"128,0,128\">"));
    }

    #[test]
    fn test_extended_color_reset() {
        // Start with 256-color, then reset to default
        let input = b"\x1b[38;5;196mRed\x1b[0mNormal";
        let result = convert(input);
        assert!(result.contains("<ans-256 fg=\"196\""));
        assert!(result.contains("Red"));
        assert!(result.contains("</ans-256>"));
        assert!(result.contains("<ans-07>Normal"));
    }

    #[test]
    fn test_switch_cga_to_256() {
        // Start with CGA red, then switch to 256-color
        let input = b"\x1b[31mCGA Red\x1b[38;5;196m256 Red";
        let result = convert(input);
        assert!(result.contains("<ans-04>CGA Red</ans-04>"));
        assert!(result.contains("<ans-256 fg=\"196\""));
        assert!(result.contains("256 Red"));
    }

    #[test]
    fn test_switch_256_to_rgb() {
        // Start with 256-color, then switch to RGB
        let input = b"\x1b[38;5;196m256\x1b[38;2;255;0;0mRGB";
        let result = convert(input);
        assert!(result.contains("<ans-256"));
        assert!(result.contains("256"));
        assert!(result.contains("<ans-rgb fg=\"255,0,0\""));
        assert!(result.contains("RGB"));
    }

    #[test]
    fn test_256_color_cga_range() {
        // 256-color palette indices 0-15 are the standard CGA colors
        // Test index 4 (red in 256-color, which maps to CGA red)
        let input = b"\x1b[38;5;4mBlue";
        let result = convert(input);
        assert!(result.contains("<ans-256 fg=\"4\""));
    }

    #[test]
    fn test_256_color_grayscale() {
        // Test grayscale colors (232-255)
        let input = b"\x1b[38;5;240mGray";
        let result = convert(input);
        assert!(result.contains("<ans-256 fg=\"240\""));
    }

    // ========== Renegade escaped pipe tests ==========

    #[test]
    fn test_renegade_escaped_pipe() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // || should output a single | and continue
        let input = b"||Hello";
        let result = convert_with_options(input, &options);
        assert!(result.contains("|Hello"));
    }

    #[test]
    fn test_renegade_escaped_pipe_followed_by_digits() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // ||04 should output |04 (literal pipe followed by 04)
        let input = b"||04Red";
        let result = convert_with_options(input, &options);
        assert!(result.contains("|04Red"));
    }

    #[test]
    fn test_renegade_high_intensity_background() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |24 = dark gray background (high intensity black)
        let input = b"|24Dark Gray BG";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-87>")); // Dark Gray bg (8), Light Gray fg (7)
    }

    #[test]
    fn test_renegade_high_intensity_background_range() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |31 = white background (high intensity)
        let input = b"|31White BG";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-f7>")); // White bg (f), Light Gray fg (7)
    }

    #[test]
    fn test_renegade_combined_high_intensity_bg_with_fg() {
        let options = ConvertOptions {
            renegade_pipe: true,
            ..Default::default()
        };
        // |00 = black fg, |28 = light red background
        let input = b"|00|28Black on Light Red";
        let result = convert_with_options(input, &options);
        assert!(result.contains("<ans-c0>")); // Light Red bg (c), Black fg (0)
    }
}

