import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';

test('the OIDC release job installs only the reviewed exact npm toolchain', () => {
  const workflow = fs.readFileSync(new URL('../../.github/workflows/release.yml', import.meta.url), 'utf8');
  const install = workflow.match(/^      - name: Ensure OIDC-capable npm\r?\n        run: (.+)$/m);
  assert.ok(install, 'The release npm bootstrap must remain explicit and reviewable.');
  assert.equal(install[1], 'npm install --global --ignore-scripts --no-audit --no-fund npm@11.19.1');
  assert.deepEqual(workflow.match(/npm@[^\s'"`]+/g), ['npm@11.19.1'], 'No mutable npm bootstrap or fallback may bypass the reviewed pin.');
});
