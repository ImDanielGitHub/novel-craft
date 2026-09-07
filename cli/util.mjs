import fs from 'node:fs';
import path from 'node:path';
import {createHash, randomUUID} from 'node:crypto';
export const VERSION = '0.2.0';
export class CraftError extends Error {
  constructor(code, message, exit = 2, details = {}) { super(message); this.code = code; this.exit = exit; this.details = details; }
}
export function fail(code, message, exit = 2, details) { throw new CraftError(code, message, exit, details); }
export const id = prefix => `${prefix}-${randomUUID()}`;
export const hash = text => createHash('sha256').update(text).digest('hex');
export const json = value => `${JSON.stringify(value, null, 2)}\n`;
export function text(value, name, max = 1000000) {
  if (typeof value !== 'string' || !value.trim() || value.length > max || value.includes('\0')) fail('INVALID_INPUT', `${name} must be non-empty text of at most ${max} characters.`);
  return value;
}
export function integer(value, name, min = 1, max = 10000) {
  const n = Number(value);
  if (!Number.isSafeInteger(n) || n < min || n > max) fail('INVALID_INPUT', `${name} must be an integer from ${min} to ${max}.`);
  return n;
}
export function safeId(value) {
  if (typeof value !== 'string' || !/^[a-z][a-z0-9-]{0,99}$/.test(value)) fail('INVALID_ID', 'Invalid record identifier.');
  return value;
}
export function inside(root, relative) {
  if (typeof relative !== 'string' || path.isAbsolute(relative) || relative.split(/[\\/]/).some(p => p === '..' || p === '.')) fail('UNSAFE_PATH', 'Project paths must not escape the workspace.');
  let current = path.resolve(root);
  for (const part of relative.split(/[\\/]/).filter(Boolean)) {
    current = path.join(current, part);
    try { if (fs.lstatSync(current).isSymbolicLink()) fail('UNSAFE_PATH', `Refusing symbolic link: ${relative}`); }
    catch (e) { if (e.code !== 'ENOENT') throw e; }
  }
  return current;
}
export function readText(file, max = 16000000) {
  if (fs.statSync(file).size > max) fail('INPUT_TOO_LARGE', `Input exceeds ${max} bytes: ${file}`);
  return fs.readFileSync(file, 'utf8');
}
export function readJSON(file) {
  const raw = readText(file);
  try { return JSON.parse(raw); }
  catch { fail('CORRUPT_JSON', `Cannot parse JSON: ${file}`, 3); }
}
export function atomic(file, contents) {
  fs.mkdirSync(path.dirname(file), {recursive:true});
  const temp = `${file}.${randomUUID()}.tmp`;
  let fd;
  try {
    fd = fs.openSync(temp, 'wx', 0o600);
    fs.writeFileSync(fd, contents); fs.fsyncSync(fd); fs.closeSync(fd); fd = undefined;
    fs.renameSync(temp, file);
    try { const dir = fs.openSync(path.dirname(file), 'r'); try { fs.fsyncSync(dir); } finally { fs.closeSync(dir); } } catch { /* Directory fsync is unavailable on some platforms. */ }
  } finally {
    if (fd !== undefined) fs.closeSync(fd);
    if (fs.existsSync(temp)) fs.unlinkSync(temp);
  }
}
export function immutable(file, contents) {
  fs.mkdirSync(path.dirname(file), {recursive:true});
  try { const fd = fs.openSync(file, 'wx', 0o600); try { fs.writeFileSync(fd, contents); fs.fsyncSync(fd); } finally { fs.closeSync(fd); } }
  catch (e) { if (e.code !== 'EEXIST' || readText(file) !== contents) throw e; }
}
export function wordCount(value) {
  return [...new Intl.Segmenter('en', {granularity:'word'}).segment(value)].filter(x => x.isWordLike).length;
}
export const estimateTokens = value => Math.ceil(Buffer.byteLength(typeof value === 'string' ? value : JSON.stringify(value), 'utf8') / 4);
