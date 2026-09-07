import {wordCount,fail} from './util.mjs';
import {load,dirty,body} from './store.mjs';
import {SCHEMAS,validate} from './schemas.mjs';
export function checkText(prose,{include=[],avoid=[],min,max}={}) {
  const words=wordCount(prose);const findings=[];
  for(const literal of include)if(!prose.includes(literal))findings.push({code:'MISSING_LITERAL',severity:'error',literal,method:'case-sensitive literal match; not semantic fact checking'});
  for(const literal of avoid)if(prose.includes(literal))findings.push({code:'FORBIDDEN_LITERAL',severity:'error',literal,method:'case-sensitive literal match; not semantic fact checking'});
  if(min!==undefined&&words<min)findings.push({code:'BELOW_MIN_WORDS',severity:'error',expected:min,actual:words});
  if(max!==undefined&&words>max)findings.push({code:'ABOVE_MAX_WORDS',severity:'error',expected:max,actual:words});
  return {passed:findings.length===0,words,findings,literary_quality:'not evaluated',scope:'Only explicit measurable constraints are checked. Passing does not prove coherence or quality.'};
}
export function validateChapter(value) {
  validate(value,SCHEMAS.chapter);
  for(const fact of value.facts)if(!value.prose.includes(fact.quote))fail('UNSUPPORTED_FACT','Model returned a fact with an excerpt absent from its prose.',4);
  return value;
}
export function validateReview(value,prose) {
  validate(value,SCHEMAS.review);
  for(const finding of value.findings)if(!prose.includes(finding.excerpt))fail('UNSUPPORTED_REVIEW','Reviewer cited an excerpt absent from the draft.',4);
  if(value.revise&&!value.findings.length)fail('UNSUPPORTED_REVIEW','A requested revision needs at least one evidenced finding.',4);
  return value;
}
export function audit(root) {
  const state=load(root,{allowDirty:true});const changed=dirty(root,state);const findings=[];
  for(const p of changed)findings.push({code:'SOURCE_CHANGED',severity:'error',path:p});
  for(const f of state.facts.filter(f=>f.status==='accepted')) {
    const ch=state.chapters.find(c=>c.id===f.source.chapter);
    if(!ch||ch.hash!==f.source.hash||!body(root,ch).includes(f.source.quote))findings.push({code:'STALE_FACT_SOURCE',severity:'error',fact:f.id});
  }
  const facts=state.facts.filter(f=>f.status==='accepted');
  for(let a=0;a<facts.length;a++)for(let b=a+1;b<facts.length;b++) {
    const x=facts[a],y=facts[b];
    if(x.subject===y.subject&&x.predicate===y.predicate&&x.kind===y.kind&&x.value!==y.value&&Math.max(x.valid_from,y.valid_from)<=Math.min(x.valid_until??Infinity,y.valid_until??Infinity))findings.push({code:'POTENTIALLY_COMPETING_FACTS',severity:'review',facts:[x,y],note:'Different values can be valid together, a state change or different beliefs. This is not an automatic contradiction verdict.'});
  }
  for(const ch of state.chapters.filter(c=>c.needs_review))findings.push({code:'DOWNSTREAM_REVIEW',severity:'review',chapter:ch.id});
  return {revision:state.revision,passed:!findings.some(f=>f.severity==='error'),findings,semantic_continuity:'Not proven by deterministic checks. Request a source-backed agent review.'};
}
