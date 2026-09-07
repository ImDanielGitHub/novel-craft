// Deterministic protocol fixture. It is not a real language model or a quality benchmark.
import fs from 'node:fs';
let raw='';for await(const chunk of process.stdin)raw+=chunk;
const {stage,input}=JSON.parse(raw);
const [mode,marker]=process.argv.slice(2);
if(mode==='invalid'){process.stdout.write('not JSON');process.exit(0);}
if(mode==='timeout'){setTimeout(()=>{},10000);await new Promise(()=>{});}
if(mode==='fail-review-once'&&stage==='review'&&!fs.existsSync(marker)){fs.writeFileSync(marker,'failed');process.exit(7);}
if(stage==='plan'){
  process.stdout.write(JSON.stringify({title:'The Undelivered Letter',premise:'A courier discovers a misplaced letter.',voice:'Restrained close third person.',world:'A rain-soaked harbour town.',characters:[{name:'Mara',desire:'Deliver the letter.',voice:'Observant and reserved.',knowledge:[]}],arcs:[{title:'The crossing',question:'Who changed the address?',payoff:'The choice to deliver it anyway.'}],chapters:input.chapter_numbers.map(number=>({number,title:`Delivery ${number}`,pov:'Mara',intent:'A small decision changes trust.',change:'A new obligation.',ending:'Quiet anticipation.',threads:['the unopened letter']}))}));
}else if(stage==='draft'||stage==='revise'){
  const n=input.chapter.number;
  if(n>1&&!input.context.chapters.some(c=>c.number===n-1))process.exit(8);
  const prose=n===1?'Mara handed the brass key to Ivo. He weighed it in his palm while she checked the address again. The rain had blurred the street name, but not the signature. She folded the letter along its old crease and waited.':'Ivo returned the brass key before the ferry left. Mara put it in her pocket without counting the other keys. Across the harbour, a light appeared in the window of the house she had been told was empty. She asked for a return ticket.';
  process.stdout.write(JSON.stringify({title:`Delivery ${n}`,prose,summary:`Chapter ${n}: the courier makes a consequential discovery.`,facts:[{subject:n===1?'Ivo':'Mara',predicate:'holds',value:'brass key',kind:'world',known_by:['Mara','Ivo'],quote:n===1?'Mara handed the brass key to Ivo.':'Ivo returned the brass key before the ferry left.'}]}));
}else if(stage==='review'){
  const prose=typeof input.draft==='string'?input.draft:input.draft.prose;
  process.stdout.write(JSON.stringify({assessment:'Protocol fixture only; not a real editorial judgement.',revise:mode==='major',findings:mode==='bad-quote'?[{excerpt:'This sentence does not exist.',concern:'Fixture.',intentional_reading:'Fixture.',suggestion:'Fixture.',priority:'major'}]:mode==='major'?[{excerpt:prose.split('. ')[0]+'.',concern:'Fixture unresolved finding.',intentional_reading:'Could be deliberate.',suggestion:'Inspect it.',priority:'major'}]:[]}));
}
