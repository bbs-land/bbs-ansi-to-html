// ANSI color web components
(function() {
  const colors = [
    "#000000", "#0000AA", "#00AA00", "#00AAAA",
    "#AA0000", "#AA00AA", "#AA5500", "#AAAAAA",
    "#555555", "#5555FF", "#55FF55", "#55FFFF",
    "#FF5555", "#FF55FF", "#FFFF55", "#FFFFFF"
  ];

  const hexChars = "0123456789abcdef";

  // Standard 16-color CGA web components: <ans-KF>
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

  /**
   * Parse a color attribute value.
   * Supports:
   * - "fg-#" or "bg-#" for CGA fallback (# is hex digit 0-f)
   * - Numeric string for 256-color palette index
   * - "R,G,B" for RGB values
   *
   * @param {string} value - The attribute value
   * @param {boolean} isForeground - Whether this is a foreground color
   * @returns {string} CSS color value
   */
  function parseColorAttribute(value, isForeground) {
    if (!value) {
      // Default: light gray foreground, black background
      return isForeground ? colors[7] : colors[0];
    }

    // Check for CGA fallback: "fg-#" or "bg-#"
    const cgaMatch = value.match(/^(fg|bg)-([0-9a-f])$/i);
    if (cgaMatch) {
      const colorIndex = parseInt(cgaMatch[2], 16);
      return colors[colorIndex] || (isForeground ? colors[7] : colors[0]);
    }

    // Check for RGB: "R,G,B"
    const rgbMatch = value.match(/^(\d{1,3}),(\d{1,3}),(\d{1,3})$/);
    if (rgbMatch) {
      const r = Math.min(255, parseInt(rgbMatch[1], 10));
      const g = Math.min(255, parseInt(rgbMatch[2], 10));
      const b = Math.min(255, parseInt(rgbMatch[3], 10));
      return `rgb(${r},${g},${b})`;
    }

    // Check for 256-color palette index
    const paletteIndex = parseInt(value, 10);
    if (!isNaN(paletteIndex) && paletteIndex >= 0 && paletteIndex <= 255) {
      return palette256ToRgb(paletteIndex);
    }

    // Fallback to default
    return isForeground ? colors[7] : colors[0];
  }

  /**
   * Convert a 256-color palette index to RGB CSS value.
   * @param {number} index - Palette index (0-255)
   * @returns {string} CSS rgb() color value
   */
  function palette256ToRgb(index) {
    // Colors 0-15: Standard CGA colors
    if (index < 16) {
      return colors[index];
    }

    // Colors 16-231: 6x6x6 color cube
    if (index < 232) {
      const cubeIndex = index - 16;
      const r = Math.floor(cubeIndex / 36);
      const g = Math.floor((cubeIndex % 36) / 6);
      const b = cubeIndex % 6;
      // Each component: 0, 95, 135, 175, 215, 255
      const toValue = (c) => c === 0 ? 0 : 55 + c * 40;
      return `rgb(${toValue(r)},${toValue(g)},${toValue(b)})`;
    }

    // Colors 232-255: Grayscale ramp
    const gray = (index - 232) * 10 + 8;
    return `rgb(${gray},${gray},${gray})`;
  }

  // 256-color web component: <ans-256 fg="N" bg="N">
  if (!customElements.get("ans-256")) {
    class Ans256Element extends HTMLElement {
      constructor() {
        super();
      }

      connectedCallback() {
        const fg = this.getAttribute("fg");
        const bg = this.getAttribute("bg");
        this.style.color = parseColorAttribute(fg, true);
        this.style.backgroundColor = parseColorAttribute(bg, false);
        this.style.display = "inline";
      }

      static get observedAttributes() {
        return ["fg", "bg"];
      }

      attributeChangedCallback(name, oldValue, newValue) {
        if (name === "fg") {
          this.style.color = parseColorAttribute(newValue, true);
        } else if (name === "bg") {
          this.style.backgroundColor = parseColorAttribute(newValue, false);
        }
      }
    }

    customElements.define("ans-256", Ans256Element);
  }

  // RGB color web component: <ans-rgb fg="R,G,B" bg="R,G,B">
  if (!customElements.get("ans-rgb")) {
    class AnsRgbElement extends HTMLElement {
      constructor() {
        super();
      }

      connectedCallback() {
        const fg = this.getAttribute("fg");
        const bg = this.getAttribute("bg");
        this.style.color = parseColorAttribute(fg, true);
        this.style.backgroundColor = parseColorAttribute(bg, false);
        this.style.display = "inline";
      }

      static get observedAttributes() {
        return ["fg", "bg"];
      }

      attributeChangedCallback(name, oldValue, newValue) {
        if (name === "fg") {
          this.style.color = parseColorAttribute(newValue, true);
        } else if (name === "bg") {
          this.style.backgroundColor = parseColorAttribute(newValue, false);
        }
      }
    }

    customElements.define("ans-rgb", AnsRgbElement);
  }
})();
