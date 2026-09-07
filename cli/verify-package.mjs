#!/usr/bin/env node
// Verify the distribution itself, not a source-tree launcher with implicit dependencies.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {VERSION} from './util.mjs';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'novel-craft-package-'));
const npmCandidates=[process.env.npm_execpath,path.join(path.dirname(process.execPath),'node_modules/npm/bin/npm-cli.js'),path.resolve(path.dirname(process.execPath),'../lib/node_modules/npm/bin/npm-cli.js')].filter(Boolean);
const npmCli=npmCandidates.find(p=>fs.existsSync(p));
const environment={...process.env,npm_config_cache:path.join(scratch,'cache'),npm_config_update_notifier:'false'};
function execute(executable,args,cwd=root){
  const run=spawnSync(executable,args,{cwd,env:environment,encoding:'utf8',timeout:45000,windowsHide:true,shell:false,maxBuffer:16*1024*1024});
  assert.equal(run.status,0,`${path.basename(executable)} failed: ${run.error?.message??''}\n${run.stdout}\n${run.stderr}`);
  return run.stdout;
}
try {
  assert.ok(npmCli,'Could not locate npm-cli.js. Run this check through npm run verify:package.');
  const pkg=JSON.parse(fs.readFileSync(path.join(root,'package.json'),'utf8'));
  assert.equal(pkg.version,VERSION);assert.equal(pkg.name,'novel-craft');assert.equal(pkg.bin['novel-craft'],'cli/index.mjs');
  assert.equal(Object.keys(pkg.dependencies??{}).length,0,'The runtime must remain dependency-free.');
  const packed=JSON.parse(execute(process.execPath,[npmCli,'pack','--json','--ignore-scripts','--pack-destination',scratch]))[0];
  const files=new Set(packed.files.map(f=>f.path));
  for(const required of ['package.json','README.md','LICENSE','cli/index.mjs','cli/store.mjs','cli/generate.mjs','cli/runner.mjs','cli/catalogue.mjs','cli/SKILL.md','docs/CLI.md','docs/REBIRTH.md'])assert.ok(files.has(required),`Missing distribution file: ${required}`);
  for(const p of files)assert.ok(/^(package\.json|README\.md|LICENSE|cli\/[a-z-]+\.mjs|cli\/SKILL\.md|docs\/(CLI|REBIRTH)\.md)$/.test(p),`Unexpected packed file: ${p}`);
  const prefix=path.join(scratch,'installation');
  execute(process.execPath,[npmCli,'install','--prefix',prefix,'--offline','--ignore-scripts','--no-audit','--no-fund','--package-lock=false',path.join(scratch,packed.filename)]);
  const installed=path.join(prefix,'node_modules','novel-craft','cli','index.mjs');
  assert.equal(execute(process.execPath,[installed,'--version']).trim(),VERSION);
  const alias=JSON.parse(execute(process.execPath,[npmCli,'exec','--offline','--prefix',prefix,'--','novel-craft','schema','--json'],prefix));
  assert.equal(alias.ok,true);assert.ok(alias.data.commands.includes('generate'));
  const workspace=path.join(scratch,'book');
  const invoke=args=>{
    const data=JSON.parse(execute(process.execPath,[installed,...args,'--project',workspace,'--json'],scratch));
    assert.equal(data.ok,true);return data.data;
  };
  invoke(['init','--title','A clean installation','--genre','mystery']);
  const source=path.join(scratch,'chapter.md');fs.writeFileSync(source,'Mira placed the letter beneath the blue cup.\n');
  const facts=path.join(scratch,'facts.json');fs.writeFileSync(facts,JSON.stringify([{subject:'letter',predicate:'location',value:'beneath blue cup',kind:'world',known_by:['Mira'],quote:'Mira placed the letter beneath the blue cup.'}]));
  const proposal=invoke(['import','--file',source,'--chapter','1','--facts',facts]);
  invoke(['commit',proposal.id,'--expect',proposal.base_revision,'--accept-facts']);
  const context=invoke(['context','--chapter','2']);assert.ok(context.facts.some(f=>f.subject==='letter'));
  const epub=path.join(scratch,'book.epub');invoke(['export','--format','epub','--out',epub]);assert.equal(fs.readFileSync(epub).readUInt32LE(0),0x04034b50);
  process.stdout.write(JSON.stringify({ok:true,version:VERSION,packed_files:files.size,packed_bytes:packed.size,integrity:packed.integrity,checks:['distribution allowlist','offline clean installation','installed version','actual executable alias','installed canon propagation','installed EPUB export']})+'\n');
} finally {
  fs.rmSync(scratch,{recursive:true,force:true});
}
