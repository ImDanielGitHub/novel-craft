#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import {parseArgs} from 'node:util';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {VERSION,CraftError,fail,readText,readJSON,json,integer,text,wordCount,inside} from './util.mjs';
import {PROFILES,SOURCES,CRAFT,getProfile,customProfile,withReferences,coverage} from './catalogue.mjs';
import * as store from './store.mjs';
import {context} from './context.mjs';
import {checkText,audit,validateReview} from './checks.mjs';
import {SCHEMAS} from './schemas.mjs';
import {generate} from './generate.mjs';
import {runModel} from './runner.mjs';
import {exportBook} from './export.mjs';
export const COMMANDS=['init','create','project','generate','import','context','review','revise','check','audit','diff','commit','canon','preferences','status','history','runs','undo','recover','unlock','export','genres','guide','schema','setup','doctor'];
const flags={};
for(const name of ['json','help','version','accept','accept-facts','yes','examples','adopt','packet'])flags[name]={type:'boolean'};
for(const name of ['project','title','idea','language','audience','voice','genre-file','chapter','chapters','words','min-words','max-words','budget','passes','max-calls','timeout-ms','runner','runner-command','model','resume','instruction','file','facts','summary','expect','pov','query','out','format','target'])flags[name]={type:'string'};
for(const name of ['genre','runner-arg','must-include','must-avoid'])flags[name]={type:'string',multiple:true};
const COMMAND_FLAGS={
 project:['title','idea','genre','genre-file','language','audience','voice','expect'],
 init:['title','idea','genre','genre-file','language','audience','voice'],create:['title','idea','genre','genre-file','language','audience','voice','chapters','words','min-words','max-words','budget','passes','max-calls','timeout-ms','runner','runner-command','runner-arg','model','accept','accept-facts','examples'],
 generate:['chapters','words','min-words','max-words','budget','passes','max-calls','timeout-ms','runner','runner-command','runner-arg','model','accept','accept-facts','examples','resume'],
 revise:['chapter','instruction','words','min-words','max-words','budget','passes','max-calls','timeout-ms','runner','runner-command','runner-arg','model','accept','accept-facts','examples'],
 import:['file','chapter','title','facts','summary','adopt'],context:['chapter','pov','query','budget','examples'],review:['chapter','budget','examples','packet','runner','runner-command','runner-arg','model','timeout-ms'],
 check:['file','must-include','must-avoid','min-words','max-words'],commit:['expect','accept-facts'],canon:['expect'],preferences:['expect','instruction'],undo:['expect','yes'],recover:['yes'],unlock:['yes'],export:['format'],setup:['target'],doctor:['runner'],genres:['examples'],schema:[],guide:[],diff:[],audit:[],status:[],history:[],runs:[]
};
const HELP=`Novel Craft ${VERSION}\n\nCreate and revise original webnovels with your existing writing agent. Node 22.14+; no platform binary or API key is bundled.\n\nStart from scratch:\n  novel-craft create ./my-book --idea "A translator discovers tomorrow's laws" --genre fantasy --genre mystery --chapters 3 --runner codex\n  novel-craft diff <proposal-id> --project ./my-book\n  novel-craft commit <proposal-id> --expect <revision> --project ./my-book\n\nExisting manuscript:\n  novel-craft init --title "My Book" --genre general-fiction\n  novel-craft import --file existing-chapter.md --chapter 1\n  novel-craft commit <proposal-id> --expect <revision>\n  novel-craft revise --chapter 1 --instruction "Strengthen the subtext; preserve restraint"\n\nDiscovery:\n  novel-craft genres [category | coverage] --examples\n  novel-craft guide\n  novel-craft schema [plan | chapter | review]\n  novel-craft setup --target .agents/skills\n\nCommands: ${COMMANDS.join(', ')}\n\nAll commands accept --json, --project and --out. Errors in JSON mode are one versioned JSON object on stdout.\nGeneration uses an authenticated Codex CLI by default. Other agents use --runner-command <executable> and repeated --runner-arg=<argument>. No shell is invoked.\nGeneration checkpoints every stage. Resume with generate --resume <run-id>. --max-calls caps model calls; --passes caps revision rounds.\n--accept delegates manuscript acceptance only if no major review finding or explicit constraint remains unresolved. --accept-facts separately approves candidate canon.\nThe CLI never guarantees literary quality. Read the prose and inspect changes.\n`;
function runnerOptions(v){return {runner:v.runner??'codex',command:v['runner-command'],args:v['runner-arg']??[],model:v.model,timeout:v['timeout-ms']};}
function generationOptions(v){return {chapters:v.chapters,words:v.words,min:v['min-words']===undefined?undefined:integer(v['min-words'],'min-words',1,100000),max:v['max-words']===undefined?undefined:integer(v['max-words'],'max-words',1,100000),budget:v.budget,passes:v.passes,maxCalls:v['max-calls'],accept:v.accept,acceptFacts:v['accept-facts'],examples:v.examples,resume:v.resume,runnerOptions:runnerOptions(v)};}
function projectOptions(v){const genres=(v.genre??(v['genre-file']?[]:['general-fiction'])).map(getProfile);if(v['genre-file'])genres.push(customProfile(readJSON(v['genre-file'])));return {title:v.title??'Untitled webnovel',idea:v.idea??'An existing manuscript; preserve its actual intent.',genres,language:v.language??'en-NZ',audience:v.audience??'general',voice:v.voice??'Choose a distinctive voice appropriate to the story; preserve it during revision.'};}
function requireYes(v,command){if(!v.yes)fail('APPROVAL_REQUIRED',`${command} requires --yes. Inspect status/history first.`);}
function outputFile(target,value){const absolute=path.resolve(target);if(fs.existsSync(absolute))fail('OUTPUT_EXISTS','Output file exists. Choose another filename.',3);fs.mkdirSync(path.dirname(absolute),{recursive:true});const fd=fs.openSync(absolute,'wx',0o600);try{fs.writeFileSync(fd,typeof value==='string'?value:json(value));fs.fsyncSync(fd);}finally{fs.closeSync(fd);}return {written:absolute};}
async function execute(command,args,v){
  const location=path.resolve(v.project??process.cwd());
  if(command==='help')return HELP;
  if(command==='schema')return args[0]?SCHEMAS[args[0]]??fail('UNKNOWN_SCHEMA','Use schema plan, chapter or review.'):{commands:COMMANDS,protocol:'novel-craft/1',output:{schema_version:1,version:VERSION,ok:'boolean',command:'string',data:'result on success',error:'{code,message,details} on failure'},exit_codes:{0:'success',2:'invalid input',3:'workspace conflict or corrupt state',4:'model/runner failure',5:'explicit check failure or unresolved delegated review'},command_flags:COMMAND_FLAGS};
  if(command==='genres')return args[0]==='coverage'?coverage():args[0]&&args[0]!=='list'?withReferences(getProfile(args.join(' ')),{examples:Boolean(v.examples)}):{profiles:PROFILES.map(({id,name,kind})=>({id,name,kind})),coverage:coverage()};
  if(command==='guide')return {...CRAFT,references:SOURCES};
  if(command==='setup'){
    const target=path.resolve(v.target??path.join(location,'.agents','skills'),'novel-craft','SKILL.md');
    const skill=readText(fileURLToPath(new URL('./SKILL.md',import.meta.url)));
    if(fs.existsSync(target)){if(readText(target)!==skill)fail('SKILL_EXISTS','An existing skill differs; it was not overwritten. Choose a new target and reconcile explicitly.',3);return {installed:target,unchanged:true};}
    return {installed:outputFile(target,skill).written};
  }
  if(command==='doctor'){
    const node=process.versions.node.split('.').map(Number);const supported=node[0]>22||(node[0]===22&&node[1]>=14);
    let workspace=null;try{workspace=store.status(store.findRoot(location));}catch(e){if(e.code!=='NO_PROJECT')workspace={error:e.code??'ERROR',message:e.message};}
    const probe=spawnSync('codex',['--version'],{encoding:'utf8',timeout:5000,windowsHide:true,shell:false});
    return {version:VERSION,node:process.version,node_supported:supported,runtime:'dependency-free Node',workspace,codex_available:probe.status===0,generation_requires:'Authenticated Codex CLI or an explicitly supplied JSON runner. Availability does not prove authentication or prose quality.'};
  }
  if(command==='init'||command==='create'){
    const root=command==='create'&&args[0]?path.resolve(args[0]):location;
    if(command==='create')text(v.idea,'--idea');
    const initial=store.init(root,projectOptions(v));
    if(command==='init')return initial;
    const data=await generate(initial.root,generationOptions(v));if(v.accept&&!data.accepted)process.exitCode=5;
    return {...data,project:initial.root};
  }
  if(command==='check'){
    const input=v.file??args[0];if(!input)fail('INPUT_REQUIRED','check requires --file or a manuscript path.');
    const min=v['min-words']===undefined?undefined:integer(v['min-words'],'min-words');const max=v['max-words']===undefined?undefined:integer(v['max-words'],'max-words',1,1000000);
    if(min!==undefined&&max!==undefined&&min>max)fail('INVALID_INPUT','min-words cannot exceed max-words.');
    const result=checkText(readText(input),{include:v['must-include']??[],avoid:v['must-avoid']??[],min,max});if(!result.passed)process.exitCode=5;return result;
  }
  const root=store.findRoot(location);
  if(command==='project'){
    const state=store.load(root);if(args[0]!=='update')return {revision:state.revision,project:state.project};
    const patch=Object.fromEntries(['title','idea','language','audience','voice'].filter(k=>v[k]!==undefined).map(k=>[k,v[k]]));
    if(v.genre||v['genre-file'])patch.genres=projectOptions(v).genres;
    return store.updateProject(root,patch,v.expect);
  }
  if(command==='status')return store.status(root);
  if(command==='history')return {revision:store.load(root).revision,receipts:store.load(root).receipts};
  if(command==='runs')return store.listRuns(root);
  if(command==='audit'){const data=audit(root);if(!data.passed)process.exitCode=5;return data;}
  if(command==='context')return context(root,{chapter:v.chapter,pov:v.pov,query:v.query,budget:v.budget,examples:v.examples});
  if(command==='import'){
    if(!v.file)fail('INPUT_REQUIRED','import requires --file. The source is never modified.');
    const state=store.load(root,{allowDirty:Boolean(v.adopt)});const prose=readText(v.file);
    const number=integer(v.chapter??Math.max(0,...state.chapters.map(c=>c.number))+1,'chapter');
    const proposal=store.propose(root,[{number,title:v.title??prose.match(/^#\s+(.+)$/m)?.[1]??`Chapter ${number}`,prose,summary:v.summary??'',facts:v.facts?readJSON(v.facts):[]}],{allowDirty:Boolean(v.adopt),reason:v.adopt?'Explicitly adopt external manuscript edit':'Import manuscript'});
    return {id:proposal.id,base_revision:proposal.base_revision,chapters:proposal.changes.map(c=>({number:c.number,title:c.title,words:wordCount(c.prose)})),next_action:`diff ${proposal.id}`};
  }
  if(command==='diff'){
    const p=store.getProposal(root,args[0]);const state=store.load(root,{allowDirty:true});
    const base=store.revision(root,p.base_revision);
    return {id:p.id,base_revision:p.base_revision,current_revision:state.revision,stale:p.base_revision!==state.revision,reason:p.reason,plan:p.plan??null,reviews:p.reviews,changes:p.changes.map(c=>{const previous=base.chapters.find(ch=>ch.id===c.id);return {chapter:c.id,title:c.title,before:previous?store.body(root,previous):'',after:c.prose,candidate_facts:c.facts};})};
  }
  if(command==='commit')return store.commit(root,args[0],v.expect,{acceptFacts:Boolean(v['accept-facts'])});
  if(command==='canon')return ['accept','reject'].includes(args[0])?store.acceptFact(root,args[1],v.expect,args[0]==='accept'?'accepted':'rejected'):{revision:store.load(root).revision,facts:store.load(root).facts};
  if(command==='preferences')return args[0]==='add'?store.preference(root,v.instruction,v.expect):{revision:store.load(root).revision,preferences:store.load(root).preferences};
  if(command==='undo'){requireYes(v,'undo');return store.undo(root,v.expect);}
  if(command==='recover'){requireYes(v,'recover');return store.recover(root);}
  if(command==='unlock'){requireYes(v,'unlock');return store.unlock(root);}
  if(command==='export')return exportBook(root,v.out,v.format??'md');
  if(command==='generate'||command==='revise'){
    if(command==='revise')text(v.instruction,'--instruction');
    const options={...generationOptions(v),revise:command==='revise',chapter:v.chapter,instruction:v.instruction};
    const data=await generate(root,options);if(v.accept&&!data.accepted)process.exitCode=5;return data;
  }
  if(command==='review'){
    const ctx=context(root,{chapter:integer(v.chapter,'chapter'),task:'review',budget:v.budget,examples:v.examples});
    const draft=ctx.chapters.find(c=>c.role==='target').prose;
    const input={instructions:CRAFT.review_contract,context:ctx,draft};
    if(v.packet)return {mode:'review-packet',executed:false,input,schema:SCHEMAS.review};
    return {mode:'agent-review',executed:true,review:validateReview(await runModel('review',input,runnerOptions(v)),draft)};
  }
  fail('UNKNOWN_COMMAND',`Unknown command '${command}'. Use --help.`);
}
async function main(){
  let values={json:process.argv.includes('--json')};let command='';
  try {
    const parsed=parseArgs({args:process.argv.slice(2),options:flags,allowPositionals:true,strict:true});values=parsed.values;
    command=parsed.positionals[0]??'help';const args=parsed.positionals.slice(1);
    if(values.version){const result={schema_version:1,version:VERSION,ok:true,command:'version',data:{version:VERSION}};process.stdout.write(values.json?json(result):`${VERSION}\n`);return;}
    if(values.help)command='help';
    if(command!=='help'&&!COMMANDS.includes(command))fail('UNKNOWN_COMMAND',`Unknown command '${command}'. Use --help.`);
    for(const name of Object.keys(values))if(!['json','project','out','help','version'].includes(name)&&!(COMMAND_FLAGS[command]??[]).includes(name))fail('INVALID_FLAG',`--${name} does not apply to ${command}.`);
    const positionalLimits={help:0,schema:1,genres:4,create:1,check:1,diff:1,commit:1,canon:2,preferences:1,project:1};
    if(command!=='help'&&args.length>(positionalLimits[command]??0))fail('INVALID_ARGUMENT',`Unexpected positional argument for ${command}.`);
    if(command==='canon' && args.length && !['list','accept','reject'].includes(args[0]))fail('INVALID_ARGUMENT','Use canon list, canon accept <fact-id> or canon reject <fact-id>.');
    if(command==='project' && args.length && !['show','update'].includes(args[0]))fail('INVALID_ARGUMENT','Use project show or project update.');
    if(command==='preferences' && args.length && !['list','add'].includes(args[0]))fail('INVALID_ARGUMENT','Use preferences list or preferences add.');
    if(command==='canon' && args[0]==='list' && args.length>1)fail('INVALID_ARGUMENT','canon list takes no further arguments.');
    let data=await execute(command,args,values);
    if(values.out&&command!=='export')data=outputFile(values.out,data);
    const envelope={schema_version:1,version:VERSION,ok:true,command,data};
    process.stdout.write(values.json?json(envelope):typeof data==='string'?data:json(data));
  } catch(e) {
    const error=e instanceof CraftError?e:new CraftError(e.code??'ERROR',e.message,2);
    const envelope={schema_version:1,version:VERSION,ok:false,command,error:{code:error.code,message:error.message,details:error.details}};
    process.exitCode=error.exit;
    if(values.json)process.stdout.write(json(envelope));else process.stderr.write(`${error.code}: ${error.message}\n${Object.keys(error.details??{}).length?json(error.details):''}`);
  }
}
await main();
