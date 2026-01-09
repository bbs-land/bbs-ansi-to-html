import { defineConfig, loadEnv } from 'npm:vite@^6.0.0';
import react from 'npm:@vitejs/plugin-react@^4.3.4';
import * as path from 'jsr:@std/path';

/**
 * Search upward from a directory for a wwwroot folder.
 */
function findWwwrootUpward(startDir: string, maxLevels: number): string | null {
  let dir = startDir;
  for (let i = 0; i <= maxLevels; i++) {
    const candidate = path.join(dir, 'wwwroot');
    try {
      const stat = Deno.statSync(candidate);
      if (stat.isDirectory) {
        return candidate;
      }
    } catch {
      // Directory doesn't exist, continue
    }
    const parent = path.dirname(dir);
    if (parent === dir) break; // Reached root
    dir = parent;
  }
  return null;
}

/**
 * Find the wwwroot directory using the same logic as ansi-display-rs.
 */
function getWwwrootPath(): string {
  // 1. Check WWWROOT environment variable
  const wwwrootEnv = Deno.env.get('WWWROOT');
  if (wwwrootEnv) {
    const resolved = path.isAbsolute(wwwrootEnv)
      ? wwwrootEnv
      : path.resolve(Deno.cwd(), wwwrootEnv);
    try {
      const stat = Deno.statSync(resolved);
      if (stat.isDirectory) {
        return resolved;
      }
    } catch {
      // Fall through
    }
  }

  // 2. Search from cwd upward
  const fromCwd = findWwwrootUpward(Deno.cwd(), 3);
  if (fromCwd) return fromCwd;

  // 3. Search from this file's directory upward
  const thisDir = path.dirname(path.fromFileUrl(import.meta.url));
  const fromFile = findWwwrootUpward(thisDir, 3);
  if (fromFile) return fromFile;

  // 4. Fallback to /var/www/html
  try {
    const stat = Deno.statSync('/var/www/html');
    if (stat.isDirectory) {
      return '/var/www/html';
    }
  } catch {
    // Fall through
  }

  // Default to workspace root wwwroot
  return path.resolve(thisDir, '../../wwwroot');
}

const wwwrootPath = getWwwrootPath();
console.log(`Using wwwroot: ${wwwrootPath}`);

const port = parseInt(
  Deno.env.get('HTTP_PORT') || Deno.env.get('PORT') || '3000',
  10,
);

const baseConfig = {
  // base: '/bbs-ansi-to-html/',
  plugins: [react()],
  publicDir: wwwrootPath,
  server: {
    port,
  },
  preview: {
    port,
  },
  build: {
    outDir: 'dist',
  },
  resolve: {
    alias: {
      '@bbs/ansi-to-html': path.resolve(
        path.dirname(path.fromFileUrl(import.meta.url)),
        '../lib/src/index.ts',
      ),
    },
  },
};

export default defineConfig(({ command, mode }) => {
  const env = loadEnv(mode, Deno.cwd(), '');

  if (env.REPO_NAME) {
    return Object.assign(
      {}, 
      baseConfig, 
      {
        base: `/${env.REPO_NAME}/`
      }
    );
  }

  return baseConfig;
});
