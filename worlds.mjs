import * as THREE from './vendor/three.module.js';
import {createGame,applyAction,gravityStep,foldCells,pathBetween,roomPath,roomHint} from './puzzles.mjs';

const $=id=>document.getElementById(id),canvas=$('scene'),stage=$('stage');
const descriptions={
 fold:{title:'Fold Together',subtitle:'PAPER & POSSIBILITY',description:'Fold the three paper islands to join their paths. Tap a connected path to walk. Gather three stars, then reach the far gateway.',help:'Folding carries you with the paper. Gold tiles are paths; blank paper is not.',coop:'One player would walk and fold the outer islands; the other would control the middle island, with different clues on each side of the paper.'},
 gravity:{title:'Pocket Gravity',subtitle:'A LITTLE MOMENTUM',description:'Drag the tilt pad to roll. Collect three crystals, open the middle gate, then settle into the glowing cup. Hold Brake for precision.',help:'The pad tilts the board—not your phone. Dark holes reset the marble; collected crystals stay yours. Arrow keys also work.',coop:'One player would tilt the tray; the other would control the transfer gate. The gate and physics already have separate actions, so roles can be enforced by a shared host.'},
 rooms:{title:'Room Shuffle',subtitle:'MAKE ROOM FOR EACH OTHER',description:'Slide a row to connect its doorways. Choose a character, then tap a reachable room. Collect both keys and bring both friends to the garden.',help:'Rooms carry their characters when they slide. Corridors only connect when both doorways match. Undo lets you experiment.',coop:'Each player would control their own character. One would slide the outer rows; the other would slide the middle row. Both must reach the garden to finish.'}
};
let renderer;
try{renderer=new THREE.WebGLRenderer({canvas,antialias:true,alpha:false});}catch(error){$('description').textContent='This browser cannot start WebGL. Try opening the page in Chrome.';throw error;}
renderer.setPixelRatio(Math.min(devicePixelRatio,1.6));renderer.shadowMap.enabled=true;renderer.shadowMap.type=THREE.PCFSoftShadowMap;renderer.toneMapping=THREE.ACESFilmicToneMapping;renderer.toneMappingExposure=1.35;
const scene=new THREE.Scene();scene.background=new THREE.Color(0xdce6dc);
const camera=new THREE.OrthographicCamera(-8,8,8,-8,.1,100);
scene.add(new THREE.HemisphereLight(0xfff5df,0x6d9485,2.8));
const sun=new THREE.DirectionalLight(0xffedce,3.2);sun.position.set(-8,16,8);sun.castShadow=true;sun.shadow.mapSize.set(1024,1024);sun.shadow.camera.left=-14;sun.shadow.camera.right=14;sun.shadow.camera.top=14;sun.shadow.camera.bottom=-14;sun.shadow.bias=-.001;scene.add(sun);scene.add(sun.target);
const root=new THREE.Group();scene.add(root);const ray=new THREE.Raycaster(),pointer=new THREE.Vector2();
const geometry={box:new THREE.BoxGeometry(1,1,1),sphere:new THREE.SphereGeometry(1,16,12),gem:new THREE.OctahedronGeometry(1),cone:new THREE.ConeGeometry(1,1,7),cylinder:new THREE.CylinderGeometry(1,1,1,12),ring:new THREE.TorusGeometry(1,.06,6,32)};
const materials=new Map();
function material(color,extra={}){const key=color+JSON.stringify(extra);if(!materials.has(key))materials.set(key,new THREE.MeshStandardMaterial({color,roughness:.7,...extra}));return materials.get(key);}
function mesh(type,color,x,y,z,sx=1,sy=1,sz=1,parent=root,extra={}){const m=new THREE.Mesh(geometry[type],material(color,extra));m.position.set(x,y,z);m.scale.set(sx,sy,sz);m.castShadow=true;m.receiveShadow=true;parent.add(m);return m;}
function tile(color,x,y,z,w,h,d,parent=root){return mesh('box',color,x,y,z,w,h,d,parent);}
function pawn(color,parent=root){const g=new THREE.Group();mesh('cylinder',color,0,.25,0,.17,.36,.17,g);mesh('sphere',color,0,.54,0,.23,.23,.23,g);mesh('sphere',0xfff8df,0,.55,.2,.15,.13,.06,g);mesh('sphere',0x24423b,-.055,.56,.25,.025,.04,.025,g);mesh('sphere',0x24423b,.055,.56,.25,.025,.04,.025,g);mesh('cylinder',0xfff9ed,0,.03,0,.3,.05,.3,g);parent.add(g);return g;}
function tree(x,z,size=1,parent=root){const g=new THREE.Group();g.position.set(x,.1,z);mesh('cylinder',0x967359,0,.35,0,.09,.7,.09,g);mesh('cone',0x6f9d7d,0,.9,0,.5,1,.5,g);mesh('cone',0x88ae83,0,1.25,0,.36,.8,.36,g);g.scale.setScalar(size);parent.add(g);return g;}
function flower(x,z,parent=root){mesh('cylinder',0x63977c,x,.16,z,.018,.25,.018,parent);for(let k=0;k<5;k++)mesh('sphere',0xefbda3,x+Math.cos(k*1.256)*.075,.31,z+Math.sin(k*1.256)*.075,.065,.04,.065,parent);}
let kind='fold',state,history=[],selectedPawn='A',objects={},animationTime=0,last=performance.now(),inputPointer=null,toastTime=0;
const seed=()=>crypto.getRandomValues(new Uint32Array(1))[0];
function toast(text){$('toast').textContent=text;toastTime=animationTime+5;}
function resize(){const r=stage.getBoundingClientRect();renderer.setSize(r.width,r.height,false);const aspect=r.width/r.height,half=kind==='fold'?6.8:5.65;camera.left=-half*aspect;camera.right=half*aspect;camera.top=half;camera.bottom=-half;camera.updateProjectionMatrix();}
new ResizeObserver(resize).observe(stage);
function ground(color){tile(color,0,-1.05,kind==='fold'?-2:0,35,.2,35);}
function buildFold(){
 ground(0xb2cfc5);objects.panels=[];objects.stars=[];
 for(let i=0;i<3;i++){
  const group=new THREE.Group();group.position.set(0,.35,1-i*3);root.add(group);group.userData={panel:i};
  const sheet=tile(0xf1ead5,0,0,0,3.02,.19,2.98,group);sheet.userData={panel:i};
  for(const sign of [-1,1])for(const [x,z] of state.templates[i]){const road=tile(0xe7bd85,x-1,sign*.145,z-1,.91,.075,.91,group);road.userData={foldCell:{x,z:z-i*3,panel:i,baseX:x}};}
  // The crease and folded corners make both sides read as a paper sheet.
  tile(0xd7cbae,0,.106,0,.018,.01,2.96,group);tile(0xd7cbae,0,-.106,0,.018,.01,2.96,group);
  objects.panels.push(group);
  const star=mesh('gem',0xd29150,0,1.08,1-i*3,.21,.3,.21,root,{emissive:0x8b5d24,emissiveIntensity:.35});objects.stars.push(star);
 }
 const start=tile(0xf1ead5,-1,.38,3,.93,.3,.93);start.userData={foldCell:{x:0,z:3,panel:-1}};
 const finish=tile(0x7eac91,1,.38,-7,.95,.3,.95);finish.userData={foldCell:{x:2,z:-7,panel:-1}};
 const arch=mesh('ring',0xe8bb82,1,1.13,-7,.42,.55,.42);arch.userData={foldCell:{x:2,z:-7,panel:-1}};
 objects.pawn=pawn(0xd47857);objects.pawn.position.set(-1,.52,3);
 for(const [x,z,s] of [[-3,1,.8],[3,-3,.7],[-2.7,-6,1]]){tile(0xd5dfc6,x,-.15,z,1.4,.25,1.5);tree(x,z,s);flower(x+.3,z+.3);}
 camera.position.set(5,13,12);camera.lookAt(0,.2,-2);sun.target.position.set(0,0,-2);sun.position.set(-8,16,5);
}
function buildGravity(){
 ground(0xe0d3b6);objects.tray=new THREE.Group();root.add(objects.tray);const g=objects.tray;
 tile(0x9a6d49,0,-.2,0,8.1,.6,8.1,g);tile(0xdfbb84,0,.13,0,7.5,.12,7.5,g);
 for(const x of [-3.9,3.9])tile(0xa5784e,x,.34,0,.3,.7,8.1,g);
 for(const z of [-3.9,3.9])tile(0xa5784e,0,.34,z,7.8,.7,.3,g);
 for(const w of state.walls)tile(0xbf9562,w.x,.38,w.z,w.w,.45,w.d,g);
 objects.gate=tile(0xc87454,0,.45,0,.34,.6,1.2,g);
 objects.gems=state.pickups.map(p=>mesh('gem',0x75b696,p.x,.58,p.z,.21,.32,.21,g,{emissive:0x47795e,emissiveIntensity:.35}));
 for(const hole of state.holes){mesh('cylinder',0x624d3a,hole.x,.201,hole.z,hole.r,.02,hole.r,g);mesh('ring',0xc59862,hole.x,.205,hole.z,hole.r,.7,hole.r,g).rotation.x=-Math.PI/2;}
 const p=state.goal;mesh('cylinder',0x687d60,p.x,.205,p.z,.46,.03,.46,g);const ring=mesh('ring',0xaee0b4,p.x,.225,p.z,.48,.8,.48,g,{emissive:0x79b182,emissiveIntensity:.6});ring.rotation.x=-Math.PI/2;
 objects.ball=mesh('sphere',0xd77e42,state.ball.x,.45,state.ball.z,.23,.23,.23,g,{metalness:.25,roughness:.2});
 for(let i=0;i<4;i++){const x=i%2?3.4:-3.4,z=i<2?-3.4:3.4;tile(0x916646,x,-1.05,z,.55,1.4,.55,g);}
 camera.position.set(0,13,10);camera.lookAt(0,0,0);sun.target.position.set(0,0,0);sun.position.set(-8,16,8);
}
function buildRooms(){
 ground(0xbad0bd);objects.rooms=new Map();objects.keys=new Map();
 for(const room of state.board){
  const g=new THREE.Group();g.userData={room:room.id};root.add(g);objects.rooms.set(room.id,g);
  const garden=room.id===4;const floor=tile(garden?0x92b69a:0xf0e5cd,0,.05,0,2.3,.3,2.3,g);floor.userData={room:room.id};
  tile(0xd2b79a,0,-.28,0,2.16,.4,2.16,g);
  for(let d=0;d<4;d++){
   const door=room.doors&(1<<d),horizontal=d%2===0,edge=d===0||d===3?-1.09:1.09;
   if(door){
    for(const side of [-1,1])tile(0xd2c4ab,horizontal?side*.79:edge,.53,horizontal?edge:side*.79,horizontal?.65:.14,.8,horizontal?.14:.65,g);
    tile(garden?0xc6d7aa:0x92b6a1,horizontal?0:edge/2,.215,horizontal?edge/2:0,horizontal?.22:1.1,.035,horizontal?1.1:.22,g);
   }else tile(0xd2c4ab,horizontal?0:edge,.53,horizontal?edge:0,horizontal?2.3:.14,.8,horizontal?.14:2.3,g);
  }
  if(garden){tree(-.65,-.6,.65,g);flower(.6,-.45,g);flower(.67,-.7,g);flower(-.65,.5,g);mesh('cylinder',0xb8d5a7,0,.24,0,.5,.08,.5,g);}
  else {tile(0xbfa489,-.73,.4,-.73,.32,.4,.32,g);mesh('sphere',0x7da584,-.73,.67,-.73,.23,.23,.23,g);}
  if([2,6].includes(room.id)){const key=mesh('gem',0xd99553,.5,.62,.45,.18,.28,.18,g,{emissive:0x835324,emissiveIntensity:.3});objects.keys.set(room.id,key);}
  const index=state.board.indexOf(room);g.position.set((index%3-1)*2.5,0,(Math.floor(index/3)-1)*2.5);
 }
 objects.pawns={A:pawn(0xd47857),B:pawn(0x4b9b8c)};
 camera.position.set(9,14,13);camera.lookAt(0,0,0);sun.target.position.set(0,0,0);sun.position.set(-8,16,8);
}
function build(){root.clear();objects={};scene.background.setHex(kind==='gravity'?0xe9deca:kind==='rooms'?0xd9e3d4:0xdce6dc);({fold:buildFold,gravity:buildGravity,rooms:buildRooms})[kind]();resize();sync(true);}
function choose(next,newSeed=seed()){
 releaseInput();keyboard.clear();kind=next;state=createGame(kind,newSeed);history=[];selectedPawn='A';$('victory').hidden=true;
 const d=descriptions[kind];$('title').textContent=d.title;$('subtitle').textContent=d.subtitle;$('stage-label').textContent=d.subtitle;$('description').textContent=d.description;$('help').textContent=d.help;$('coop').textContent=d.coop;
 document.querySelectorAll('[data-game]').forEach(b=>b.classList.toggle('active',b.dataset.game===kind));buildInputs();build();toast(kind==='fold'?'Fold a sheet, then tap a connected gold path.':kind==='gravity'?'Gentle tilts work best. Open the gate when you are ready.':'Pick a friend. Slide a row. Follow matching doorways.');
}
function send(action){
 if(kind!=='gravity'&&objects.walking?.route.length)return false;
 const before=state;state=applyAction(state,{actor:'A',...action});
 if(state===before)return false;
 if(action.type==='walk'&&kind==='fold')objects.walking={pawn:objects.pawn,route:pathBetween(foldCells(before),before.pawn,action.target).slice(1).map(c=>new THREE.Vector3(c.x-1,.52,c.z))};
 if(action.type==='walk'&&kind==='rooms')objects.walking={pawn:objects.pawns[action.pawn],route:roomPath(before,before.pawns[action.pawn],action.room).slice(1).map(id=>{const i=before.board.findIndex(r=>r.id===id);return new THREE.Vector3((i%3-1)*2.5+(action.pawn==='A'?-.25:.25),.24,(Math.floor(i/3)-1)*2.5);})};
 if(kind!=='gravity'){history.push(before);if(history.length>80)history.shift();}
 sync();return true;
}
function undo(){if(history.length){objects.walking=null;state=history.pop();sync();toast('One move undone. Try a different connection.');}}
function buildInputs(){
 if(kind==='fold'){
  $('inputs').innerHTML='<div class="fold-buttons">'+[0,1,2].map(i=>`<button data-fold="${i}">Fold ${i+1}<span>${['near island','middle island','far island'][i]}</span></button>`).join('')+'</div><div class="move-list"><button id="walk">Walk the connected path ↗</button><button id="hint">Hint</button><button id="undo">Undo</button></div>';
  document.querySelectorAll('[data-fold]').forEach(b=>b.onclick=()=>{send({type:'fold',panel:Number(b.dataset.fold)});toast('Paper flipped. Tap a gold tile to walk onto it.');});
  $('walk').onclick=()=>{const cells=foldCells(state),reachable=cells.filter(c=>pathBetween(cells,state.pawn,c));const star=state.stars.map((v,i)=>v?null:{x:1,z:1-i*3}).find(p=>p&&pathBetween(cells,state.pawn,p));const target=star||reachable.sort((a,b)=>a.z-b.z)[0];if(!send({type:'walk',target:{x:target.x,z:target.z}}))toast('No forward path yet. Fold a sheet so the gold paths meet.');};
  $('hint').onclick=()=>{const i=state.folds.findIndex((v,i)=>v!==state.solution[i]);toast(i>=0?'Try flipping island '+(i+1)+'. Its gold path should meet the next sheet.':'The path is connected. Visit the stars, then the far gateway.');};$('undo').onclick=undo;
 }else if(kind==='gravity'){
  $('inputs').innerHTML='<div class="gravity-controls"><div class="tilt-pad" id="tilt" role="group" aria-label="Drag to tilt the maze; keyboard arrows also work"><div class="tilt-thumb" id="tilt-thumb"></div></div><div class="gravity-buttons"><button id="gate">Open gate<span>connect the two halves</span></button><button id="brake">Hold brake<span>slow the marble down</span></button></div></div>';
  $('gate').addEventListener('pointerdown',e=>{e.preventDefault();send({type:'gate'});});$('gate').addEventListener('click',e=>{if(e.detail===0)send({type:'gate'});});
  $('brake').addEventListener('pointerdown',e=>{e.preventDefault();$('brake').setPointerCapture(e.pointerId);send({type:'brake',down:true});});
  for(const event of ['pointerup','pointercancel','lostpointercapture'])$('brake').addEventListener(event,()=>send({type:'brake',down:false}));
  $('brake').addEventListener('keydown',e=>{if(['Space','Enter'].includes(e.code)){e.preventDefault();send({type:'brake',down:true});}});$('brake').addEventListener('keyup',()=>send({type:'brake',down:false}));
  $('tilt').addEventListener('pointerdown',e=>{if(inputPointer!==null)return;inputPointer=e.pointerId;$('tilt').setPointerCapture(e.pointerId);tiltMove(e);});$('tilt').addEventListener('pointermove',e=>{if(e.pointerId===inputPointer)tiltMove(e);});
  for(const event of ['pointerup','pointercancel','lostpointercapture'])$('tilt').addEventListener(event,e=>{if(e.pointerId===inputPointer)releaseInput();});
 }else{
  $('inputs').innerHTML='<div class="room-controls"><div class="pawn-picks"><button id="pawn-A" class="active">● Coral friend</button><button id="pawn-B">● Mint friend</button></div>'+[0,1,2].map(row=>`<div class="row-control"><button data-row="${row}" data-direction="-1" aria-label="Slide row ${row+1} left">←</button><span>${['Back row','Middle row','Front row'][row]}</span><button data-row="${row}" data-direction="1" aria-label="Slide row ${row+1} right">→</button></div>`).join('')+'</div><div class="move-list"><button id="room-walk">Find a reachable key ↗</button><button id="room-hint">Hint</button><button id="undo">Undo</button></div>';
  for(const id of ['A','B'])$('pawn-'+id).onclick=()=>{selectedPawn=id;sync();toast('Now guiding the '+(id==='A'?'coral':'mint')+' friend. Tap a room with a matching corridor.');};
  document.querySelectorAll('[data-row]').forEach(b=>b.onclick=()=>{send({type:'slide',row:Number(b.dataset.row),direction:Number(b.dataset.direction)});toast('The whole row moved, carrying its friends and doorways.');});
  $('room-walk').onclick=()=>{const targets=[2,6,4].filter(id=>id===4||!state.keys.includes(id));for(const room of targets){if(roomPath(state,state.pawns[selectedPawn],room)?.length>1){send({type:'walk',pawn:selectedPawn,room});return;}}toast('No key or garden is connected yet. Slide a row to line up its doorways.');};
  $('room-hint').onclick=()=>{const hint=roomHint(state);toast(hint?'Try sliding the '+['back','middle','front'][hint.row]+' row '+(hint.direction<0?'left':'right')+'.':'The corridors connect! Guide each friend through the keys and into the garden.');};$('undo').onclick=undo;
 }
}
function tiltMove(e){const r=$('tilt').getBoundingClientRect(),radius=r.width*.34,x=e.clientX-r.left-r.width/2,z=e.clientY-r.top-r.height/2,length=Math.max(radius,Math.hypot(x,z));send({type:'tilt',x:x/length,z:z/length});$('tilt-thumb').style.transform=`translate(${x/length*radius}px,${z/length*radius}px)`;}
function releaseInput(){inputPointer=null;if(state?.kind==='gravity'){state=applyAction(state,{type:'tilt',x:0,z:0,actor:'A'});state=applyAction(state,{type:'brake',down:false,actor:'A'});}if($('tilt-thumb'))$('tilt-thumb').style.transform='';}
function sync(immediate=false){
 if(kind==='fold'){
  objects.panels.forEach((g,i)=>{g.userData.target=state.folds[i]*Math.PI;if(immediate)g.rotation.z=g.userData.target;});
  objects.stars.forEach((m,i)=>m.visible=!state.stars[i]);objects.pawn.userData.target=new THREE.Vector3(state.pawn.x-1,.52,state.pawn.z);if(immediate)objects.pawn.position.copy(objects.pawn.userData.target);
  document.querySelectorAll('[data-fold]').forEach(b=>b.classList.toggle('active',!!state.folds[Number(b.dataset.fold)]));$('score').textContent=state.stars.map(v=>v?'◆':'◇').join('');
 }else if(kind==='rooms'){
  state.board.forEach((tile,i)=>{const g=objects.rooms.get(tile.id);g.userData.target=new THREE.Vector3((i%3-1)*2.5,0,(Math.floor(i/3)-1)*2.5);if(immediate)g.position.copy(g.userData.target);});
  for(const id of ['A','B']){const i=state.board.findIndex(r=>r.id===state.pawns[id]);objects.pawns[id].userData.target=new THREE.Vector3((i%3-1)*2.5+(id==='A'?-.25:.25),.24,(Math.floor(i/3)-1)*2.5);if(immediate)objects.pawns[id].position.copy(objects.pawns[id].userData.target);$('pawn-'+id).classList.toggle('active',selectedPawn===id);}
  objects.keys.forEach((m,id)=>m.visible=!state.keys.includes(id));$('score').textContent=(state.keys.includes(2)?'◆':'◇')+(state.keys.includes(6)?'◆':'◇');
  state.board.forEach(tile=>{const g=objects.rooms.get(tile.id),reachable=!!roomPath(state,state.pawns[selectedPawn],tile.id);g.children[0].material=material(tile.id===4?0x92b69a:reachable?0xd1e2c5:0xf0e5cd);});
 }else{$('score').textContent=state.gems.map(v=>v?'◆':'◇').join('');if($('gate')){$('gate').textContent=state.gateOpen?'Close gate':'Open gate';$('gate').classList.toggle('active',state.gateOpen);}$('brake')?.classList.toggle('active',state.brake);}
 if(state.won&&!objects.walking?.route.length){releaseInput();$('win-title').textContent=kind==='fold'?'You joined the impossible.':kind==='gravity'?'A perfect little landing.':'Together, at last.';$('win-detail').textContent=kind==='gravity'?'Three crystals collected. '+state.falls+' marble rescues.':state.moves+' moves. A little world made whole.';$('victory').hidden=false;}else $('victory').hidden=true;
}
let downPoint=null;
canvas.addEventListener('pointerdown',e=>{downPoint={x:e.clientX,y:e.clientY,id:e.pointerId};});
canvas.addEventListener('pointerup',e=>{
 if(!downPoint||e.pointerId!==downPoint.id||Math.hypot(e.clientX-downPoint.x,e.clientY-downPoint.y)>15)return;downPoint=null;if(state.won||kind==='gravity')return;
 const r=canvas.getBoundingClientRect();pointer.set((e.clientX-r.left)/r.width*2-1,-(e.clientY-r.top)/r.height*2+1);ray.setFromCamera(pointer,camera);const hits=ray.intersectObjects(root.children,true);
 for(const hit of hits){let o=hit.object;while(o&&o!==root){
  if(kind==='fold'&&o.userData.foldCell){const c=o.userData.foldCell,x=c.panel<0?c.x:state.folds[c.panel]?2-c.baseX:c.baseX;const ok=send({type:'walk',target:{x,z:c.z}});if(!ok)toast('That path is not connected to you yet. Try folding a sheet.');return;}
  if(kind==='rooms'&&o.userData.room!==undefined){if(!send({type:'walk',pawn:selectedPawn,room:o.userData.room}))toast('Those rooms need matching doorways. Slide a row first.');return;}
  if(kind==='fold'&&o.userData.panel!==undefined){send({type:'fold',panel:o.userData.panel});return;}o=o.parent;
 }}
});canvas.addEventListener('pointercancel',()=>downPoint=null);
const keyboard=new Set();
window.addEventListener('keydown',e=>{if(kind!=='gravity'||state.won||!['ArrowUp','ArrowDown','ArrowLeft','ArrowRight','Space'].includes(e.code))return;if(e.target.closest('button')&&e.code==='Space')return;e.preventDefault();keyboard.add(e.code);keyboardInput();});window.addEventListener('keyup',e=>{keyboard.delete(e.code);if(kind==='gravity')keyboardInput();});
function keyboardInput(){send({type:'tilt',x:(keyboard.has('ArrowRight')?1:0)-(keyboard.has('ArrowLeft')?1:0),z:(keyboard.has('ArrowDown')?1:0)-(keyboard.has('ArrowUp')?1:0)});send({type:'brake',down:keyboard.has('Space')});}
window.addEventListener('blur',()=>{keyboard.clear();releaseInput();});document.addEventListener('visibilitychange',()=>{if(document.hidden){keyboard.clear();releaseInput();}});
async function fullscreen(){try{if(document.fullscreenElement)await document.exitFullscreen();else if(document.documentElement.requestFullscreen)await document.documentElement.requestFullscreen();else toast('Use Share → Add to Home Screen to open without browser bars.');}catch{toast('Open this page directly in your browser for fullscreen.');}}
$('fullscreen').onclick=fullscreen;document.querySelectorAll('[data-game]').forEach(b=>b.onclick=()=>choose(b.dataset.game));$('new').onclick=()=>choose(kind);$('again').onclick=()=>choose(kind);
function frame(now){requestAnimationFrame(frame);const dt=Math.min((now-last)/1000,.05);last=now;animationTime+=dt;
 if(kind==='gravity'){
  const before=state;let t=dt;while(t>0){const step=Math.min(t,1/120);state=gravityStep(state,step);t-=step;}objects.ball.position.set(state.ball.x,.45,state.ball.z);objects.ball.rotation.x+=state.ball.vz*dt*4;objects.ball.rotation.z-=state.ball.vx*dt*4;
  objects.tray.rotation.x=THREE.MathUtils.damp(objects.tray.rotation.x,-state.tilt.z*.07,7,dt);objects.tray.rotation.z=THREE.MathUtils.damp(objects.tray.rotation.z,state.tilt.x*.07,7,dt);objects.gate.position.y=THREE.MathUtils.damp(objects.gate.position.y,state.gateOpen?-.5:.45,9,dt);objects.gems.forEach((m,i)=>{m.visible=!state.gems[i];m.rotation.y=animationTime;m.position.y=.58+Math.sin(animationTime*2+i)*.08;});if(before.gems.join()!==state.gems.join()||before.won!==state.won)sync();if(before.falls!==state.falls)toast('Marble rescued. Your collected crystals are safe.');
 }else if(kind==='fold'){
  objects.panels.forEach(g=>g.rotation.z=THREE.MathUtils.damp(g.rotation.z,g.userData.target,9,dt));if(objects.walking?.pawn!==objects.pawn)objects.pawn.position.lerp(objects.pawn.userData.target,1-Math.exp(-10*dt));objects.stars.forEach((m,i)=>{m.rotation.y=animationTime*.7;m.position.y=1.08+Math.sin(animationTime*2+i)*.06;});
 }else{
  objects.rooms.forEach(g=>g.position.lerp(g.userData.target,1-Math.exp(-12*dt)));for(const p of Object.values(objects.pawns))if(objects.walking?.pawn!==p)p.position.lerp(p.userData.target,1-Math.exp(-12*dt));objects.keys.forEach(m=>m.rotation.y=animationTime);
 }
 if(objects.walking?.route.length){const {pawn,route}=objects.walking;const delta=route[0].clone().sub(pawn.position),distance=delta.length(),step=dt*8;if(distance<=step){pawn.position.copy(route.shift());if(!route.length){objects.walking=null;sync();}}else pawn.position.addScaledVector(delta,step/distance);}
 if(animationTime>toastTime)$('toast').textContent=kind==='gravity'?'Gentle tilt · hold brake to slow down':kind==='fold'?'Tap gold paths to walk · fold sheets to connect them':'Tap a mint-lit room to walk there';renderer.render(scene,camera);
}
choose(kind);requestAnimationFrame(frame);
// Diagnostics are read-only; all game changes go through actions.
window.littleWorldsState=()=>structuredClone(state);
