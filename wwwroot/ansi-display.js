// ANSI color web components
(function() {
  const colors = [
    "#000000", "#0000AA", "#00AA00", "#00AAAA",
    "#AA0000", "#AA00AA", "#AA5500", "#AAAAAA",
    "#555555", "#5555FF", "#55FF55", "#55FFFF",
    "#FF5555", "#FF55FF", "#FFFF55", "#FFFFFF"
  ];

  const hexChars = "0123456789abcdef";

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
