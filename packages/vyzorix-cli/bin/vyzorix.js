#!/usr/bin/env node

/**
 * Executes the compiled output of the CLI module natively.
 */
import('../dist/index.js').catch(err => {
  console.error("Failed to boot Vyzorix CLI:", err);
  process.exit(1);
});
