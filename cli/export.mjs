import fs from 'node:fs';
import path from 'node:path';
import {load,body} from './store.mjs';
import {fail,atomic} from './util.mjs';
const escape=s=>String(s).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;').replaceAll('"','&quot;').replaceAll("'",'&#39;');
function proseHTML(prose) {
  return prose.split(/\r?\n\s*\r?\n/).map(p=>{
    const clean=p.trim();if(!clean)return '';
    if(/^#{1,6}\s/.test(clean)){const level=Math.min(6,(clean.match(/^#+/)??['##'])[0].length);return `<h${level}>${escape(clean.replace(/^#+\s/,''))}</h${level}>`;}
    if(/^\*\*\*$|^---$/.test(clean))return '<hr />';
    return `<p>${escape(clean).replaceAll('\n','<br />')}</p>`;
  }).join('\n');
}
const STYLE='body{font-family:Georgia,serif;line-height:1.75;max-width:44rem;margin:3rem auto;padding:0 1.2rem;color:#242424;background:#fffdf9}h1,h2{line-height:1.25}a{color:inherit}nav{font-family:system-ui,sans-serif;font-size:.95rem}section{margin:4rem 0;break-before:page}p{orphans:3;widows:3}hr{border:0;text-align:center}hr:after{content:"* * *"}@media(prefers-color-scheme:dark){body{background:#181818;color:#eee}}';
// Minimal store-only ZIP implementation. EPUB needs no runtime dependency or shell command.
function crc32(bytes){let c=0xffffffff;for(const byte of bytes){c^=byte;for(let i=0;i<8;i++)c=(c>>>1)^((c&1)?0xedb88320:0);}return(c^0xffffffff)>>>0;}
export function zip(entries){
  const chunks=[],central=[];let offset=0;
  for(const [name,content] of entries){
    const n=Buffer.from(name),data=Buffer.from(content),crc=crc32(data),local=Buffer.alloc(30);
    local.writeUInt32LE(0x04034b50,0);local.writeUInt16LE(20,4);local.writeUInt16LE(0x800,6);local.writeUInt16LE(0x21,12);local.writeUInt32LE(crc,14);local.writeUInt32LE(data.length,18);local.writeUInt32LE(data.length,22);local.writeUInt16LE(n.length,26);
    chunks.push(local,n,data);const record=Buffer.alloc(46);record.writeUInt32LE(0x02014b50,0);record.writeUInt16LE(20,4);record.writeUInt16LE(20,6);record.writeUInt16LE(0x800,8);record.writeUInt16LE(0x21,14);record.writeUInt32LE(crc,16);record.writeUInt32LE(data.length,20);record.writeUInt32LE(data.length,24);record.writeUInt16LE(n.length,28);record.writeUInt32LE(offset,42);central.push(record,n);offset+=local.length+n.length+data.length;
  }
  const directory=Buffer.concat(central),end=Buffer.alloc(22);end.writeUInt32LE(0x06054b50,0);end.writeUInt16LE(entries.length,8);end.writeUInt16LE(entries.length,10);end.writeUInt32LE(directory.length,12);end.writeUInt32LE(offset,16);return Buffer.concat([...chunks,directory,end]);
}
export function render(root,format='md'){
  const state=load(root);if(!state.chapters.length)fail('EMPTY_BOOK','No committed chapters to export.');
  const chapters=state.chapters.map(c=>({...c,prose:body(root,c)}));
  const title=state.project.title;
  if(format==='md')return `# ${title}\n\n${chapters.map(c=>`## ${c.number}. ${c.title}\n\n${c.prose.trim()}\n`).join('\n')}`;
  const sections=chapters.map(c=>`<section id="${c.id}"><h2>${c.number}. ${escape(c.title)}</h2>${proseHTML(c.prose)}</section>`).join('\n');
  if(format==='html')return `<!doctype html><html lang="${escape(state.project.language??'en-NZ')}"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>${escape(title)}</title><style>${STYLE}</style></head><body><h1>${escape(title)}</h1><nav aria-label="Chapters"><ol>${chapters.map(c=>`<li><a href="#${c.id}">${escape(c.title)}</a></li>`).join('')}</ol></nav><main>${sections}</main></body></html>`;
  if(format!=='epub')fail('INVALID_FORMAT','Export format must be md, html or epub.');
  const wrap=(name,html)=>`<?xml version="1.0" encoding="UTF-8"?><html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" lang="${escape(state.project.language??'en-NZ')}"><head><title>${escape(name)}</title><link rel="stylesheet" type="text/css" href="style.css" /></head><body>${html}</body></html>`;
  const nav=wrap(title,`<nav epub:type="toc" id="toc"><h1>${escape(title)}</h1><ol>${chapters.map(c=>`<li><a href="${c.id}.xhtml">${escape(c.title)}</a></li>`).join('')}</ol></nav>`);
  const opf=`<?xml version="1.0" encoding="UTF-8"?><package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="book-id"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:identifier id="book-id">urn:novel-craft:${state.revision}</dc:identifier><dc:title>${escape(title)}</dc:title><dc:language>${escape(state.project.language??'en-NZ')}</dc:language><meta property="dcterms:modified">${state.created_at.replace(/\.\d{3}Z$/,'Z')}</meta></metadata><manifest><item id="nav" href="nav.xhtml" media-type="application/xhtml+xml" properties="nav"/><item id="style" href="style.css" media-type="text/css"/>${chapters.map(c=>`<item id="${c.id}" href="${c.id}.xhtml" media-type="application/xhtml+xml"/>`).join('')}</manifest><spine>${chapters.map(c=>`<itemref idref="${c.id}"/>`).join('')}</spine></package>`;
  return zip([['mimetype','application/epub+zip'],['META-INF/container.xml','<?xml version="1.0"?><container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container"><rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>'],['OEBPS/content.opf',opf],['OEBPS/nav.xhtml',nav],['OEBPS/style.css',STYLE],...chapters.map(c=>[`OEBPS/${c.id}.xhtml`,wrap(c.title,`<h1>${escape(c.title)}</h1>${proseHTML(c.prose)}`)])]);
}
export function exportBook(root,out,format){
  if(!out)fail('OUTPUT_REQUIRED','Export requires --out; existing files are never overwritten.');
  const target=path.resolve(out);if(fs.existsSync(target))fail('OUTPUT_EXISTS','Export destination already exists; choose a new filename.',3);
  const contents=render(root,format);fs.mkdirSync(path.dirname(target),{recursive:true});
  const fd=fs.openSync(target,'wx',0o600);try{fs.writeFileSync(fd,contents);fs.fsyncSync(fd);}finally{fs.closeSync(fd);}
  return {path:target,format,bytes:Buffer.byteLength(contents),revision:load(root).revision};
}
