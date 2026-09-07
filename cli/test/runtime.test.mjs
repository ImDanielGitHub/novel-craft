import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const bin = fileURLToPath(new URL('../index.mjs', import.meta.url));
const temp = () => fs.mkdtempSync(path.join(os.tmpdir(), 'novel-craft-test-'));
function cli(root, args, status = 0) {
  const result = spawnSync(process.execPath, [bin, ...args, '--project', root, '--json'], {encoding:'utf8'});
  assert.equal(result.status, status, result.stderr || result.stdout);
  const value = JSON.parse(result.stdout);
  assert.equal(value.schema_version, 1);
  return value.data ?? value.error;
}
test('CLI emits a versioned discovery contract', () => {
  const result = spawnSync(process.execPath, [bin, 'schema', '--json'], {encoding:'utf8'});
  assert.equal(result.status, 0, 'new agent-native CLI must exist and expose schema');
  const body = JSON.parse(result.stdout);
  assert.equal(body.schema_version, 1);
  assert.ok(body.data.commands.includes('generate'));
});
test('accepted sourced memory appears in a fresh-process context', () => {
  const root = temp();
  try {
    cli(root, ['init', '--title', 'Glass Harbour', '--idea', 'A courier returns a stolen letter.', '--genre', 'mystery']);
    const prose = 'Mara handed the brass key to Ivo. Neither of them mentioned the unopened letter.\n';
    fs.writeFileSync(path.join(root, 'input.md'), prose);
    fs.writeFileSync(path.join(root, 'facts.json'), JSON.stringify([{subject:'Ivo',predicate:'owns',value:'brass key',kind:'world',known_by:['Mara','Ivo'],quote:'Mara handed the brass key to Ivo.'}]));
    const p = cli(root, ['import', '--file', path.join(root,'input.md'), '--chapter','1','--facts',path.join(root,'facts.json')]);
    cli(root, ['commit',p.id,'--expect',p.base_revision,'--accept-facts']);
    const context = cli(root,['context','--chapter','2']);
    assert.ok(context.facts.some(f=>f.subject==='Ivo' && f.value==='brass key'));
    assert.equal(fs.readFileSync(path.join(root,'input.md'),'utf8'),prose);
    assert.equal(context.facts[0].source.quote,'Mara handed the brass key to Ivo.');
  } finally { fs.rmSync(root,{recursive:true,force:true}); }
});
test('external edits are never overwritten by a stale proposal', () => {
  const root = temp();
  try {
    cli(root,['init','--title','Test','--idea','A quiet family dinner.','--genre','drama']);
    const input=path.join(root,'draft.md');fs.writeFileSync(input,'The table was set for three.\n');
    let p=cli(root,['import','--file',input,'--chapter','1']);cli(root,['commit',p.id,'--expect',p.base_revision]);
    fs.writeFileSync(input,'The table was set for four.\n');p=cli(root,['import','--file',input,'--chapter','1']);
    const manuscript=path.join(root,'manuscript','chapter-0001.md');fs.writeFileSync(manuscript,'A human changed this.\n');
    const error=cli(root,['commit',p.id,'--expect',p.base_revision],3);
    assert.equal(error.code,'SOURCE_CHANGED');assert.equal(fs.readFileSync(manuscript,'utf8'),'A human changed this.\n');
  } finally { fs.rmSync(root,{recursive:true,force:true}); }
});
test('unknown genres fail rather than falling back to isekai', () => {
  const root=temp();try {const error=cli(root,['init','--title','T','--idea','I','--genre','invented-unknown-label'],2);assert.equal(error.code,'UNKNOWN_GENRE');}
  finally {fs.rmSync(root,{recursive:true,force:true});}
});
