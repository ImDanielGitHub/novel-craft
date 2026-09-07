import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import childProcess from 'node:child_process';
import {syncBuiltinESMExports} from 'node:module';
import {EventEmitter} from 'node:events';
import {PassThrough} from 'node:stream';
import {runModel} from '../runner.mjs';

// Reproduce Windows' locked-current-directory behaviour deterministically on any OS.
async function interruptedRunner(alwaysLocked=false) {
  const originalSpawn=childProcess.spawn;
  const originalRemove=fs.rmSync;
  let closed=false,removedBeforeClose=false,temporaryDirectory;
  childProcess.spawn=()=>{
    const child=new EventEmitter();
    child.stdout=new PassThrough();child.stderr=new PassThrough();child.stdin=new PassThrough();
    child.kill=()=>true;child.unref=()=>{};
    setTimeout(()=>{closed=true;child.stdout.end();child.stderr.end();child.emit('close',null);},180);
    return child;
  };
  fs.rmSync=(target,options)=>{
    temporaryDirectory=target;
    if(!closed || alwaysLocked) {
      removedBeforeClose ||= !closed;
      throw Object.assign(new Error('Simulated Windows directory lock'),{code:'EPERM'});
    }
    return originalRemove(target,options);
  };
  syncBuiltinESMExports();
  let error;
  try {await runModel('review',{}, {command:'mock-agent',timeout:50});}
  catch(e){error=e;}
  finally {
    childProcess.spawn=originalSpawn;fs.rmSync=originalRemove;syncBuiltinESMExports();
    await new Promise(resolve=>setTimeout(resolve,200));
    if(temporaryDirectory) originalRemove(temporaryDirectory,{recursive:true,force:true});
  }
  return {error,removedBeforeClose,closed,temporaryDirectory};
}

test('timeout waits for process close before removing its current directory',async()=>{
  const result=await interruptedRunner();
  assert.equal(result.error.code,'RUNNER_TIMEOUT');
  assert.equal(result.removedBeforeClose,false);
  assert.equal(result.closed,true);
});

test('cleanup failure preserves the original timeout and identifies pending cleanup',async()=>{
  const result=await interruptedRunner(true);
  assert.equal(result.error.code,'RUNNER_TIMEOUT');
  assert.equal(result.error.details.cleanup_pending,result.temporaryDirectory);
});
