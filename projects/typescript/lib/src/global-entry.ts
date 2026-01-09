// Entry point for the build that exposes a single global function `ansiToHtml`
// This file imports the library's `ansiToHtml` implementation and attaches a
// thin wrapper function to the global object so consumers get a single global
// `ansiToHtml` function regardless of environment.

import { ansiToHtml as _impl } from './index';

// Create a simple wrapper function compatible with older JS runtimes.
function ansiToHtml() {
  // forward arguments to implementation
  return (_impl as any).apply(null, arguments as any);
}

// Attach the wrapper to the global object (browser, webworker, or node-like)
(() => {
  // select a global object in various environments
  // Prefer `globalThis` where available; it's supported in modern browsers
  // and Node. Fall back to undefined if not present.
  const g: any = (typeof globalThis !== 'undefined' && (globalThis as any)) || undefined;

  try {
    g.ansiToHtml = ansiToHtml;
  } catch (e) {
    // best-effort - if assignment fails, nothing else to do
  }
})();

export {};
