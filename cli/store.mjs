import fs from 'node:fs';
import path from 'node:path';
import {atomic, immutable, readJSON, readText, json, id, hash, fail, safeId, inside, integer, text} from './util.mjs';
import {validate,SCHEMAS} from './schemas.mjs';
const META = '.novelcraft';
const file = (root, suffix) => inside(root, `${META}/${suffix}`);
export const chapterId = n => `chapter-${String(integer(n,'chapter')).padStart(4,'0')}`;
export const chapterPath = n => `manuscript/${chapterId(n)}.md`;
export function findRoot(start = process.cwd()) {
  let root = path.resolve(start);
  while (!fs.existsSync(path.join(root,META,'HEAD.json'))) {
    const parent = path.dirname(root);
    if (parent === root) fail('NO_PROJECT','No Novel Craft project found. Run novel-craft init first.');
    root = parent;
  }
  return fs.realpathSync(root);
}
function lock(root, action) {
  const target=file(root,'LOCK'); let fd;
  try { fd=fs.openSync(target,'wx',0o600); }
  catch(e) { if(e.code==='EEXIST') fail('WORKSPACE_LOCKED','Another operation holds the workspace lock. Use doctor; unlock only after that process stops.',3); throw e; }
  try { fs.writeFileSync(fd,json({pid:process.pid,created_at:new Date().toISOString()})); return action(); }
  finally {fs.closeSync(fd);fs.unlinkSync(target);}
}
export function unlock(root) {
  const target=file(root,'LOCK');
  if(!fs.existsSync(target)) return {unlocked:false};
  const data=readJSON(target);integer(data.pid,'lock PID',1,2147483647);
  try { process.kill(data.pid,0); fail('PROCESS_RUNNING','The lock owner is still running. Stop it before unlocking.',3); }
  catch(e) {if(e.code!=='ESRCH') throw e;}
  fs.unlinkSync(target); return {unlocked:true};
}
function validateState(state) {
  if(state?.schema_version!==1 || !Array.isArray(state.chapters) || !Array.isArray(state.facts) || !Array.isArray(state.receipts) || !Array.isArray(state.preferences) || !state.project) fail('INVALID_STATE','Unsupported or malformed project state.',3);
  safeId(state.revision); if(state.parent!==null) safeId(state.parent);
  const seen=new Set();
  for(const ch of state.chapters) {
    if(ch.id!==chapterId(ch.number) || ch.path!==chapterPath(ch.number) || !/^[a-f0-9]{64}$/.test(ch.hash) || seen.has(ch.id)) fail('INVALID_STATE','Invalid or duplicate chapter record.',3);
    seen.add(ch.id);
  }
  return state;
}
export function revision(root, revisionId) {
  const state=validateState(readJSON(file(root,`revisions/${safeId(revisionId)}.json`)));
  if(state.revision!==revisionId) fail('INVALID_STATE','Revision identifier does not match its file.',3);
  return state;
}
export function load(root, {allowDirty=false,allowPending=false}={}) {
  if(!allowPending && fs.existsSync(file(root,'transaction.json'))) fail('RECOVERY_REQUIRED','An interrupted transaction exists. Run recover --yes before continuing.',3);
  const head=readJSON(file(root,'HEAD.json'));safeId(head.revision);
  const state=validateState(readJSON(file(root,`revisions/${head.revision}.json`)));
  if(state.revision!==head.revision) fail('INVALID_STATE','HEAD and revision disagree.',3);
  if(!allowDirty) {
    const changed=dirty(root,state);
    if(changed.length) fail('SOURCE_CHANGED','Manuscript files changed outside Novel Craft. Import the edited chapter with --adopt; no text was overwritten.',3,{changed});
  }
  return state;
}
export function dirty(root,state) {
  return state.chapters.filter(ch=>currentHash(root,ch.path)!==ch.hash).map(ch=>ch.path);
}
function currentHash(root,p) {
  const target=inside(root,p); return fs.existsSync(target) ? hash(readText(target)) : null;
}
export function sourceHashes(root,state) {return Object.fromEntries(state.chapters.map(ch=>[ch.path,currentHash(root,ch.path)]));}
export function checkSources(root,sources) {
  for(const [p,expected] of Object.entries(sources)) {
    if(!/^manuscript\/chapter-\d{4,5}\.md$/.test(p)) fail('INVALID_STATE','Invalid source manifest.',3);
    if(currentHash(root,p)!==expected) fail('SOURCE_CHANGED',`Source changed: ${p}. Prepare a new proposal.`,3);
  }
}
export function body(root,ch) {
  const result=readText(file(root,`objects/${ch.hash}.md`));
  if(hash(result)!==ch.hash) fail('CORRUPT_OBJECT',`Content object failed verification: ${ch.id}`,3);
  return result;
}
export function init(root,project) {
  fs.mkdirSync(root,{recursive:true});root=fs.realpathSync(root);
  if(fs.existsSync(inside(root,META))) fail('PROJECT_EXISTS','This project already has Novel Craft state; refusing to replace it.',3);
  if(fs.existsSync(path.join(root,'.novel'))) fail('LEGACY_PROJECT','Legacy .novel state found. Create a new workspace and import manuscripts; the original is preserved.',3);
  const stage=inside(root,`${META}-init-${id('stage')}`);
  fs.mkdirSync(stage);
  const state={schema_version:1,revision:id('rev'),parent:null,created_at:new Date().toISOString(),project,plan:null,chapters:[],facts:[],preferences:[],receipts:[]};
  for(const dir of ['revisions','objects','proposals','runs']) fs.mkdirSync(path.join(stage,dir));
  immutable(path.join(stage,'revisions',`${state.revision}.json`),json(state));
  atomic(path.join(stage,'HEAD.json'),json({revision:state.revision}));
  fs.renameSync(stage,inside(root,META));
  return {root,revision:state.revision,next_action:'generate or import'};
}
export function normaliseChange(change) {
  const number=integer(change.number,'chapter');
  text(change.prose,'prose',2000000);
  if(!Array.isArray(change.facts??[])) fail('INVALID_FACTS','facts must be an array.');
  const facts=(change.facts??[]).map(f=>{
    for(const key of ['subject','predicate','value','quote']) text(f[key],`fact.${key}`,20000);
    if(!['world','belief','reader','plan'].includes(f.kind)) fail('INVALID_FACT_KIND','Fact kind must be world, belief, reader or plan.');
    if(!Array.isArray(f.known_by) || f.known_by.some(x=>typeof x!=='string')) fail('INVALID_FACTS','known_by must be an array of names.');
    if(!change.prose.includes(f.quote)) fail('UNSUPPORTED_FACT','A canon candidate quotes text absent from the chapter.',2,{subject:f.subject});
    return {id:id('fact'),subject:f.subject,predicate:f.predicate,value:f.value,kind:f.kind,known_by:f.known_by,valid_from:number,valid_until:null,status:'candidate',source:{chapter:chapterId(number),hash:hash(change.prose),quote:f.quote}};
  });
  return {number,id:chapterId(number),path:chapterPath(number),title:text(change.title??`Chapter ${number}`,'title',1000),prose:change.prose,summary:change.summary??'',facts};
}
export function propose(root,changes,{plan,baseRevision,expectedSources,allowDirty=false,reason='Author proposal',reviews=[],proposalId}={}) {
  return lock(root,()=>{
    const state=load(root,{allowDirty});
    if(baseRevision && state.revision!==baseRevision) fail('STALE_REVISION','The project changed during this task.',3);
    if(expectedSources) checkSources(root,expectedSources);
    if(!Array.isArray(changes) || !changes.length) fail('EMPTY_PROPOSAL','A proposal needs at least one chapter.');
    if(proposalId && fs.existsSync(file(root,`proposals/${safeId(proposalId)}.json`))) { const previous=getProposal(root,proposalId); if(previous.base_revision!==state.revision || previous.reason!==reason) fail('PROPOSAL_CONFLICT','Proposal identifier already belongs to another operation.',3); return previous; }
    const normal=changes.map(normaliseChange);
    if(new Set(normal.map(x=>x.id)).size!==normal.length) fail('DUPLICATE_CHAPTER','A proposal repeats a chapter.');
    const sources=sourceHashes(root,state);
    for(const change of normal) if(!(change.path in sources)) {
      if(fs.existsSync(inside(root,change.path))) fail('UNTRACKED_MANUSCRIPT',`Refusing to replace untracked file: ${change.path}`,3);
      sources[change.path]=null;
    }
    const proposal={schema_version:1,id:proposalId??id('proposal'),base_revision:state.revision,sources,created_at:new Date().toISOString(),reason,changes:normal,reviews};
    if(plan!==undefined) proposal.plan=plan;
    immutable(file(root,`proposals/${proposal.id}.json`),json(proposal));
    return proposal;
  });
}
export function getProposal(root,proposalId) {
  const p=readJSON(file(root,`proposals/${safeId(proposalId)}.json`));
  if(p.schema_version!==1 || p.id!==proposalId || !Array.isArray(p.changes) || !p.changes.length || new Set(p.changes.map(c=>c.id)).size!==p.changes.length || !p.sources || typeof p.sources!=='object' || Array.isArray(p.sources)) fail('INVALID_PROPOSAL','Malformed proposal.',3);
  safeId(p.base_revision);
  if(p.plan!==undefined && p.plan!==null) {try{validate(p.plan,{...SCHEMAS.plan,properties:{...SCHEMAS.plan.properties,chapters:{...SCHEMAS.plan.properties.chapters,maxItems:10000},characters:{...SCHEMAS.plan.properties.characters,maxItems:10000}}});}catch{fail('INVALID_PROPOSAL','Proposal contains an invalid story plan.',3);}}
  for(const c of p.changes) {
    if(c.id!==chapterId(c.number) || c.path!==chapterPath(c.number)) fail('INVALID_PROPOSAL','Invalid proposal path.',3);
    text(c.prose,'prose',2000000);text(c.title,'title',1000);
    if(typeof c.summary!=='string') fail('INVALID_PROPOSAL','Invalid chapter summary.',3);
    if(!Array.isArray(c.facts)) fail('INVALID_PROPOSAL','Invalid facts.',3);
    for(const f of c.facts) {
      if(!['world','belief','reader','plan'].includes(f.kind) || f.status!=='candidate' || f.valid_from!==c.number || f.valid_until!==null || !Array.isArray(f.known_by) || f.known_by.some(x=>typeof x!=='string') || ['subject','predicate','value'].some(k=>typeof f[k]!=='string'||!f[k].trim())) fail('INVALID_PROPOSAL','Invalid fact metadata.',3);
      if(f.source?.hash!==hash(c.prose) || f.source.chapter!==c.id || !c.prose.includes(text(f.source.quote,'quote'))) fail('UNSUPPORTED_FACT','Fact source no longer matches its proposal.',3);
      safeId(f.id);
    }
  }
  return p;
}
function saveObject(root,contents) {const sha=hash(contents);immutable(file(root,`objects/${sha}.md`),contents);return sha;}
function transact(root,current,next,writes) {
  validateState(next);
  immutable(file(root,`revisions/${next.revision}.json`),json(next));
  const transaction={schema_version:1,base_revision:current.revision,next_revision:next.revision,writes};
  atomic(file(root,'transaction.json'),json(transaction));
  finishTransaction(root,transaction);
}
function finishTransaction(root,t) {
  if(t.schema_version!==1 || !Array.isArray(t.writes)) fail('INVALID_TRANSACTION','Invalid recovery journal.',3);
  safeId(t.base_revision);safeId(t.next_revision);
  const next=validateState(readJSON(file(root,`revisions/${t.next_revision}.json`)));
  if(next.revision!==t.next_revision || next.parent!==t.base_revision || new Set(t.writes.map(w=>w.path)).size!==t.writes.length) fail('INVALID_TRANSACTION','Invalid transaction ancestry or duplicate paths.',3);
  const head=readJSON(file(root,'HEAD.json'));
  if(![t.base_revision,t.next_revision].includes(head.revision)) fail('RECOVERY_CONFLICT','HEAD changed after this transaction.',3);
  for(const w of t.writes) {
    if(!/^manuscript\/chapter-\d{4,5}\.md$/.test(w.path)) fail('INVALID_TRANSACTION','Invalid journal path.',3);
    for(const sha of [w.before,w.after]) if(sha!==null && !/^[a-f0-9]{64}$/.test(sha)) fail('INVALID_TRANSACTION','Invalid object hash.',3);
    const actual=currentHash(root,w.path);
    if(actual!==w.before && actual!==w.after) fail('RECOVERY_CONFLICT',`File changed during transaction: ${w.path}`,3);
    if((next.chapters.find(ch=>ch.path===w.path)?.hash??null)!==w.after) fail('INVALID_TRANSACTION','Journal disagrees with next revision.',3);
  }
  // Validate the entire journal before writing any file. Re-running completes safely.
  for(const w of t.writes) {
    if(currentHash(root,w.path)===w.after) continue;
    const target=inside(root,w.path);
    if(w.after===null) {if(fs.existsSync(target)) fs.unlinkSync(target);}
    else {
      const contents=readText(file(root,`objects/${w.after}.md`));
      if(hash(contents)!==w.after) fail('CORRUPT_OBJECT','A transaction content object failed verification.',3);
      atomic(target,contents);
    }
  }
  atomic(file(root,'HEAD.json'),json({revision:t.next_revision}));
  fs.unlinkSync(file(root,'transaction.json'));
}
export function recover(root) {return lock(root,()=>{const target=file(root,'transaction.json');if(!fs.existsSync(target))return {recovered:false};const t=readJSON(target);finishTransaction(root,t);return {recovered:true,revision:t.next_revision};});}
export function commit(root,proposalId,expect,{acceptFacts=false,delegated=false}={}) {
  return lock(root,()=>{
    const state=load(root,{allowDirty:true});
    const previous=state.receipts.find(r=>r.proposal===proposalId);
    const p=getProposal(root,proposalId);
    if(previous) {
      if(dirty(root,state).length || p.changes.some(c=>state.chapters.find(ch=>ch.id===c.id)?.hash!==hash(c.prose))) fail('PROPOSAL_SUPERSEDED','This proposal was committed previously, but is no longer the current manuscript. Its historical receipt is preserved.',3);
      return {...previous,idempotent:true};
    }
    if(!expect || expect!==state.revision || p.base_revision!==state.revision) fail('STALE_REVISION','Commit requires the current --expect revision and a proposal based on it.',3);
    checkSources(root,p.sources);
    const next=structuredClone(state);next.revision=id('rev');next.parent=state.revision;
    if(p.plan) {
      // Keep earlier scene intentions and named cast when extending a serial.
      const chapters=new Map((state.plan?.chapters??[]).map(ch=>[ch.number,ch]));
      for(const ch of p.plan.chapters)chapters.set(ch.number,ch);
      const characters=new Map((state.plan?.characters??[]).map(c=>[c.name,c]));
      for(const c of p.plan.characters)characters.set(c.name,c);
      next.plan={...p.plan,chapters:[...chapters.values()].sort((a,b)=>a.number-b.number),characters:[...characters.values()]};
    }
    const writes=[];
    const revisedNumbers=p.changes.filter(c=>state.chapters.some(old=>old.id===c.id)).map(c=>c.number);
    if(revisedNumbers.length) next.chapters=next.chapters.map(ch=>ch.number>Math.min(...revisedNumbers)?{...ch,needs_review:true}:ch);
    for(const c of p.changes) {
      const actual=fs.existsSync(inside(root,c.path))?readText(inside(root,c.path)):null;
      if(actual!==null) saveObject(root,actual);
      const sha=saveObject(root,c.prose);
      const record={id:c.id,number:c.number,path:c.path,title:c.title,hash:sha,summary:c.summary,status:delegated?'delegated-draft':'accepted'};
      next.chapters=next.chapters.filter(ch=>ch.id!==c.id).concat(record).sort((a,b)=>a.number-b.number);
      next.facts=next.facts.map(f=>f.source.chapter===c.id?{...f,status:'superseded'}:f);
      next.facts.push(...c.facts.map(f=>({...f,status:acceptFacts?'accepted':'candidate'})));
      writes.push({path:c.path,before:actual===null?null:hash(actual),after:sha});
    }
    const receipt={id:id('receipt'),revision:next.revision,parent:state.revision,proposal:p.id,operation:'commit',chapters:p.changes.map(c=>c.id),facts_accepted:acceptFacts,approval:delegated?'delegated':'explicit',created_at:new Date().toISOString()};
    next.receipts.push(receipt);transact(root,state,next,writes);return receipt;
  });
}
export function acceptFact(root,factId,expect,decision='accepted') {
  if(!['accepted','rejected'].includes(decision)) fail('INVALID_DECISION','Fact decisions must be accepted or rejected.');
  return lock(root,()=>{
    const state=load(root);if(state.revision!==expect) fail('STALE_REVISION','Canon approval requires the current --expect revision.',3);
    const fact=state.facts.find(f=>f.id===factId);
    if(!fact || fact.status!=='candidate') fail('NOT_CANDIDATE','This fact is not a current candidate.');
    const ch=state.chapters.find(ch=>ch.id===fact.source.chapter);
    if(!ch || ch.hash!==fact.source.hash || !body(root,ch).includes(fact.source.quote)) fail('STALE_FACT','The supporting manuscript has changed.',3);
    const next=structuredClone(state);next.revision=id('rev');next.parent=state.revision;next.facts.find(f=>f.id===factId).status=decision;
    next.receipts.push({id:id('receipt'),operation:decision==='accepted'?'accept-fact':'reject-fact',fact:factId,revision:next.revision,created_at:new Date().toISOString()});
    transact(root,state,next,[]);return {revision:next.revision,fact:factId,status:decision};
  });
}
export function updateProject(root,patch,expect) {
  return lock(root,()=>{
    const state=load(root);if(state.revision!==expect)fail('STALE_REVISION','Project updates require the current --expect revision.',3);
    if(!Object.keys(patch).length)fail('EMPTY_UPDATE','Provide at least one project field to update.');
    for(const key of Object.keys(patch)) {
      if(!['title','idea','language','audience','voice','genres'].includes(key))fail('INVALID_INPUT',`Unknown project field: ${key}`);
      if(key!=='genres')text(patch[key],key,20000);
      else if(!Array.isArray(patch.genres)||!patch.genres.length)fail('INVALID_INPUT','A project needs at least one genre profile.');
    }
    const next=structuredClone(state);next.revision=id('rev');next.parent=state.revision;next.project={...next.project,...patch};
    if(['idea','voice','genres'].some(k=>k in patch))next.chapters=next.chapters.map(ch=>({...ch,needs_review:true}));
    next.receipts.push({id:id('receipt'),operation:'update-project',fields:Object.keys(patch),revision:next.revision,created_at:new Date().toISOString()});
    transact(root,state,next,[]);return {revision:next.revision,project:next.project};
  });
}
export function preference(root,value,expect) {
  return lock(root,()=>{const state=load(root);if(state.revision!==expect)fail('STALE_REVISION','Preferences require the current --expect revision.',3);text(value,'preference',5000);const next=structuredClone(state);next.revision=id('rev');next.parent=state.revision;next.preferences.push({id:id('preference'),instruction:value});transact(root,state,next,[]);return {revision:next.revision,preferences:next.preferences};});
}
export function undo(root,expect) {
  return lock(root,()=>{
    const current=load(root);if(current.revision!==expect)fail('STALE_REVISION','Undo requires the current --expect revision.',3);
    if(!current.parent)fail('NO_HISTORY','There is no earlier revision.');
    const target=validateState(readJSON(file(root,`revisions/${current.parent}.json`)));
    const next=structuredClone(target);next.revision=id('rev');next.parent=current.revision;next.receipts=[...current.receipts,{id:id('receipt'),operation:'undo',revision:next.revision,restored:target.revision,created_at:new Date().toISOString()}];
    const paths=new Set([...current.chapters,...target.chapters].map(c=>c.path));
    const writes=[...paths].map(p=>({path:p,before:currentHash(root,p),after:target.chapters.find(c=>c.path===p)?.hash??null}));
    transact(root,current,next,writes);return {revision:next.revision,restored:target.revision};
  });
}
export function status(root) {
  const state=load(root,{allowDirty:true,allowPending:true});
  return {revision:state.revision,title:state.project.title,chapters:state.chapters.map(({hash,...c})=>c),accepted_facts:state.facts.filter(f=>f.status==='accepted').length,candidate_facts:state.facts.filter(f=>f.status==='candidate').length,modified_files:dirty(root,state),recovery_required:fs.existsSync(file(root,'transaction.json')),next_action:state.chapters.length?'context, revise or generate':'generate or import'};
}
export function saveRun(root,run) {safeId(run.id);atomic(file(root,`runs/${run.id}.json`),json(run));}
export function readRun(root,runId) {return readJSON(file(root,`runs/${safeId(runId)}.json`));}
export function listRuns(root) {return fs.readdirSync(file(root,'runs')).filter(x=>x.endsWith('.json')).sort().map(name=>{const r=readJSON(file(root,`runs/${name}`));return {id:r.id,status:r.status,stage:r.stage,completed_chapters:r.chapters?.length??0,proposal:r.proposal??null,calls:r.calls??0};});}
