import test from 'node:test';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
test('the packed package installs offline and runs independently of the source tree',()=>{
  const script=fileURLToPath(new URL('../verify-package.mjs',import.meta.url));
  const run=spawnSync(process.execPath,[script],{encoding:'utf8',timeout:60000});
  assert.equal(run.status,0,run.stdout+'\n'+run.stderr);
  const report=JSON.parse(run.stdout);assert.equal(report.ok,true);assert.equal(report.version,'0.2.0');
  assert.ok(report.checks.includes('installed canon propagation'));assert.ok(report.checks.includes('actual executable alias'));
});
