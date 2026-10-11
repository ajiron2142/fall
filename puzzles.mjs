// DOM-free, serializable rules. UI and a future network transport send the same actions.
export function rng(seed){return()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};}
const clone=s=>structuredClone(s);
export function allowed(state,actor,role){return state.mode!=='coop'||actor===role;}
const cellKey=(x,z)=>`${x},${z}`;
const deltas=[[0,-1],[1,0],[0,1],[-1,0]];
export function pathBetween(cells,start,target){
 const valid=new Set(cells.map(c=>cellKey(c.x,c.z))),queue=[start],parents=new Map([[cellKey(start.x,start.z),null]]);
 for(let i=0;i<queue.length;i++){
  const p=queue[i];if(p.x===target.x&&p.z===target.z){const route=[];let c=p;while(c){route.unshift(c);c=parents.get(cellKey(c.x,c.z));}return route;}
  for(const [dx,dz] of deltas){const n={x:p.x+dx,z:p.z+dz},k=cellKey(n.x,n.z);if(valid.has(k)&&!parents.has(k)){parents.set(k,p);queue.push(n);}}
 }return null;
}

export function makeFold(seed,mode='solo'){
 const r=rng(seed),variants=[[[0,2],[0,1],[1,1],[2,1],[2,0]],[[0,2],[1,2],[1,1],[2,1],[2,0]],[[0,2],[0,1],[0,0],[1,0],[1,1],[2,1],[2,0]]],solution=[],templates=[];
 for(let i=0;i<3;i++){const reversed=r()<.5;solution.push((i%2)^(reversed?1:0));templates.push(variants[Math.floor(r()*variants.length)].map(([x,z])=>[reversed?2-x:x,z]));}
 const folds=solution.map(v=>r()<.8?1-v:v);
 if(folds.every((v,i)=>v===solution[i]))folds[0]=1-folds[0];
 return {kind:'fold',seed,mode,folds,templates,solution,pawn:{x:0,z:3},stars:[false,false,false],moves:0,won:false};
}
export function foldCells(s){
 const cells=[{x:0,z:3,panel:-1},{x:2,z:-7,panel:-1}];
 s.folds.forEach((fold,i)=>s.templates[i].forEach(([x,z])=>cells.push({x:fold?2-x:x,z:z-i*3,panel:i})));
 return cells;
}
export function foldAction(s,a){
 if(s.won)return s;
 if(a.type==='fold'&&Number.isInteger(a.panel)&&a.panel>=0&&a.panel<3&&allowed(s,a.actor,a.panel%2?'B':'A')){
  const n=clone(s);n.folds[a.panel]=1-n.folds[a.panel];
  const panel=foldCells(s).find(c=>c.x===s.pawn.x&&c.z===s.pawn.z)?.panel;
  if(panel===a.panel)n.pawn.x=2-n.pawn.x;
  n.moves++;return n;
 }
 if(a.type==='walk'&&allowed(s,a.actor,'A')){
  const path=pathBetween(foldCells(s),s.pawn,a.target);if(!path||path.length<2)return s;
  const n=clone(s);n.pawn={...a.target};n.moves++;
  for(const p of path)for(let i=0;i<3;i++)if(p.x===1&&p.z===1-i*3)n.stars[i]=true;
  n.won=n.stars.every(Boolean)&&n.pawn.x===2&&n.pawn.z===-7;return n;
 }return s;
}

const opposite=d=>(d+2)%4;
export function makeRooms(seed,mode='solo'){
 const random=rng(seed),doors=Array(9).fill(0),visited=new Set([4]);
 function visit(i){
  const directions=[0,1,2,3].map(d=>({d,r:random()})).sort((a,b)=>a.r-b.r);
  for(const {d} of directions){const x=i%3+deltas[d][0],z=Math.floor(i/3)+deltas[d][1];if(x<0||x>2||z<0||z>2)continue;const j=z*3+x;if(visited.has(j))continue;doors[i]|=1<<d;doors[j]|=1<<opposite(d);visited.add(j);visit(j);}
 }visit(4);
 // A few extra corridors preserve choices without making every room connected after scrambling.
 for(let i=0;i<9;i++)for(const d of [1,2]){const x=i%3+deltas[d][0],z=Math.floor(i/3)+deltas[d][1];if(x<3&&z<3&&random()<.2){const j=z*3+x;doors[i]|=1<<d;doors[j]|=1<<opposite(d);}}
 let s={kind:'rooms',seed,mode:'solo',board:Array.from({length:9},(_,id)=>({id,doors:doors[id]})),pawns:{A:0,B:8},keys:[],moves:0,won:false,scramble:[]};
 for(let i=0;i<7;i++){const row=Math.floor(random()*3),direction=random()<.5?-1:1;s=roomsAction(s,{type:'slide',row,direction,actor:'A'});s.scramble.push({row,direction});}
 s.moves=0;s.mode=mode;return s;
}
export function roomPath(s,from,to){
 const start=s.board.findIndex(t=>t.id===from),end=s.board.findIndex(t=>t.id===to),queue=[start],parents=new Map([[start,null]]);
 if(start<0||end<0)return null;
 for(let i=0;i<queue.length;i++){
  const p=queue[i];if(p===end){const path=[];let v=p;while(v!==null){path.unshift(s.board[v].id);v=parents.get(v);}return path;}
  for(let d=0;d<4;d++){if(!(s.board[p].doors&(1<<d)))continue;const x=p%3+deltas[d][0],z=Math.floor(p/3)+deltas[d][1];if(x<0||x>2||z<0||z>2)continue;const j=z*3+x;if((s.board[j].doors&(1<<opposite(d)))&&!parents.has(j)){parents.set(j,p);queue.push(j);}}
 }return null;
}
export function roomsAction(s,a){
 if(s.won)return s;
 if(a.type==='slide'&&Number.isInteger(a.row)&&a.row>=0&&a.row<3&&[-1,1].includes(a.direction)&&allowed(s,a.actor,a.row===1?'B':'A')){
  const n=clone(s),tiles=n.board.slice(a.row*3,a.row*3+3);
  for(let x=0;x<3;x++)n.board[a.row*3+(x+a.direction+3)%3]=tiles[x];
  n.moves++;return n;
 }
 if(a.type==='walk'&&['A','B'].includes(a.pawn)&&allowed(s,a.actor,a.pawn)){
  const path=roomPath(s,s.pawns[a.pawn],a.room);if(!path||path.length<2)return s;
  const n=clone(s);n.pawns[a.pawn]=a.room;n.moves++;
  for(const id of path)if([2,6].includes(id)&&!n.keys.includes(id))n.keys.push(id);
  n.won=n.keys.length===2&&n.pawns.A===4&&n.pawns.B===4;return n;
 }return s;
}

export function roomHint(s){
 const queue=[{s,first:null}],seen=new Set([s.board.map(r=>r.id).join(',')]);
 for(let i=0;i<queue.length;i++){
  const current=queue[i];if(current.s.board.every(r=>roomPath(current.s,4,r.id)))return current.first;
  for(let row=0;row<3;row++)for(const direction of [-1,1]){const a={type:'slide',row,direction,actor:row===1?'B':'A'},next=roomsAction(current.s,a),key=next.board.map(r=>r.id).join(',');if(!seen.has(key)){seen.add(key);queue.push({s:next,first:current.first||a});}}
 }return null;
}
export function makeGravity(seed,mode='solo'){
 const r=rng(seed),mirror={x:r()<.5?-1:1,z:r()<.5?-1:1};
 const walls=[{x:0,z:-2.3,w:.3,d:3.4},{x:0,z:2.3,w:.3,d:3.4},{x:-2.1,z:.6,w:2.2,d:.3},{x:2.1,z:-.6,w:2.2,d:.3}];
 const s={kind:'gravity',seed,mode,mirror,ball:{x:-3,z:3,vx:0,vz:0},tilt:{x:0,z:0},walls,gateOpen:false,brake:false,gems:[false,false,false],falls:0,won:false,time:0,
  pickups:[{x:-2.6,z:-2.7},{x:2.6,z:-2.7},{x:2.8,z:2.7}],holes:[{x:-2.7,z:1.65,r:.4},{x:2.7,z:1.3,r:.4}],goal:{x:3,z:3.4}};
 for(const p of [s.ball,...s.walls,...s.pickups,...s.holes,s.goal]){p.x*=mirror.x;p.z*=mirror.z;}
 s.start={x:s.ball.x,z:s.ball.z};return s;
}
export function gravityAction(s,a){
 if(s.won)return s;
 if(a.type==='tilt'&&allowed(s,a.actor,'A')&&Number.isFinite(a.x)&&Number.isFinite(a.z)){return {...s,tilt:{x:Math.max(-1,Math.min(1,a.x)),z:Math.max(-1,Math.min(1,a.z))}};}
 if(a.type==='gate'&&allowed(s,a.actor,'B'))return {...s,gateOpen:!s.gateOpen};
 if(a.type==='brake'&&allowed(s,a.actor,'A'))return {...s,brake:Boolean(a.down)};
 return s;
}
export function gravityStep(state,dt){
 if(state.won)return state;dt=Math.max(0,Math.min(dt,1/60));
 const s={...state,ball:{...state.ball},gems:[...state.gems],time:state.time+dt},b=s.ball,r=.23;
 const damping=Math.exp(-(s.brake?13:1.35)*dt);
 b.vx=(b.vx+s.tilt.x*9*dt)*damping;b.vz=(b.vz+s.tilt.z*9*dt)*damping;b.x+=b.vx*dt;b.z+=b.vz*dt;
 for(const axis of ['x','z']){const v=axis==='x'?'vx':'vz';if(Math.abs(b[axis])>3.7-r){b[axis]=Math.sign(b[axis])*(3.7-r);b[v]*=-.35;}}
 const barriers=s.gateOpen?s.walls:[...s.walls,{x:0,z:0,w:.3,d:1.2}];
 for(const w of barriers){
  const x=Math.max(w.x-w.w/2,Math.min(w.x+w.w/2,b.x)),z=Math.max(w.z-w.d/2,Math.min(w.z+w.d/2,b.z)),dx=b.x-x,dz=b.z-z,dist=Math.hypot(dx,dz);
  if(dist>=r)continue;
  if(dist>1e-7){const nx=dx/dist,nz=dz/dist;b.x+=nx*(r-dist);b.z+=nz*(r-dist);const speed=b.vx*nx+b.vz*nz;if(speed<0){b.vx-=1.35*speed*nx;b.vz-=1.35*speed*nz;}}
  else {const px=w.w/2-Math.abs(b.x-w.x),pz=w.d/2-Math.abs(b.z-w.z);if(px<pz){b.x=w.x+Math.sign(b.x-w.x||-b.vx||1)*(w.w/2+r);b.vx*= -.35;}else{b.z=w.z+Math.sign(b.z-w.z||-b.vz||1)*(w.d/2+r);b.vz*= -.35;}}
 }
 for(let i=0;i<3;i++)if(Math.hypot(b.x-s.pickups[i].x,b.z-s.pickups[i].z)<.5)s.gems[i]=true;
 for(const hole of s.holes)if(Math.hypot(b.x-hole.x,b.z-hole.z)<hole.r*.7){s.falls++;const p=s.gems[0]?s.pickups[0]:s.start;s.ball={x:p.x,z:p.z,vx:0,vz:0};break;}
 s.won=s.gems.every(Boolean)&&Math.hypot(s.ball.x-s.goal.x,s.ball.z-s.goal.z)<.48&&Math.hypot(s.ball.vx,s.ball.vz)<1.5;
 return s;
}
export function createGame(kind,seed,mode='solo'){return ({fold:makeFold,gravity:makeGravity,rooms:makeRooms})[kind](seed,mode);}
export function applyAction(state,action){return ({fold:foldAction,gravity:gravityAction,rooms:roomsAction})[state.kind](state,action);}
