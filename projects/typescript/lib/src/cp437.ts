/**
 * CP437 to Unicode character mapping table.
 * Maps all 256 CP437 byte values to their Unicode equivalents.
 */
export const CP437_TO_UNICODE: string[] = [
  // 0x00-0x0F: Control characters and special symbols
  '\u0000', // 0x00 - NUL (kept as null for compatibility)
  '\u263A', // 0x01 - White Smiling Face ☺
  '\u263B', // 0x02 - Black Smiling Face ☻
  '\u2665', // 0x03 - Black Heart Suit ♥
  '\u2666', // 0x04 - Black Diamond Suit ♦
  '\u2663', // 0x05 - Black Club Suit ♣
  '\u2660', // 0x06 - Black Spade Suit ♠
  '\u2022', // 0x07 - Bullet •
  '\u25D8', // 0x08 - Inverse Bullet ◘
  '\u25CB', // 0x09 - White Circle ○
  '\u25D9', // 0x0A - Inverse White Circle ◙
  '\u2642', // 0x0B - Male Sign ♂
  '\u2640', // 0x0C - Female Sign ♀
  '\u266A', // 0x0D - Eighth Note ♪
  '\u266B', // 0x0E - Beamed Eighth Notes ♫
  '\u263C', // 0x0F - White Sun With Rays ☼

  // 0x10-0x1F: Arrows and special symbols
  '\u25BA', // 0x10 - Black Right-Pointing Pointer ►
  '\u25C4', // 0x11 - Black Left-Pointing Pointer ◄
  '\u2195', // 0x12 - Up Down Arrow ↕
  '\u203C', // 0x13 - Double Exclamation Mark ‼
  '\u00B6', // 0x14 - Pilcrow Sign ¶
  '\u00A7', // 0x15 - Section Sign §
  '\u25AC', // 0x16 - Black Rectangle ▬
  '\u21A8', // 0x17 - Up Down Arrow With Base ↨
  '\u2191', // 0x18 - Upwards Arrow ↑
  '\u2193', // 0x19 - Downwards Arrow ↓
  '\u2192', // 0x1A - Rightwards Arrow →
  '\u2190', // 0x1B - Leftwards Arrow ←
  '\u221F', // 0x1C - Right Angle ∟
  '\u2194', // 0x1D - Left Right Arrow ↔
  '\u25B2', // 0x1E - Black Up-Pointing Triangle ▲
  '\u25BC', // 0x1F - Black Down-Pointing Triangle ▼

  // 0x20-0x7E: Standard ASCII printable characters
  ' ', '!', '"', '#', '$', '%', '&', "'", '(', ')', '*', '+', ',', '-', '.', '/',
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
  '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
  'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_',
  '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
  'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~',

  // 0x7F: Delete (House symbol in CP437)
  '\u2302', // 0x7F - House ⌂

  // 0x80-0x9F: Extended Latin characters
  '\u00C7', // 0x80 - Latin Capital Letter C With Cedilla Ç
  '\u00FC', // 0x81 - Latin Small Letter U With Diaeresis ü
  '\u00E9', // 0x82 - Latin Small Letter E With Acute é
  '\u00E2', // 0x83 - Latin Small Letter A With Circumflex â
  '\u00E4', // 0x84 - Latin Small Letter A With Diaeresis ä
  '\u00E0', // 0x85 - Latin Small Letter A With Grave à
  '\u00E5', // 0x86 - Latin Small Letter A With Ring Above å
  '\u00E7', // 0x87 - Latin Small Letter C With Cedilla ç
  '\u00EA', // 0x88 - Latin Small Letter E With Circumflex ê
  '\u00EB', // 0x89 - Latin Small Letter E With Diaeresis ë
  '\u00E8', // 0x8A - Latin Small Letter E With Grave è
  '\u00EF', // 0x8B - Latin Small Letter I With Diaeresis ï
  '\u00EE', // 0x8C - Latin Small Letter I With Circumflex î
  '\u00EC', // 0x8D - Latin Small Letter I With Grave ì
  '\u00C4', // 0x8E - Latin Capital Letter A With Diaeresis Ä
  '\u00C5', // 0x8F - Latin Capital Letter A With Ring Above Å

  // 0x90-0x9F: More extended Latin
  '\u00C9', // 0x90 - Latin Capital Letter E With Acute É
  '\u00E6', // 0x91 - Latin Small Letter Ae æ
  '\u00C6', // 0x92 - Latin Capital Letter Ae Æ
  '\u00F4', // 0x93 - Latin Small Letter O With Circumflex ô
  '\u00F6', // 0x94 - Latin Small Letter O With Diaeresis ö
  '\u00F2', // 0x95 - Latin Small Letter O With Grave ò
  '\u00FB', // 0x96 - Latin Small Letter U With Circumflex û
  '\u00F9', // 0x97 - Latin Small Letter U With Grave ù
  '\u00FF', // 0x98 - Latin Small Letter Y With Diaeresis ÿ
  '\u00D6', // 0x99 - Latin Capital Letter O With Diaeresis Ö
  '\u00DC', // 0x9A - Latin Capital Letter U With Diaeresis Ü
  '\u00A2', // 0x9B - Cent Sign ¢
  '\u00A3', // 0x9C - Pound Sign £
  '\u00A5', // 0x9D - Yen Sign ¥
  '\u20A7', // 0x9E - Peseta Sign ₧
  '\u0192', // 0x9F - Latin Small Letter F With Hook ƒ

  // 0xA0-0xAF: More extended characters
  '\u00E1', // 0xA0 - Latin Small Letter A With Acute á
  '\u00ED', // 0xA1 - Latin Small Letter I With Acute í
  '\u00F3', // 0xA2 - Latin Small Letter O With Acute ó
  '\u00FA', // 0xA3 - Latin Small Letter U With Acute ú
  '\u00F1', // 0xA4 - Latin Small Letter N With Tilde ñ
  '\u00D1', // 0xA5 - Latin Capital Letter N With Tilde Ñ
  '\u00AA', // 0xA6 - Feminine Ordinal Indicator ª
  '\u00BA', // 0xA7 - Masculine Ordinal Indicator º
  '\u00BF', // 0xA8 - Inverted Question Mark ¿
  '\u2310', // 0xA9 - Reversed Not Sign ⌐
  '\u00AC', // 0xAA - Not Sign ¬
  '\u00BD', // 0xAB - Vulgar Fraction One Half ½
  '\u00BC', // 0xAC - Vulgar Fraction One Quarter ¼
  '\u00A1', // 0xAD - Inverted Exclamation Mark ¡
  '\u00AB', // 0xAE - Left-Pointing Double Angle Quotation Mark «
  '\u00BB', // 0xAF - Right-Pointing Double Angle Quotation Mark »

  // 0xB0-0xBF: Box drawing light shade and corners
  '\u2591', // 0xB0 - Light Shade ░
  '\u2592', // 0xB1 - Medium Shade ▒
  '\u2593', // 0xB2 - Dark Shade ▓
  '\u2502', // 0xB3 - Box Drawings Light Vertical │
  '\u2524', // 0xB4 - Box Drawings Light Vertical And Left ┤
  '\u2561', // 0xB5 - Box Drawings Vertical Single And Left Double ╡
  '\u2562', // 0xB6 - Box Drawings Vertical Double And Left Single ╢
  '\u2556', // 0xB7 - Box Drawings Down Double And Left Single ╖
  '\u2555', // 0xB8 - Box Drawings Down Single And Left Double ╕
  '\u2563', // 0xB9 - Box Drawings Double Vertical And Left ╣
  '\u2551', // 0xBA - Box Drawings Double Vertical ║
  '\u2557', // 0xBB - Box Drawings Double Down And Left ╗
  '\u255D', // 0xBC - Box Drawings Double Up And Left ╝
  '\u255C', // 0xBD - Box Drawings Up Double And Left Single ╜
  '\u255B', // 0xBE - Box Drawings Up Single And Left Double ╛
  '\u2510', // 0xBF - Box Drawings Light Down And Left ┐

  // 0xC0-0xCF: More box drawing
  '\u2514', // 0xC0 - Box Drawings Light Up And Right └
  '\u2534', // 0xC1 - Box Drawings Light Up And Horizontal ┴
  '\u252C', // 0xC2 - Box Drawings Light Down And Horizontal ┬
  '\u251C', // 0xC3 - Box Drawings Light Vertical And Right ├
  '\u2500', // 0xC4 - Box Drawings Light Horizontal ─
  '\u253C', // 0xC5 - Box Drawings Light Vertical And Horizontal ┼
  '\u255E', // 0xC6 - Box Drawings Vertical Single And Right Double ╞
  '\u255F', // 0xC7 - Box Drawings Vertical Double And Right Single ╟
  '\u255A', // 0xC8 - Box Drawings Double Up And Right ╚
  '\u2554', // 0xC9 - Box Drawings Double Down And Right ╔
  '\u2569', // 0xCA - Box Drawings Double Up And Horizontal ╩
  '\u2566', // 0xCB - Box Drawings Double Down And Horizontal ╦
  '\u2560', // 0xCC - Box Drawings Double Vertical And Right ╠
  '\u2550', // 0xCD - Box Drawings Double Horizontal ═
  '\u256C', // 0xCE - Box Drawings Double Vertical And Horizontal ╬
  '\u2567', // 0xCF - Box Drawings Up Single And Horizontal Double ╧

  // 0xD0-0xDF: More box drawing and blocks
  '\u2568', // 0xD0 - Box Drawings Up Double And Horizontal Single ╨
  '\u2564', // 0xD1 - Box Drawings Down Single And Horizontal Double ╤
  '\u2565', // 0xD2 - Box Drawings Down Double And Horizontal Single ╥
  '\u2559', // 0xD3 - Box Drawings Up Double And Right Single ╙
  '\u2558', // 0xD4 - Box Drawings Up Single And Right Double ╘
  '\u2552', // 0xD5 - Box Drawings Down Single And Right Double ╒
  '\u2553', // 0xD6 - Box Drawings Down Double And Right Single ╓
  '\u256B', // 0xD7 - Box Drawings Vertical Double And Horizontal Single ╫
  '\u256A', // 0xD8 - Box Drawings Vertical Single And Horizontal Double ╪
  '\u2518', // 0xD9 - Box Drawings Light Up And Left ┘
  '\u250C', // 0xDA - Box Drawings Light Down And Right ┌
  '\u2588', // 0xDB - Full Block █
  '\u2584', // 0xDC - Lower Half Block ▄
  '\u258C', // 0xDD - Left Half Block ▌
  '\u2590', // 0xDE - Right Half Block ▐
  '\u2580', // 0xDF - Upper Half Block ▀

  // 0xE0-0xEF: Greek letters
  '\u03B1', // 0xE0 - Greek Small Letter Alpha α
  '\u00DF', // 0xE1 - Latin Small Letter Sharp S ß
  '\u0393', // 0xE2 - Greek Capital Letter Gamma Γ
  '\u03C0', // 0xE3 - Greek Small Letter Pi π
  '\u03A3', // 0xE4 - Greek Capital Letter Sigma Σ
  '\u03C3', // 0xE5 - Greek Small Letter Sigma σ
  '\u00B5', // 0xE6 - Micro Sign µ
  '\u03C4', // 0xE7 - Greek Small Letter Tau τ
  '\u03A6', // 0xE8 - Greek Capital Letter Phi Φ
  '\u0398', // 0xE9 - Greek Capital Letter Theta Θ
  '\u03A9', // 0xEA - Greek Capital Letter Omega Ω
  '\u03B4', // 0xEB - Greek Small Letter Delta δ
  '\u221E', // 0xEC - Infinity ∞
  '\u03C6', // 0xED - Greek Small Letter Phi φ
  '\u03B5', // 0xEE - Greek Small Letter Epsilon ε
  '\u2229', // 0xEF - Intersection ∩

  // 0xF0-0xFF: Math symbols and special characters
  '\u2261', // 0xF0 - Identical To ≡
  '\u00B1', // 0xF1 - Plus-Minus Sign ±
  '\u2265', // 0xF2 - Greater-Than Or Equal To ≥
  '\u2264', // 0xF3 - Less-Than Or Equal To ≤
  '\u2320', // 0xF4 - Top Half Integral ⌠
  '\u2321', // 0xF5 - Bottom Half Integral ⌡
  '\u00F7', // 0xF6 - Division Sign ÷
  '\u2248', // 0xF7 - Almost Equal To ≈
  '\u00B0', // 0xF8 - Degree Sign °
  '\u2219', // 0xF9 - Bullet Operator ∙
  '\u00B7', // 0xFA - Middle Dot ·
  '\u221A', // 0xFB - Square Root √
  '\u207F', // 0xFC - Superscript Latin Small Letter N ⁿ
  '\u00B2', // 0xFD - Superscript Two ²
  '\u25A0', // 0xFE - Black Square ■
  '\u00A0', // 0xFF - No-Break Space
];
