import {load,sourceHashes,checkSources,saveRun,readRun,propose,commit,chapterId} from './store.mjs';
import {context} from './context.mjs';
import {CRAFT} from './catalogue.mjs';
import {SCHEMAS,validate} from './schemas.mjs';
import {validateChapter,validateReview,checkText} from './checks.mjs';
import {runModel} from './runner.mjs';
import {id,integer,fail,wordCount} from './util.mjs';
function result(run) {
  return {run_id:run.id,status:run.status,proposal_id:run.proposal_id,base_revision:run.base_revision,calls:run.calls,chapters:run.chapters.map(c=>({number:c.number,title:c.title,words:wordCount(c.prose)})),reviews:run.reviews,accepted:Boolean(run.receipt),receipt:run.receipt??null,quality:'Model review is evidence-backed advice, not proof of literary quality.',next_action:run.receipt?'export or continue':`diff ${run.proposal_id}, then commit ${run.proposal_id} --expect ${run.base_revision}`};
}
export async function generate(root,options={},runner=runModel) {
  if(options.min!==undefined&&options.max!==undefined&&options.min>options.max)fail('INVALID_INPUT','min-words cannot exceed max-words.');
  let run;
  if(options.resume)run=readRun(root,options.resume);
  else {
    const state=load(root);const type=options.revise?'revise':'generate';
    const start=options.revise?integer(options.chapter,'chapter'):Math.max(0,...state.chapters.map(c=>c.number))+1;
    const count=type==='revise'?1:integer(options.chapters??1,'chapters',1,50);
    if(start+count-1>10000)fail('CHAPTER_LIMIT','This run would exceed the supported chapter number range.');
    if(type==='revise'&&!state.chapters.some(c=>c.number===start))fail('NO_CHAPTER','The requested revision target does not exist.');
    run={schema_version:1,id:id('run'),type,base_revision:state.revision,sources:sourceHashes(root,state),created_at:new Date().toISOString(),options:{start,count,words:integer(options.words??1800,'words',50,12000),passes:integer(options.passes??1,'passes',0,3),max_calls:integer(options.maxCalls??(1+count*4),'max-calls',1,1000),budget:integer(options.budget??12000,'budget',2500,200000),min:options.min,max:options.max,examples:Boolean(options.examples)},instruction:options.instruction??'',stage:type==='revise'?'revise':'plan',plan:state.plan,chapters:[],current:null,current_review:null,review_pass:0,reviews:[],calls:0,attempts:[],status:'running',proposal_id:id('proposal')};
    saveRun(root,run);
  }
  if(run.schema_version!==1||!Array.isArray(run.chapters)||!run.options)fail('INVALID_RUN','Malformed or unsupported generation checkpoint.',3);
  if(options.maxCalls!==undefined)run.options.max_calls=integer(options.maxCalls,'max-calls',run.calls,1000);
  if(run.status==='complete')return result(run);
  try {
    const state=load(root);
    if(state.revision!==run.base_revision)fail('STALE_REVISION','The project changed since this run began. Keep its drafts, but start a fresh run against current state.',3);
    checkSources(root,run.sources);run.status='running';saveRun(root,run);
    const call=async(stage,input)=>{
      if(run.calls>=run.options.max_calls)fail('CALL_BUDGET','The run reached its model-call cap. Resume with an explicitly larger --max-calls to continue.',4);
      run.calls++;saveRun(root,run);
      const output=await runner(stage,input,options.runnerOptions??{});
      run.attempts.push({stage,chapter:run.options.start+run.chapters.length,output});saveRun(root,run);return output;
    };
    while(run.stage!=='finish') {
      const number=run.options.start+run.chapters.length;
      const ctx=()=>context(root,{chapter:number,task:run.type==='revise'?'revise':'draft',budget:run.options.budget,examples:run.options.examples,drafts:run.chapters,plan:run.plan});
      if(run.stage==='plan') {
        const output=await call('plan',{instructions:'Create a distinctive, editable story plan for exactly the requested chapters. Respect a settled premise. Plan local chapter experiences and wider arcs; vary openings and endings. Do not require combat, a wound, power cost or cliffhanger in every genre. Reading references are technique pointers, never imitation requests.',context:ctx(),chapter_numbers:Array.from({length:run.options.count},(_,i)=>run.options.start+i),target_words:run.options.words});
        validate(output,SCHEMAS.plan);
        const expected=Array.from({length:run.options.count},(_,i)=>run.options.start+i);
        if(output.chapters.length!==expected.length||output.chapters.some((c,i)=>c.number!==expected[i]))fail('INVALID_PLAN','The plan must contain each requested chapter number once, in order.',4);
        run.plan=output;run.stage='draft';saveRun(root,run);continue;
      }
      if(run.stage==='draft'||run.stage==='revise') {
        const stage=run.stage;
        const output=await call(stage,{instructions:stage==='draft'?'Write finished original prose, not an outline. Match the requested viewpoint and voice. Use summary where it serves pace and dramatise the moments the reader should experience. Let the chapter have its own satisfaction. Extract only durable candidate facts with exact supporting quotes; do not confuse belief with world truth.':'Revise only where the writer\'s request or evidenced review justifies it. Preserve effective language, intended ambiguity and voice. Do not reseed the story. Return the full revised chapter and newly supported candidate facts.',context:ctx(),chapter:run.plan?.chapters?.find(c=>c.number===number)??{number},writer_instruction:run.instruction,target_words:run.options.words,draft:run.current,review:run.current_review,measurable_constraints:{min_words:run.options.min??null,max_words:run.options.max??null}});
        validateChapter(output);run.current={...output,number};
        if(stage==='revise')run.review_pass++;
        run.stage='review';saveRun(root,run);continue;
      }
      if(run.stage==='review') {
        const measured=checkText(run.current.prose,{min:run.options.min,max:run.options.max});
        const review=validateReview(await call('review',{instructions:`Read the actual chapter against its intention and preceding context. ${CRAFT.review_contract} Examine continuity, causal or emotional movement, viewpoint, subtext, pacing and genre fit. Telling, quiet endings and stylistic density can be intentional. Recommend revision only for concrete worthwhile improvements; minor preferences are not blockers.`,context:ctx(),draft:run.current,checks:measured}),run.current.prose);
        run.current_review=review;
        const mustRevise=!measured.passed||(review.revise&&review.findings.some(f=>f.priority==='major'));
        if(mustRevise&&run.review_pass<run.options.passes)run.stage='revise';
        else {
          run.reviews.push({chapter:number,...review,checks:measured,unresolved:mustRevise});
          run.chapters.push(run.current);run.current=null;run.current_review=null;run.review_pass=0;
          run.stage=run.chapters.length===run.options.count?'finish':'draft';
        }
        saveRun(root,run);continue;
      }
      fail('INVALID_RUN','Unrecognised checkpoint stage.',3);
    }
    // A whole generation run becomes one atomic, inspectable proposal.
    const proposal=propose(root,run.chapters,{plan:run.plan,baseRevision:run.base_revision,expectedSources:run.sources,reason:`${run.type} run ${run.id}`,reviews:run.reviews,proposalId:run.proposal_id});
    run.proposal_id=proposal.id;run.status=run.reviews.some(r=>r.unresolved)?'needs-review':'proposed';saveRun(root,run);
    if(options.accept&&run.status==='proposed') {
      run.receipt=commit(root,run.proposal_id,run.base_revision,{acceptFacts:Boolean(options.acceptFacts),delegated:true});run.status='complete';saveRun(root,run);
    }
    return result(run);
  } catch(e) {
    run.status='failed';run.error={code:e.code??'ERROR',message:e.message};saveRun(root,run);
    e.details={...(e.details??{}),run_id:run.id,resume:`novel-craft generate --resume ${run.id}`};throw e;
  }
}
