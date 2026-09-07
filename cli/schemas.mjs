import {fail} from './util.mjs';
const str={type:'string',minLength:1,maxLength:10000};
const strings={type:'array',items:str,maxItems:100};
const object=properties=>({type:'object',properties,required:Object.keys(properties),additionalProperties:false});
export const FACT=object({subject:str,predicate:str,value:str,kind:{type:'string',enum:['world','belief','reader','plan']},known_by:strings,quote:{type:'string',minLength:1,maxLength:20000}});
export const SCHEMAS={
  plan:object({title:str,premise:str,voice:str,world:str,characters:{type:'array',maxItems:60,items:object({name:str,desire:str,voice:str,knowledge:strings})},arcs:{type:'array',maxItems:50,items:object({title:str,question:str,payoff:str})},chapters:{type:'array',minItems:1,maxItems:50,items:object({number:{type:'integer',minimum:1,maximum:10000},title:str,pov:str,intent:str,change:str,ending:str,threads:strings})}}),
  chapter:object({title:str,prose:{type:'string',minLength:1,maxLength:2000000},summary:{type:'string',maxLength:6000},facts:{type:'array',maxItems:60,items:FACT}}),
  review:object({assessment:str,revise:{type:'boolean'},findings:{type:'array',maxItems:12,items:object({excerpt:{type:'string',minLength:1,maxLength:20000},concern:str,intentional_reading:str,suggestion:str,priority:{type:'string',enum:['major','minor','question']}})}})
};
export function validate(value,schema,at='$') {
  const valid=schema.type==='object'?value!==null&&typeof value==='object'&&!Array.isArray(value):schema.type==='array'?Array.isArray(value):schema.type==='integer'?Number.isSafeInteger(value):typeof value===schema.type;
  if(!valid)fail('INVALID_MODEL_OUTPUT',`${at} must be ${schema.type}.`,4);
  if(schema.type==='string'&&schema.minLength&&value.trim().length<schema.minLength)fail('INVALID_MODEL_OUTPUT',`${at} must contain substantive text.`,4);
  if(schema.enum&&!schema.enum.includes(value))fail('INVALID_MODEL_OUTPUT',`${at} has an unsupported value.`,4);
  if(schema.type==='string'&&((schema.minLength!==undefined&&value.length<schema.minLength)||(schema.maxLength!==undefined&&value.length>schema.maxLength)))fail('INVALID_MODEL_OUTPUT',`${at} has invalid length.`,4);
  if(schema.type==='integer'&&(value<schema.minimum||value>schema.maximum))fail('INVALID_MODEL_OUTPUT',`${at} is outside bounds.`,4);
  if(schema.type==='array'){
    if(value.length<(schema.minItems??0)||value.length>(schema.maxItems??100000))fail('INVALID_MODEL_OUTPUT',`${at} has invalid item count.`,4);
    value.forEach((item,i)=>validate(item,schema.items,`${at}[${i}]`));
  }
  if(schema.type==='object'){
    for(const key of schema.required??[])if(!(key in value))fail('INVALID_MODEL_OUTPUT',`${at}.${key} is required.`,4);
    for(const key of Object.keys(value)){
      if(!schema.properties[key]){if(schema.additionalProperties===false)fail('INVALID_MODEL_OUTPUT',`${at}.${key} is not allowed.`,4);}
      else validate(value[key],schema.properties[key],`${at}.${key}`);
    }
  }
  return value;
}
