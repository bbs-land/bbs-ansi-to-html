//! Code Page 437 to Unicode mapping table
//!
//! This module provides a complete mapping from IBM Code Page 437 byte values
//! to their Unicode equivalents.

/// CP437 to Unicode lookup table
/// Index is the CP437 byte value, value is the Unicode character
pub const CP437_TO_UNICODE: [char; 256] = [
    // 0x00 - 0x0F: Control characters and special symbols
    '\u{0000}', // 0x00 - NUL (null)
    '\u{263A}', // 0x01 - White smiling face
    '\u{263B}', // 0x02 - Black smiling face
    '\u{2665}', // 0x03 - Black heart suit
    '\u{2666}', // 0x04 - Black diamond suit
    '\u{2663}', // 0x05 - Black club suit
    '\u{2660}', // 0x06 - Black spade suit
    '\u{2022}', // 0x07 - Bullet
    '\u{25D8}', // 0x08 - Inverse bullet
    '\u{25CB}', // 0x09 - White circle
    '\u{25D9}', // 0x0A - Inverse white circle
    '\u{2642}', // 0x0B - Male sign
    '\u{2640}', // 0x0C - Female sign
    '\u{266A}', // 0x0D - Eighth note
    '\u{266B}', // 0x0E - Beamed eighth notes
    '\u{263C}', // 0x0F - White sun with rays
    // 0x10 - 0x1F: More special symbols
    '\u{25BA}', // 0x10 - Black right-pointing pointer
    '\u{25C4}', // 0x11 - Black left-pointing pointer
    '\u{2195}', // 0x12 - Up down arrow
    '\u{203C}', // 0x13 - Double exclamation mark
    '\u{00B6}', // 0x14 - Pilcrow sign
    '\u{00A7}', // 0x15 - Section sign
    '\u{25AC}', // 0x16 - Black rectangle
    '\u{21A8}', // 0x17 - Up down arrow with base
    '\u{2191}', // 0x18 - Upwards arrow
    '\u{2193}', // 0x19 - Downwards arrow
    '\u{2192}', // 0x1A - Rightwards arrow
    '\u{2190}', // 0x1B - Leftwards arrow
    '\u{221F}', // 0x1C - Right angle
    '\u{2194}', // 0x1D - Left right arrow
    '\u{25B2}', // 0x1E - Black up-pointing triangle
    '\u{25BC}', // 0x1F - Black down-pointing triangle
    // 0x20 - 0x7E: Standard ASCII printable characters
    ' ',  // 0x20 - Space
    '!',  // 0x21
    '"',  // 0x22
    '#',  // 0x23
    '$',  // 0x24
    '%',  // 0x25
    '&',  // 0x26
    '\'', // 0x27
    '(',  // 0x28
    ')',  // 0x29
    '*',  // 0x2A
    '+',  // 0x2B
    ',',  // 0x2C
    '-',  // 0x2D
    '.',  // 0x2E
    '/',  // 0x2F
    '0',  // 0x30
    '1',  // 0x31
    '2',  // 0x32
    '3',  // 0x33
    '4',  // 0x34
    '5',  // 0x35
    '6',  // 0x36
    '7',  // 0x37
    '8',  // 0x38
    '9',  // 0x39
    ':',  // 0x3A
    ';',  // 0x3B
    '<',  // 0x3C
    '=',  // 0x3D
    '>',  // 0x3E
    '?',  // 0x3F
    '@',  // 0x40
    'A',  // 0x41
    'B',  // 0x42
    'C',  // 0x43
    'D',  // 0x44
    'E',  // 0x45
    'F',  // 0x46
    'G',  // 0x47
    'H',  // 0x48
    'I',  // 0x49
    'J',  // 0x4A
    'K',  // 0x4B
    'L',  // 0x4C
    'M',  // 0x4D
    'N',  // 0x4E
    'O',  // 0x4F
    'P',  // 0x50
    'Q',  // 0x51
    'R',  // 0x52
    'S',  // 0x53
    'T',  // 0x54
    'U',  // 0x55
    'V',  // 0x56
    'W',  // 0x57
    'X',  // 0x58
    'Y',  // 0x59
    'Z',  // 0x5A
    '[',  // 0x5B
    '\\', // 0x5C
    ']',  // 0x5D
    '^',  // 0x5E
    '_',  // 0x5F
    '`',  // 0x60
    'a',  // 0x61
    'b',  // 0x62
    'c',  // 0x63
    'd',  // 0x64
    'e',  // 0x65
    'f',  // 0x66
    'g',  // 0x67
    'h',  // 0x68
    'i',  // 0x69
    'j',  // 0x6A
    'k',  // 0x6B
    'l',  // 0x6C
    'm',  // 0x6D
    'n',  // 0x6E
    'o',  // 0x6F
    'p',  // 0x70
    'q',  // 0x71
    'r',  // 0x72
    's',  // 0x73
    't',  // 0x74
    'u',  // 0x75
    'v',  // 0x76
    'w',  // 0x77
    'x',  // 0x78
    'y',  // 0x79
    'z',  // 0x7A
    '{',  // 0x7B
    '|',  // 0x7C
    '}',  // 0x7D
    '~',  // 0x7E
    '\u{2302}', // 0x7F - House
    // 0x80 - 0x9F: Extended Latin characters
    '\u{00C7}', // 0x80 - Latin capital letter C with cedilla
    '\u{00FC}', // 0x81 - Latin small letter u with diaeresis
    '\u{00E9}', // 0x82 - Latin small letter e with acute
    '\u{00E2}', // 0x83 - Latin small letter a with circumflex
    '\u{00E4}', // 0x84 - Latin small letter a with diaeresis
    '\u{00E0}', // 0x85 - Latin small letter a with grave
    '\u{00E5}', // 0x86 - Latin small letter a with ring above
    '\u{00E7}', // 0x87 - Latin small letter c with cedilla
    '\u{00EA}', // 0x88 - Latin small letter e with circumflex
    '\u{00EB}', // 0x89 - Latin small letter e with diaeresis
    '\u{00E8}', // 0x8A - Latin small letter e with grave
    '\u{00EF}', // 0x8B - Latin small letter i with diaeresis
    '\u{00EE}', // 0x8C - Latin small letter i with circumflex
    '\u{00EC}', // 0x8D - Latin small letter i with grave
    '\u{00C4}', // 0x8E - Latin capital letter A with diaeresis
    '\u{00C5}', // 0x8F - Latin capital letter A with ring above
    '\u{00C9}', // 0x90 - Latin capital letter E with acute
    '\u{00E6}', // 0x91 - Latin small letter ae
    '\u{00C6}', // 0x92 - Latin capital letter AE
    '\u{00F4}', // 0x93 - Latin small letter o with circumflex
    '\u{00F6}', // 0x94 - Latin small letter o with diaeresis
    '\u{00F2}', // 0x95 - Latin small letter o with grave
    '\u{00FB}', // 0x96 - Latin small letter u with circumflex
    '\u{00F9}', // 0x97 - Latin small letter u with grave
    '\u{00FF}', // 0x98 - Latin small letter y with diaeresis
    '\u{00D6}', // 0x99 - Latin capital letter O with diaeresis
    '\u{00DC}', // 0x9A - Latin capital letter U with diaeresis
    '\u{00A2}', // 0x9B - Cent sign
    '\u{00A3}', // 0x9C - Pound sign
    '\u{00A5}', // 0x9D - Yen sign
    '\u{20A7}', // 0x9E - Peseta sign
    '\u{0192}', // 0x9F - Latin small letter f with hook
    // 0xA0 - 0xAF: More extended characters
    '\u{00E1}', // 0xA0 - Latin small letter a with acute
    '\u{00ED}', // 0xA1 - Latin small letter i with acute
    '\u{00F3}', // 0xA2 - Latin small letter o with acute
    '\u{00FA}', // 0xA3 - Latin small letter u with acute
    '\u{00F1}', // 0xA4 - Latin small letter n with tilde
    '\u{00D1}', // 0xA5 - Latin capital letter N with tilde
    '\u{00AA}', // 0xA6 - Feminine ordinal indicator
    '\u{00BA}', // 0xA7 - Masculine ordinal indicator
    '\u{00BF}', // 0xA8 - Inverted question mark
    '\u{2310}', // 0xA9 - Reversed not sign
    '\u{00AC}', // 0xAA - Not sign
    '\u{00BD}', // 0xAB - Vulgar fraction one half
    '\u{00BC}', // 0xAC - Vulgar fraction one quarter
    '\u{00A1}', // 0xAD - Inverted exclamation mark
    '\u{00AB}', // 0xAE - Left-pointing double angle quotation mark
    '\u{00BB}', // 0xAF - Right-pointing double angle quotation mark
    // 0xB0 - 0xBF: Box drawing light
    '\u{2591}', // 0xB0 - Light shade
    '\u{2592}', // 0xB1 - Medium shade
    '\u{2593}', // 0xB2 - Dark shade
    '\u{2502}', // 0xB3 - Box drawings light vertical
    '\u{2524}', // 0xB4 - Box drawings light vertical and left
    '\u{2561}', // 0xB5 - Box drawings vertical single and left double
    '\u{2562}', // 0xB6 - Box drawings vertical double and left single
    '\u{2556}', // 0xB7 - Box drawings down double and left single
    '\u{2555}', // 0xB8 - Box drawings down single and left double
    '\u{2563}', // 0xB9 - Box drawings double vertical and left
    '\u{2551}', // 0xBA - Box drawings double vertical
    '\u{2557}', // 0xBB - Box drawings double down and left
    '\u{255D}', // 0xBC - Box drawings double up and left
    '\u{255C}', // 0xBD - Box drawings up double and left single
    '\u{255B}', // 0xBE - Box drawings up single and left double
    '\u{2510}', // 0xBF - Box drawings light down and left
    // 0xC0 - 0xCF: More box drawing
    '\u{2514}', // 0xC0 - Box drawings light up and right
    '\u{2534}', // 0xC1 - Box drawings light up and horizontal
    '\u{252C}', // 0xC2 - Box drawings light down and horizontal
    '\u{251C}', // 0xC3 - Box drawings light vertical and right
    '\u{2500}', // 0xC4 - Box drawings light horizontal
    '\u{253C}', // 0xC5 - Box drawings light vertical and horizontal
    '\u{255E}', // 0xC6 - Box drawings vertical single and right double
    '\u{255F}', // 0xC7 - Box drawings vertical double and right single
    '\u{255A}', // 0xC8 - Box drawings double up and right
    '\u{2554}', // 0xC9 - Box drawings double down and right
    '\u{2569}', // 0xCA - Box drawings double up and horizontal
    '\u{2566}', // 0xCB - Box drawings double down and horizontal
    '\u{2560}', // 0xCC - Box drawings double vertical and right
    '\u{2550}', // 0xCD - Box drawings double horizontal
    '\u{256C}', // 0xCE - Box drawings double vertical and horizontal
    '\u{2567}', // 0xCF - Box drawings up single and horizontal double
    // 0xD0 - 0xDF: More box drawing and blocks
    '\u{2568}', // 0xD0 - Box drawings up double and horizontal single
    '\u{2564}', // 0xD1 - Box drawings down single and horizontal double
    '\u{2565}', // 0xD2 - Box drawings down double and horizontal single
    '\u{2559}', // 0xD3 - Box drawings up double and right single
    '\u{2558}', // 0xD4 - Box drawings up single and right double
    '\u{2552}', // 0xD5 - Box drawings down single and right double
    '\u{2553}', // 0xD6 - Box drawings down double and right single
    '\u{256B}', // 0xD7 - Box drawings vertical double and horizontal single
    '\u{256A}', // 0xD8 - Box drawings vertical single and horizontal double
    '\u{2518}', // 0xD9 - Box drawings light up and left
    '\u{250C}', // 0xDA - Box drawings light down and right
    '\u{2588}', // 0xDB - Full block
    '\u{2584}', // 0xDC - Lower half block
    '\u{258C}', // 0xDD - Left half block
    '\u{2590}', // 0xDE - Right half block
    '\u{2580}', // 0xDF - Upper half block
    // 0xE0 - 0xEF: Greek letters and math symbols
    '\u{03B1}', // 0xE0 - Greek small letter alpha
    '\u{00DF}', // 0xE1 - Latin small letter sharp s
    '\u{0393}', // 0xE2 - Greek capital letter Gamma
    '\u{03C0}', // 0xE3 - Greek small letter pi
    '\u{03A3}', // 0xE4 - Greek capital letter Sigma
    '\u{03C3}', // 0xE5 - Greek small letter sigma
    '\u{00B5}', // 0xE6 - Micro sign
    '\u{03C4}', // 0xE7 - Greek small letter tau
    '\u{03A6}', // 0xE8 - Greek capital letter Phi
    '\u{0398}', // 0xE9 - Greek capital letter Theta
    '\u{03A9}', // 0xEA - Greek capital letter Omega
    '\u{03B4}', // 0xEB - Greek small letter delta
    '\u{221E}', // 0xEC - Infinity
    '\u{03C6}', // 0xED - Greek small letter phi
    '\u{03B5}', // 0xEE - Greek small letter epsilon
    '\u{2229}', // 0xEF - Intersection
    // 0xF0 - 0xFF: More math and special symbols
    '\u{2261}', // 0xF0 - Identical to
    '\u{00B1}', // 0xF1 - Plus-minus sign
    '\u{2265}', // 0xF2 - Greater-than or equal to
    '\u{2264}', // 0xF3 - Less-than or equal to
    '\u{2320}', // 0xF4 - Top half integral
    '\u{2321}', // 0xF5 - Bottom half integral
    '\u{00F7}', // 0xF6 - Division sign
    '\u{2248}', // 0xF7 - Almost equal to
    '\u{00B0}', // 0xF8 - Degree sign
    '\u{2219}', // 0xF9 - Bullet operator
    '\u{00B7}', // 0xFA - Middle dot
    '\u{221A}', // 0xFB - Square root
    '\u{207F}', // 0xFC - Superscript latin small letter n
    '\u{00B2}', // 0xFD - Superscript two
    '\u{25A0}', // 0xFE - Black square
    '\u{00A0}', // 0xFF - No-break space
];
