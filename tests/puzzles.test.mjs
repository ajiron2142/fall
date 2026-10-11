import {test} from 'node:test';
import assert from 'node:assert/strict';
import {createGame,applyAction,gravityStep,foldCells,pathBetween,roomPath,roomHint} from '../puzzles.mjs';

test('100 paper worlds have a complete route, collect all stars and preserve their input state',()=>{
 for(let seed=0;seed<100;seed++){
  let s=createGame('fold',seed);const initial=JSON.stringify(s);
  const original=s;
  for(let i=0;i<3;i++)if(s.folds[i]!==s.solution[i])s=applyAction(s,{type:'fold',panel:i,actor:'A'});
  assert(pathBetween(foldCells(s),s.pawn,{x:2,z:-7}));
  for(let i=0;i<3;i++)s=applyAction(s,{type:'walk',target:{x:1,z:1-i*3},actor:'A'});
  s=applyAction(s,{type:'walk',target:{x:2,z:-7},actor:'A'});
  assert(s.won);assert(s.stars.every(Boolean));assert.equal(JSON.stringify(original),initial);
 }
});
test('folding a occupied sheet carries the walker onto a valid mirrored path',()=>{
 let s=createGame('fold',22);
 if(s.folds[0]!==s.solution[0])s=applyAction(s,{type:'fold',panel:0});
 s=applyAction(s,{type:'walk',target:{x:0,z:2}});
 s=applyAction(s,{type:'fold',panel:0});assert.equal(s.pawn.x,2);assert(foldCells(s).some(c=>c.x===s.pawn.x&&c.z===s.pawn.z));
});
test('100 shuffled room worlds can be restored and both friends can collect keys and reunite',()=>{
 for(let seed=0;seed<100;seed++){
  let s=createGame('rooms',seed);
  const scramble=[...s.scramble];
  for(const a of scramble.reverse())s=applyAction(s,{type:'slide',row:a.row,direction:-a.direction});
  assert.deepEqual(s.board.map(r=>r.id),[0,1,2,3,4,5,6,7,8]);
  for(const room of [2,6,4]){assert(roomPath(s,s.pawns.A,room));s=applyAction(s,{type:'walk',pawn:'A',room});}
  s=applyAction(s,{type:'walk',pawn:'B',room:4});assert(s.won);assert.equal(s.keys.length,2);
 }
});
test('role ownership is enforced only in co-op mode and games serialize',()=>{
 for(const kind of ['fold','rooms','gravity']){const s=createGame(kind,42,'coop');assert.deepEqual(JSON.parse(JSON.stringify(s)),s);}
 let s=createGame('fold',1,'coop');assert.equal(applyAction(s,{type:'fold',panel:0,actor:'B'}),s);assert.notEqual(applyAction(s,{type:'fold',panel:1,actor:'B'}),s);
 s=createGame('rooms',2,'coop');assert.equal(applyAction(s,{type:'slide',row:1,direction:1,actor:'A'}),s);assert.equal(applyAction(s,{type:'walk',pawn:'B',room:4,actor:'A'}),s);
 s=createGame('gravity',3,'coop');assert.equal(applyAction(s,{type:'gate',actor:'A'}),s);assert.equal(applyAction(s,{type:'tilt',x:1,z:1,actor:'B'}),s);assert(applyAction(s,{type:'gate',actor:'B'}).gateOpen);
});

test('room hints find a connected arrangement using the same moves available to a player',()=>{
 for(let seed=0;seed<30;seed++){
  let s=createGame('rooms',seed);
  for(let i=0;i<10;i++){const action=roomHint(s);if(!action)break;s=applyAction(s,action);}
  assert(s.board.every(r=>roomPath(s,4,r.id)));
 }
});
function steer(s,target){
 for(let i=0;i<2400;i++){
  if(s.won)return s;
  const dx=target.x-s.ball.x,dz=target.z-s.ball.z;
  if(Math.hypot(dx,dz)<.12&&Math.hypot(s.ball.vx,s.ball.vz)<.4)return s;
  s=applyAction(s,{type:'tilt',x:dx*.9-s.ball.vx*.5,z:dz*.9-s.ball.vz*.5});s=gravityStep(s,1/120);
 }assert.fail('Marble could not reach '+JSON.stringify(target)+' from '+JSON.stringify(s.ball));
}
test('marble physics can complete all four maze orientations without teleporting or falling',()=>{
 const variants=new Set();
 for(let seed=0;seed<12;seed++){
  let s=createGame('gravity',seed);variants.add(JSON.stringify(s.mirror));const x=s.mirror.x,z=s.mirror.z;
  const points=[[-.65,2.5],[-.65,-2.7],[-2.6,-2.7],[-.65,-2.7],[-.65,0]];
  for(const [a,b] of points)s=steer(s,{x:a*x,z:b*z});
  s=applyAction(s,{type:'gate'});
  for(const [a,b] of [[.65,0],[.65,-2.7],[2.6,-2.7],[.65,-2.7],[.65,2.7],[2.8,2.7],[3,3.4]])s=steer(s,{x:a*x,z:b*z});
  assert(s.won);assert(s.gems.every(Boolean));assert.equal(s.falls,0);
 }assert.equal(variants.size,4);
});
test('closed gate blocks crossing; open gate permits it; brake reduces momentum',()=>{
 let s=createGame('gravity',5);s={...s,ball:{x:-.7,z:0,vx:0,vz:0},tilt:{x:1,z:0}};
 for(let i=0;i<240;i++)s=gravityStep(s,1/120);assert(s.ball.x<0);
 s=applyAction(s,{type:'gate'});for(let i=0;i<120;i++)s=gravityStep(s,1/120);assert(s.ball.x>0);
 s={...s,tilt:{x:0,z:0},ball:{x:1,z:2,vx:2,vz:0}};const braking=gravityStep(applyAction(s,{type:'brake',down:true}),1/60),normal=gravityStep(s,1/60);assert(braking.ball.vx<normal.ball.vx);
});
