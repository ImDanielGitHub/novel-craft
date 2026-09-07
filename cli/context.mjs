import {load,body,chapterId} from './store.mjs';
import {CRAFT,withReferences} from './catalogue.mjs';
import {estimateTokens,integer,fail} from './util.mjs';
export function context(root,{chapter,task='draft',pov='',query='',budget=12000,examples=false,drafts=[],plan}={}) {
  budget=integer(budget,'budget',500,200000);
  if(!['plan','draft','revise','review'].includes(task))fail('INVALID_TASK','Context task must be plan, draft, revise or review.');
  const state=load(root);
  chapter=integer(chapter??Math.max(0,...state.chapters.map(c=>c.number))+1,'chapter');
  const activePlan=plan??state.plan;
  const target=activePlan?.chapters?.find(c=>c.number===chapter)??null;
  const prior=state.chapters.filter(c=>c.number<chapter);
  const generated=drafts.filter(c=>c.number<chapter);
  const recentNumber=Math.max(0,...prior.map(c=>c.number),...generated.map(c=>c.number));
  const current=state.chapters.find(c=>c.number===chapter);
  const packet={schema_version:1,revision:state.revision,task,chapter,pov:pov||null,project:{...state.project,genres:state.project.genres.map(p=>({id:p.id,name:p.name,kind:p.kind}))},craft:CRAFT.principles,profiles:state.project.genres.map(p=>withReferences(p,{examples})),preferences:state.preferences,target_plan:target,arc_context:activePlan?{premise:activePlan.premise,voice:activePlan.voice,world:activePlan.world,arcs:activePlan.arcs}:null,facts:[],chapters:[],cast:[],warnings:[],coverage:{cast_total:activePlan?.characters?.length??0,cast_included:0,omitted_cast:0,facts_total:0,facts_included:0,chapters_total:prior.length+generated.length+(current?1:0),chapters_included:0,omitted_facts:0,omitted_chapters:0},budget:{method:'UTF-8 bytes / 4 estimate; not a model tokenizer',limit:budget,estimated_tokens:0}};
  if(task==='revise'||task==='review') {
    if(!current)fail('NO_CHAPTER',`Chapter ${chapter} is not in the manuscript.`);
    packet.chapters.push({...current,prose:body(root,current),role:'target'});
  }
  const lastDraft=generated.find(c=>c.number===recentNumber);
  const lastStored=prior.find(c=>c.number===recentNumber);
  if(lastDraft)packet.chapters.push({number:lastDraft.number,title:lastDraft.title,prose:lastDraft.prose,summary:lastDraft.summary,role:'unaccepted-prior-draft'});
  else if(lastStored)packet.chapters.push({...lastStored,prose:body(root,lastStored),role:'previous'});
  if(generated.length)packet.warnings.push('Prior generated chapters in this packet have not yet been accepted. Their summaries are interpretations, not approved world facts.');
  if(pov)packet.warnings.push('POV filtering applies to structured facts. Read included prose with the same knowledge boundary; the tool cannot prove that narration contains no spoilers.');
  if(estimateTokens(packet)>budget)fail('CONTEXT_BUDGET','Essential instructions and target/recent prose exceed the requested context budget. Increase --budget; nothing was silently truncated.',2,{minimum_estimate:estimateTokens(packet)});
  const append=(key,item)=>{packet[key].push(item);if(estimateTokens(packet)+100>budget){packet[key].pop();return false;}return true;};
  const terms=new Set(`${query} ${target?.pov??''} ${target?.intent??''}`.toLowerCase().match(/[\p{L}\p{N}]+/gu)??[]);
  const rank=f=>[...terms].filter(t=>`${f.subject} ${f.value}`.toLowerCase().includes(t)).length*100+f.valid_from;
  const available=state.facts.filter(f=>f.status==='accepted'&&f.valid_from<=chapter&&(f.valid_until===null||f.valid_until>=chapter));
  packet.coverage.facts_total=available.length;
  for(const fact of available.sort((a,b)=>rank(b)-rank(a)||a.id.localeCompare(b.id))) {
    const source=state.chapters.find(c=>c.id===fact.source.chapter);
    if(!source||source.hash!==fact.source.hash){packet.warnings.push(`Stale fact excluded: ${fact.id}`);continue;}
    if(pov&&(fact.kind==='plan'||!fact.known_by.includes(pov)))continue;
    append('facts',fact);
  }
  const cast=[...(activePlan?.characters??[])].sort((a,b)=>(b.name===(pov||target?.pov)?1:0)-(a.name===(pov||target?.pov)?1:0));
  for(const character of cast)append('cast',{...character,status:'author-plan, not approved canon'});
  packet.coverage.cast_included=packet.cast.length;packet.coverage.omitted_cast=cast.length-packet.cast.length;
  const summaries=[...prior.filter(c=>c.number!==recentNumber).map(c=>({id:c.id,number:c.number,title:c.title,summary:c.summary,hash:c.hash,role:'derived-summary',needs_review:c.needs_review??false})),...generated.filter(c=>c.number!==recentNumber).map(c=>({number:c.number,title:c.title,summary:c.summary,role:'unaccepted-summary'}))].sort((a,b)=>b.number-a.number);
  for(const summary of summaries)append('chapters',summary);
  if(prior.some(c=>c.needs_review))packet.warnings.push('Earlier manuscript revisions may affect later chapters. Review downstream continuity before accepting more prose.');
  packet.coverage.facts_included=packet.facts.length;packet.coverage.chapters_included=packet.chapters.length;
  packet.coverage.omitted_facts=packet.coverage.facts_total-packet.facts.length;packet.coverage.omitted_chapters=packet.coverage.chapters_total-packet.chapters.length;
  if(packet.coverage.omitted_facts||packet.coverage.omitted_chapters||packet.coverage.omitted_cast)packet.warnings.push('Coverage is partial due to scope or budget. Request focused context for an omitted subject before relying on its absence.');
  packet.budget.estimated_tokens=estimateTokens(packet);
  if(packet.budget.estimated_tokens>budget)fail('CONTEXT_BUDGET','Context metadata exceeds budget. Increase --budget.');
  return packet;
}
