import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import * as store from '../store.mjs';
import {PROFILES,OBSERVED_LABELS,getProfile,coverage} from '../catalogue.mjs';
import {context} from '../context.mjs';
import {checkText,validateReview} from '../checks.mjs';
import {render} from '../export.mjs';
const bin=fileURLToPath(new URL('../index.mjs',import.meta.url));
const fixture=fileURLToPath(new URL('./fixtures/runner.mjs',import.meta.url));
const runner=['--runner-command',process.execPath,'--runner-arg',fixture];
const temp=()=>fs.mkdtempSync(path.join(os.tmpdir(),'novel-craft-integration-'));
function call(root,args,expected=0){const r=spawnSync(process.execPath,[bin,...args,'--project',root,'--json'],{encoding:'utf8',timeout:20000});assert.equal(r.status,expected,r.stdout+'\n'+r.stderr);assert.equal(r.stderr,'');const out=JSON.parse(r.stdout);assert.equal(out.schema_version,1);return out.data??out.error;}
function workspace(){const root=temp();call(root,['init','--title','The Undelivered Letter','--idea','A courier returns a letter.','--genre','mystery']);return root;}
function proposed(root,number=1,prose='Mara kept the letter.\n',facts=[]){return store.propose(root,[{number,title:'The letter',prose,summary:'A letter is kept.',facts}]);}
function accepted(root,number=1,prose='Mara kept the letter.\n',facts=[]){const p=proposed(root,number,prose,facts);store.commit(root,p.id,p.base_revision);return p;}
const cleanup=root=>fs.rmSync(root,{recursive:true,force:true});
test('every observed catalogue label resolves with an original example and attributable references',()=>{
  assert.equal(coverage().observed_labels,OBSERVED_LABELS.length);
  for(const label of OBSERVED_LABELS){const p=getProfile(label);assert.ok(p.example.prose.length>80,label);assert.ok(p.source_ids.length,label);assert.ok(p.serial_engine.length>40,label);}
  assert.equal(new Set(PROFILES.map(p=>p.example.prose)).size,PROFILES.length);
  assert.equal(coverage().requested_source.status.includes('unverified'),true);
});
test('new novels go through a real subprocess protocol and one accepted transaction',()=>{
  const root=workspace();try{
    const r=call(root,['generate','--chapters','2','--words','80',...runner,'--accept','--accept-facts']);
    assert.equal(r.status,'complete');assert.equal(r.calls,5);assert.equal(r.chapters.length,2);
    assert.equal(store.load(root).chapters.length,2);assert.equal(store.load(root).receipts.filter(r=>r.operation==='commit').length,1);
    assert.ok(context(root,{chapter:3}).facts.some(f=>f.subject==='Mara'));
    const book=render(root,'md');assert.ok(book.includes('Mara handed'));assert.ok(book.includes('Ivo returned'));
  }finally{cleanup(root);}
});
test('default generation is proposal-only, not silent manuscript acceptance',()=>{
  const root=workspace();try{const r=call(root,['generate',...runner]);assert.equal(r.accepted,false);assert.equal(r.status,'proposed');assert.equal(store.load(root).chapters.length,0);assert.ok(store.getProposal(root,r.proposal_id).changes[0].prose.includes('Mara'));}finally{cleanup(root);}
});
test('create starts a new project and actually generates prose',()=>{
  const root=temp();const book=path.join(root,'new-book');try{const r=call(root,['create',book,'--idea','A courier returns a letter.','--genre','romance',...runner]);assert.equal(r.chapters.length,1);assert.ok(store.getProposal(book,r.proposal_id).changes[0].prose.length>50);}finally{cleanup(root);}
});
test('interrupted generation resumes the saved draft instead of starting over',()=>{
  const root=workspace();const marker=path.join(root,'fail-once');try{
    const adapter=[...runner,'--runner-arg','fail-review-once','--runner-arg',marker];
    const e=call(root,['generate','--chapters','2','--max-calls','10',...adapter],4);assert.equal(e.code,'RUNNER_FAILED');
    let run=store.readRun(root,e.details.run_id);assert.equal(run.stage,'review');assert.ok(run.current.prose);assert.equal(store.load(root).chapters.length,0);
    const r=call(root,['generate','--resume',run.id,...adapter,'--accept']);assert.equal(r.accepted,true);assert.equal(r.calls,6);
    run=store.readRun(root,run.id);assert.equal(run.attempts.filter(a=>a.stage==='draft').length,2);
  }finally{cleanup(root);}
});
test('model call budgets stop a run and can be explicitly increased on resume',()=>{
  const root=workspace();try{const e=call(root,['generate','--max-calls','1',...runner],4);assert.equal(e.code,'CALL_BUDGET');assert.equal(store.readRun(root,e.details.run_id).stage,'draft');const r=call(root,['generate','--resume',e.details.run_id,'--max-calls','5',...runner]);assert.equal(r.status,'proposed');assert.equal(r.calls,3);}finally{cleanup(root);}
});
test('an unresolved major review blocks delegated acceptance',()=>{
  const root=workspace();try{const r=call(root,['generate','--passes','0',...runner,'--runner-arg','major','--accept'],5);assert.equal(r.status,'needs-review');assert.equal(r.accepted,false);assert.equal(store.load(root).chapters.length,0);}finally{cleanup(root);}
});
test('malformed model JSON does not change the manuscript',()=>{
  const root=workspace();try{const revision=store.load(root).revision;const e=call(root,['generate',...runner,'--runner-arg','invalid'],4);assert.equal(e.code,'INVALID_MODEL_JSON');assert.equal(store.load(root).revision,revision);}finally{cleanup(root);}
});
test('a reviewer cannot cite invented excerpts',()=>{
  const root=workspace();try{const e=call(root,['generate',...runner,'--runner-arg','bad-quote'],4);assert.equal(e.code,'UNSUPPORTED_REVIEW');assert.equal(store.load(root).chapters.length,0);}finally{cleanup(root);}
});
test('runner timeout preserves a resumable checkpoint',()=>{
  const root=workspace();try{const e=call(root,['generate',...runner,'--runner-arg','timeout','--timeout-ms','100'],4);assert.equal(e.code,'RUNNER_TIMEOUT');assert.ok(store.readRun(root,e.details.run_id));}finally{cleanup(root);}
});
test('candidate facts require explicit approval and POV filtering excludes unknown facts',()=>{
  const root=workspace();try{
    accepted(root,1,'Ivo holds the key.\n',[{subject:'Ivo',predicate:'holds',value:'key',kind:'world',known_by:['Ivo'],quote:'Ivo holds the key.'}]);
    assert.equal(context(root,{chapter:2}).facts.length,0);
    const state=store.load(root);store.acceptFact(root,state.facts[0].id,state.revision);
    assert.equal(context(root,{chapter:2}).facts.length,1);assert.equal(context(root,{chapter:2,pov:'Mara'}).facts.length,0);assert.equal(context(root,{chapter:2,pov:'Ivo'}).facts.length,1);
  }finally{cleanup(root);}
});
test('context omits examples unless explicitly requested, without duplicating the catalogue',()=>{
  const root=workspace();try{const c=context(root);assert.ok(!JSON.stringify(c).includes(getProfile('mystery').example.prose));assert.ok(JSON.stringify(context(root,{examples:true})).includes(getProfile('mystery').example.prose));assert.throws(()=>context(root,{budget:500}),e=>e.code==='CONTEXT_BUDGET');}finally{cleanup(root);}
});
test('unsupported fact excerpts cannot enter a proposal',()=>{
  const root=workspace();try{assert.throws(()=>proposed(root,1,'Mara kept the letter.',[{subject:'Mara',predicate:'owns',value:'boat',kind:'world',known_by:[],quote:'Mara owns a boat.'}]),e=>e.code==='UNSUPPORTED_FACT');assert.equal(store.load(root).facts.length,0);}finally{cleanup(root);}
});
test('repeating a committed proposal is idempotent',()=>{
  const root=workspace();try{const p=accepted(root);const revision=store.load(root).revision;const receipt=store.commit(root,p.id,p.base_revision);assert.equal(receipt.idempotent,true);assert.equal(store.load(root).revision,revision);}finally{cleanup(root);}
});
test('a stale revision is rejected even when its target chapter is unchanged',()=>{
  const root=workspace();try{const p=proposed(root);const other=proposed(root,2,'A different chapter.');store.commit(root,other.id,other.base_revision);assert.throws(()=>store.commit(root,p.id,p.base_revision),e=>e.code==='STALE_REVISION');}finally{cleanup(root);}
});
test('an explicitly adopted manual edit preserves old text in history',()=>{
  const root=workspace();try{accepted(root);const target=path.join(root,'manuscript','chapter-0001.md');fs.writeFileSync(target,'The human made a different choice.');const p=call(root,['import','--adopt','--file',target,'--chapter','1']);call(root,['commit',p.id,'--expect',p.base_revision]);const state=store.load(root);store.undo(root,state.revision);assert.equal(fs.readFileSync(target,'utf8'),'Mara kept the letter.\n');}finally{cleanup(root);}
});
test('a crash between materialisation and HEAD update can be recovered',()=>{
  const root=workspace();const original=fs.renameSync;try{
    const p=proposed(root);let injected=false;
    fs.renameSync=function(a,b){if(String(b).endsWith('HEAD.json')&&!injected){injected=true;throw new Error('simulated interruption');}return original(a,b);};
    assert.throws(()=>store.commit(root,p.id,p.base_revision),/simulated/);fs.renameSync=original;
    assert.throws(()=>store.load(root),e=>e.code==='RECOVERY_REQUIRED');assert.equal(store.recover(root).recovered,true);assert.equal(store.load(root).chapters.length,1);assert.equal(store.recover(root).recovered,false);
  }finally{fs.renameSync=original;cleanup(root);}
});
test('recovery refuses to overwrite an intervening external edit',()=>{
  const root=workspace();const original=fs.renameSync;try{
    const p=proposed(root);fs.renameSync=function(a,b){if(String(b).endsWith('HEAD.json'))throw new Error('simulated interruption');return original(a,b);};
    assert.throws(()=>store.commit(root,p.id,p.base_revision));fs.renameSync=original;
    const target=path.join(root,'manuscript','chapter-0001.md');fs.writeFileSync(target,'Preserve this external edit.');assert.throws(()=>store.recover(root),e=>e.code==='RECOVERY_CONFLICT');assert.equal(fs.readFileSync(target,'utf8'),'Preserve this external edit.');
  }finally{fs.renameSync=original;cleanup(root);}
});
test('corrupt state fails loudly instead of becoming an empty project',()=>{
  const root=workspace();try{fs.writeFileSync(path.join(root,'.novelcraft','HEAD.json'),'{not json');const e=call(root,['status'],3);assert.equal(e.code,'CORRUPT_JSON');}finally{cleanup(root);}
});
test('path traversal and an untracked target are rejected',()=>{
  const root=workspace();try{assert.throws(()=>store.getProposal(root,'../../outside'),e=>e.code==='INVALID_ID');fs.mkdirSync(path.join(root,'manuscript'));fs.writeFileSync(path.join(root,'manuscript','chapter-0001.md'),'Untracked human text.');assert.throws(()=>proposed(root),e=>e.code==='UNTRACKED_MANUSCRIPT');}finally{cleanup(root);}
});
test('symlinked manuscript directories cannot redirect writes', {skip:process.platform==='win32'},()=>{
  const root=workspace(),outside=temp();try{fs.symlinkSync(outside,path.join(root,'manuscript'));assert.throws(()=>proposed(root),e=>e.code==='UNSAFE_PATH');assert.equal(fs.readdirSync(outside).length,0);}finally{cleanup(root);cleanup(outside);}
});
test('HTML escapes manuscript markup and EPUB is a readable ZIP container',()=>{
  const root=workspace();try{accepted(root,1,'<script>alert(1)</script>\n\nMara kept the letter.');const html=render(root,'html');assert.ok(!html.includes('<script>'));assert.ok(html.includes('&lt;script&gt;'));const epub=render(root,'epub');assert.equal(epub.readUInt32LE(0),0x04034b50);assert.equal(epub.subarray(30,38).toString(),'mimetype');assert.ok(epub.includes(Buffer.from('application/epub+zip')));assert.ok(epub.includes(Buffer.from('OEBPS/nav.xhtml')));}finally{cleanup(root);}
});
test('export and JSON output files are exclusive and stdout remains machine-readable',()=>{
  const root=workspace();try{accepted(root);const out=path.join(root,'book.md');const r=call(root,['export','--out',out]);assert.equal(r.format,'md');call(root,['export','--out',out],3);const meta=path.join(root,'status.json');const value=call(root,['status','--out',meta]);assert.equal(value.written,meta);assert.ok(JSON.parse(fs.readFileSync(meta)).revision);call(root,['status','--out',meta],3);}finally{cleanup(root);}
});
test('literal checks do not pretend to detect semantic facts or bad style',()=>{
  assert.equal(checkText('She was tired. The door was broken.').passed,true);assert.equal(checkText('Mara does not own the key.',{include:['Mara owns the key.']}).passed,false);
  const root=temp();try{const input=path.join(root,'input.md');fs.writeFileSync(input,'A brief paragraph.');const r=call(root,['check',input,'--must-include','missing'],5);assert.equal(r.findings[0].code,'MISSING_LITERAL');}finally{cleanup(root);}
});
test('preferences survive restart and appear in context',()=>{
  const root=workspace();try{call(root,['preferences','add','--instruction','Keep the quiet endings.','--expect',store.load(root).revision]);assert.equal(context(root).preferences[0].instruction,'Keep the quiet endings.');}finally{cleanup(root);}
});
test('the entry skill installer does not overwrite an edited skill',()=>{
  const root=temp();try{const target=path.join(root,'skills');const r=call(root,['setup','--target',target]);assert.ok(fs.existsSync(r.installed));call(root,['setup','--target',target]);fs.writeFileSync(r.installed,'My own instructions.');const e=call(root,['setup','--target',target],3);assert.equal(e.code,'SKILL_EXISTS');}finally{cleanup(root);}
});
test('revising an earlier chapter marks downstream continuity for review',()=>{
  const root=workspace();try{accepted(root,1);accepted(root,2,'Ivo waited at the quay.');const p=proposed(root,1,'Mara destroyed the letter.');store.commit(root,p.id,p.base_revision);assert.equal(store.load(root).chapters.find(c=>c.number===2).needs_review,true);}finally{cleanup(root);}
});
test('a previously accepted proposal cannot masquerade as current after undo',()=>{
  const root=workspace();try{const p=accepted(root);store.undo(root,store.load(root).revision);assert.throws(()=>store.commit(root,p.id,p.base_revision),e=>e.code==='PROPOSAL_SUPERSEDED');assert.equal(store.load(root).chapters.length,0);}finally{cleanup(root);}
});
test('tampered fact metadata is rejected before commit',()=>{
  const root=workspace();try{const p=proposed(root,1,'Mara kept the letter.',[{subject:'Mara',predicate:'has',value:'letter',kind:'world',known_by:[],quote:'Mara kept the letter.'}]);const f=path.join(root,'.novelcraft','proposals',`${p.id}.json`);const data=JSON.parse(fs.readFileSync(f,'utf8'));data.changes[0].facts[0].kind='infallible';fs.writeFileSync(f,JSON.stringify(data));assert.throws(()=>store.commit(root,p.id,p.base_revision),e=>e.code==='INVALID_PROPOSAL');assert.equal(store.load(root).chapters.length,0);}finally{cleanup(root);}
});
test('contradictory word bounds fail before any model call',()=>{
  const root=workspace();try{const e=call(root,['generate','--min-words','100','--max-words','50',...runner],2);assert.equal(e.code,'INVALID_INPUT');assert.equal(store.listRuns(root).length,0);}finally{cleanup(root);}
});
test('create honours unresolved review exit status',()=>{
  const root=temp();try{const r=call(root,['create',path.join(root,'story'),'--idea','A courier finds a letter.',...runner,'--runner-arg','major','--passes','0','--accept'],5);assert.equal(r.accepted,false);}finally{cleanup(root);}
});
test('unexpected positional arguments are rejected instead of ignored',()=>{
  const root=workspace();try{assert.equal(call(root,['status','surprise'],2).code,'INVALID_ARGUMENT');assert.equal(call(root,['canon','erase'],2).code,'INVALID_ARGUMENT');}finally{cleanup(root);}
});
test('diff shows the proposal base rather than a newer chapter',()=>{
  const root=workspace();try{accepted(root,1,'The original opening.');const pending=proposed(root,1,'A pending alternative.');const replacement=proposed(root,1,'The newer committed opening.');store.commit(root,replacement.id,replacement.base_revision);const diff=call(root,['diff',pending.id]);assert.equal(diff.stale,true);assert.equal(diff.changes[0].before,'The original opening.');assert.equal(diff.changes[0].after,'A pending alternative.');}finally{cleanup(root);}
});
test('project updates merge supplied fields and preserve earlier state',()=>{
  const root=workspace();try{accepted(root);const before=store.load(root);const r=call(root,['project','update','--voice','Dry close-third narration.','--expect',before.revision]);assert.equal(r.project.voice,'Dry close-third narration.');assert.equal(r.project.idea,before.project.idea);assert.equal(r.project.genres[0].id,'mystery');assert.equal(store.revision(root,before.revision).project.voice,before.project.voice);assert.equal(call(root,['project','show']).project.voice,r.project.voice);}finally{cleanup(root);}
});
test('rejecting a canon candidate is explicit and survives restart',()=>{
  const root=workspace();try{accepted(root,1,'Mara kept the letter.',[{subject:'Mara',predicate:'has',value:'letter',kind:'world',known_by:[],quote:'Mara kept the letter.'}]);const state=store.load(root);call(root,['canon','reject',state.facts[0].id,'--expect',state.revision]);assert.equal(call(root,['canon']).facts[0].status,'rejected');assert.equal(context(root,{chapter:2}).facts.length,0);}finally{cleanup(root);}
});
test('drafting context carries planned character motivations and voice',()=>{
  const root=workspace();try{const p=proposed(root);const proposalPath=path.join(root,'.novelcraft','proposals',`${p.id}.json`);const data=JSON.parse(fs.readFileSync(proposalPath,'utf8'));data.plan={title:'Test',premise:'Letters change a harbour.',voice:'Restrained.',world:'A harbour.',characters:[{name:'Mara',desire:'Protect her sister.',voice:'Avoids direct accusation.',knowledge:[]}],arcs:[],chapters:[{number:2,title:'A visit',pov:'Mara',intent:'Test an accusation.',change:'Trust shifts.',ending:'A departure.',threads:[]}]};fs.writeFileSync(proposalPath,JSON.stringify(data));store.commit(root,p.id,p.base_revision);assert.ok(context(root,{chapter:2}).cast.some(c=>c.name==='Mara'&&c.desire==='Protect her sister.'));}finally{cleanup(root);}
});
test('later generation retains the intentions of already planned chapters',()=>{
  const root=workspace();try{call(root,['generate',...runner,'--accept']);const original=store.load(root).plan.chapters[0];call(root,['generate',...runner,'--accept']);assert.deepEqual(store.load(root).plan.chapters.find(c=>c.number===1),original);assert.equal(context(root,{chapter:1,task:'review'}).target_plan.intent,original.intent);}finally{cleanup(root);}
});
test('a schema does not accept whitespace as a substantive review',async()=>{
  const {validate,SCHEMAS}=await import('../schemas.mjs');assert.throws(()=>validate({assessment:'  ',revise:false,findings:[]},SCHEMAS.review),e=>e.code==='INVALID_MODEL_OUTPUT');
});
test('a long serial can revise an early chapter without losing its accumulated plan',()=>{
  const root=workspace();try{const p=proposed(root);const f=path.join(root,'.novelcraft','proposals',`${p.id}.json`);const data=JSON.parse(fs.readFileSync(f,'utf8'));data.plan={title:'A serial',premise:'A courier changes a harbour.',voice:'Restrained.',world:'A harbour.',characters:[],arcs:[],chapters:Array.from({length:61},(_,i)=>({number:i+1,title:`Delivery ${i+1}`,pov:'Mara',intent:'A consequential encounter.',change:'A new interpretation.',ending:'A quiet question.',threads:[]}))};fs.writeFileSync(f,JSON.stringify(data));store.commit(root,p.id,p.base_revision);const r=call(root,['revise','--chapter','1','--instruction','Preserve the quiet tone.',...runner]);assert.equal(store.getProposal(root,r.proposal_id).plan.chapters.length,61);}finally{cleanup(root);}
});
