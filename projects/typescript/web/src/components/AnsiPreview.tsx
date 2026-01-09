import { useEffect, useRef } from 'react';

interface AnsiPreviewProps {
  html: string;
}

/**
 * Component that renders ANSI HTML output with custom web components.
 */
export function AnsiPreview({ html }: AnsiPreviewProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  

  // The `ansi-display.js` script (defines `ans-*` custom elements) should
  // be loaded from the application's public assets (wwwroot/ansi-display.js).
  // No runtime generation or injection is performed here.

  // Update innerHTML when html changes
  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.innerHTML = html;
    }
  }, [html]);

  return <div ref={containerRef} />;
}
