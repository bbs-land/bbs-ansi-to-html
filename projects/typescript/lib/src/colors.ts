/**
 * CGA color hex values indexed by color code (0-15).
 */
export const CGA_COLORS: string[] = [
  '#000000', // 0 - Black
  '#0000AA', // 1 - Blue
  '#00AA00', // 2 - Green
  '#00AAAA', // 3 - Cyan
  '#AA0000', // 4 - Red
  '#AA00AA', // 5 - Magenta
  '#AA5500', // 6 - Brown
  '#AAAAAA', // 7 - Light Gray
  '#555555', // 8 - Dark Gray
  '#5555FF', // 9 - Light Blue
  '#55FF55', // a - Light Green
  '#55FFFF', // b - Light Cyan
  '#FF5555', // c - Light Red
  '#FF55FF', // d - Light Magenta
  '#FFFF55', // e - Yellow
  '#FFFFFF', // f - White
];

/**
 * Convert a color code (0-15) to lowercase hex character (0-9, a-f).
 */
export function colorToHex(color: number): string {
  if (color >= 0 && color <= 9) {
    return String.fromCharCode(0x30 + color); // '0'-'9'
  }
  if (color >= 10 && color <= 15) {
    return String.fromCharCode(0x61 + color - 10); // 'a'-'f'
  }
  return '0';
}
