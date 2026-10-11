import * as THREE from './vendor/three.module.js';
const $=id=>document.getElementById(id),canvas=$('world');
let renderer;
try{renderer=new THREE.WebGLRenderer({canvas,antialias:true,powerPreference:'high-performance'});}catch(e){$('start').disabled=true;$('start').textContent='WebGL unavailable on this browser';throw e;}
renderer.setPixelRatio(Math.min(devicePixelRatio,1.6));renderer.setSize(innerWidth,innerHeight);renderer.toneMapping=THREE.ACESFilmicToneMapping;renderer.toneMappingExposure=1.25;
const scene=new THREE.Scene();scene.background=new THREE.Color(0x102c38);scene.fog=new THREE.FogExp2(0x102c38,.012);
const camera=new THREE.PerspectiveCamera(76,innerWidth/innerHeight,.1,350);camera.rotation.order='YXZ';
scene.add(new THREE.HemisphereLight(0xb6f5ec,0x152338,2.5));const sun=new THREE.DirectionalLight(0xffdcaa,3);sun.position.set(25,45,-35);scene.add(sun);
const world=new THREE.Group();scene.add(world);const geo=new THREE.BoxGeometry(1,1,1),orbGeo=new THREE.IcosahedronGeometry(.6,1);
const seaMaterial=new THREE.ShaderMaterial({uniforms:{time:{value:0},tint:{value:new THREE.Color(0x8df1d2)}},vertexShader:'varying vec3 point; void main(){vec4 p=modelMatrix*vec4(position,1.0);point=p.xyz;gl_Position=projectionMatrix*viewMatrix*p;}',fragmentShader:'varying vec3 point; uniform float time; uniform vec3 tint; void main(){float waves=sin(point.x*.23+time*.4)*sin(point.z*.16-time*.3);float lines=pow(max(0.0,sin(point.x*.07+point.z*.11+waves*.3+time*.2)),28.0);float fade=exp(-length(point.xz-vec2(0.0,-50.0))*.007);gl_FragColor=vec4(mix(vec3(.025,.075,.095),tint*.28,lines*fade)+waves*.009,1.0);}'});
const sea=new THREE.Mesh(new THREE.PlaneGeometry(600,600),seaMaterial);sea.rotation.x=-Math.PI/2;sea.position.set(0,-17,-70);scene.add(sea);
const moon=new THREE.Mesh(new THREE.SphereGeometry(7,24,16),new THREE.MeshBasicMaterial({color:0xa1dacd}));moon.position.set(38,42,-150);scene.add(moon);
const halo=new THREE.Mesh(new THREE.TorusGeometry(12,.035,6,100),new THREE.MeshBasicMaterial({color:0x8df1d2,transparent:true,opacity:.5}));halo.position.copy(moon.position);scene.add(halo);
const palettes=[{sky:0x102c38,solid:0x365862,accent:0x8df1d2,name:'01 / TIDE'},{sky:0x372436,solid:0x685161,accent:0xffb782,name:'02 / EMBER'}];
let platforms=[],anchors=[],phase=0,running=false,won=false,checkpoint=0,falls=0,seed=0,yaw=0,pitch=-.13,grounded=false,dashCooldown=0,jumpQueued=false,dashQueued=false,shiftCooldown=0;
let coyoteTime=0,jumpBuffer=0;
const player={x:0,y:0,z:1,vx:0,vy:0,vz:0},keys=new Set(),stick={x:0,y:0};let toastUntil=0,last=performance.now(),elapsed=0;
function randomFactory(n){return()=>{n|=0;n=n+0x6D2B79F5|0;let t=Math.imul(n^n>>>15,1|n);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};}
function box(x,y,z,w,h,d,material){const m=new THREE.Mesh(geo,material);m.position.set(x,y,z);m.scale.set(w,h,d);world.add(m);return m;}
function say(text,duration=4){$('message').textContent=text;toastUntil=elapsed+duration;}
function disposeWorld(){const materials=new Set();world.traverse(o=>{if(o.material)materials.add(o.material);if(o.isInstancedMesh)o.dispose();if(o.geometry&&o.geometry!==geo&&o.geometry!==orbGeo)o.geometry.dispose();});world.clear();materials.forEach(m=>m.dispose());}
function generate(){
 disposeWorld();seed=crypto.getRandomValues(new Uint32Array(1))[0];const rand=randomFactory(seed);platforms=[];anchors=[];checkpoint=0;falls=0;won=false;phase=0;elapsed=0;let x=0,y=0;
 for(let i=0;i<17;i++){
  if(i>3){x+=(rand()-.5)*5.2;y+=(rand()-.5)*.65;}const z=i<=3?-i*6.6:-(19.8+(i-3)*7.2),shared=i%4===0,type=i<=3?-1:shared?-1:Math.floor((i-1)/2)%2;
  const mat=new THREE.MeshStandardMaterial({color:0x365862,roughness:.7,metalness:.15,transparent:true});const mesh=box(x,y-.6,z,6,1.2,6,mat),p={x,y,z,type,mesh,index:i};platforms.push(p);
  const edge=new THREE.LineSegments(new THREE.EdgesGeometry(geo),new THREE.LineBasicMaterial({color:0x8df1d2,transparent:true,opacity:.6}));edge.scale.set(6.03,1.23,6.03);edge.position.copy(mesh.position);world.add(edge);p.edge=edge;
  p.support=box(x,y-3,z,4.2,4.5,4.2,new THREE.MeshStandardMaterial({color:0x172e39,roughness:1}));
  if(shared&&i>0&&i<16){const orb=new THREE.Mesh(orbGeo,new THREE.MeshStandardMaterial({color:0x8df1d2,emissive:0x8df1d2,emissiveIntensity:2,metalness:.4,roughness:.2}));orb.position.set(x,y+1.65,z);world.add(orb);const ring=new THREE.Mesh(new THREE.TorusGeometry(1.15,.025,6,32),new THREE.MeshBasicMaterial({color:0x8df1d2}));ring.position.copy(orb.position);world.add(ring);anchors.push({p,orb,ring,charged:false});}
  if(shared){const m=new THREE.MeshStandardMaterial({color:0x30434e,roughness:.9});box(x-2.6,y+1.3,z-2.6,.35,3.8,.35,m);box(x+2.6,y+.8,z-2.6,.35,2.8,.35,m);}
 }
 const end=platforms.at(-1),gate=new THREE.Mesh(new THREE.TorusGeometry(1.8,.16,8,48),new THREE.MeshStandardMaterial({color:0xffce9b,emissive:0xffb782,emissiveIntensity:1.8}));gate.position.set(end.x,end.y+2,end.z-1);world.add(gate);
 const cityMat=new THREE.MeshStandardMaterial({color:0x18323f,roughness:1}),city=new THREE.InstancedMesh(geo,cityMat,100),transform=new THREE.Object3D();for(let i=0;i<100;i++){const side=rand()<.5?-1:1,cx=side*(16+rand()*65),cz=18-rand()*170,h=8+rand()*38;transform.position.set(cx,-18+h/2,cz);transform.scale.set(2+rand()*5,h,2+rand()*5);transform.updateMatrix();city.setMatrixAt(i,transform.matrix);}world.add(city);
 const positions=new Float32Array(600*3);for(let i=0;i<600;i++){positions[i*3]=(rand()-.5)*200;positions[i*3+1]=rand()*65-12;positions[i*3+2]=20-rand()*200;}const starsGeo=new THREE.BufferGeometry();starsGeo.setAttribute('position',new THREE.BufferAttribute(positions,3));world.add(new THREE.Points(starsGeo,new THREE.PointsMaterial({color:0xb6f5ec,size:.1,transparent:true,opacity:.65})));
 setPhase();respawn(false);$('anchors').textContent='◇ ◇ ◇';say('Hold MOVE forward. Press JUMP with your other thumb near the edge.',10);
}
function setPhase(){const c=palettes[phase];scene.background.setHex(c.sky);scene.fog.color.setHex(c.sky);seaMaterial.uniforms.tint.value.setHex(c.accent);moon.material.color.setHex(c.accent);halo.material.color.setHex(c.accent);$('phase-label').textContent=c.name;$('phase-label').style.color='#'+c.accent.toString(16);$('shift').style.background='#'+c.accent.toString(16);platforms.forEach(p=>{const solid=p.type===-1||p.type===phase;p.mesh.material.color.setHex(c.solid);p.mesh.material.opacity=solid?1:.09;p.mesh.material.depthWrite=solid;p.edge.material.color.setHex(c.accent);p.edge.material.opacity=solid?.65:.23;p.support.visible=solid;});}
function shift(){if(!running||shiftCooldown>0)return;phase=1-phase;shiftCooldown=.18;setPhase();say(phase?'Ember. A different path becomes real.':'Tide. The world rearranges around you.',2);}
function respawn(count=true){const p=platforms[checkpoint];if(p.type!==-1){phase=p.type;setPhase();}Object.assign(player,{x:p.x,y:p.y+.05,z:p.z+1,vx:0,vy:0,vz:0});grounded=false;coyoteTime=jumpBuffer=0;jumpQueued=dashQueued=false;yaw=0;pitch=-.13;dashCooldown=0;if(count){falls++;say('Back on your last platform. Keep moving while you jump.',4);}}
function physics(dt){
 dashCooldown=Math.max(0,dashCooldown-dt);shiftCooldown=Math.max(0,shiftCooldown-dt);
 coyoteTime=grounded?.16:Math.max(0,coyoteTime-dt);
 jumpBuffer=jumpQueued?.2:Math.max(0,jumpBuffer-dt);jumpQueued=false;
 const forward=(keys.has('KeyW')||keys.has('ArrowUp')?1:0)-(keys.has('KeyS')||keys.has('ArrowDown')?1:0)-stick.y,side=(keys.has('KeyD')||keys.has('ArrowRight')?1:0)-(keys.has('KeyA')||keys.has('ArrowLeft')?1:0)+stick.x,length=Math.max(1,Math.hypot(forward,side));
 const fx=(-Math.sin(yaw)*forward+Math.cos(yaw)*side)/length,fz=(-Math.cos(yaw)*forward-Math.sin(yaw)*side)/length,blend=1-Math.exp(-(grounded?11:4)*dt);player.vx+=(fx*8-player.vx)*blend;player.vz+=(fz*8-player.vz)*blend;
 if(jumpBuffer>0&&coyoteTime>0){player.vy=10.2;grounded=false;coyoteTime=jumpBuffer=0;}if(dashQueued){if(dashCooldown<=0){player.vx=-Math.sin(yaw)*19;player.vz=-Math.cos(yaw)*19;player.vy=Math.max(player.vy,2);dashCooldown=1.1;}dashQueued=false;}
 const oldY=player.y;player.vy-=22*dt;player.x+=player.vx*dt;player.z+=player.vz*dt;player.y+=player.vy*dt;grounded=false;
 if(player.vy<=0)for(const p of platforms){if((p.type===-1||p.type===phase)&&Math.abs(player.x-p.x)<3.18&&Math.abs(player.z-p.z)<3.18&&oldY>=p.y-.03&&player.y<=p.y){player.y=p.y;player.vy=0;grounded=true;checkpoint=Math.max(checkpoint,p.index);break;}}
 if(player.y<-20)respawn();
 for(const a of anchors)if(!a.charged&&grounded&&Math.hypot(player.x-a.p.x,player.z-a.p.z)<1.9){a.charged=true;checkpoint=a.p.index;a.orb.material.color.setHex(0xffdfb0);a.orb.material.emissive.setHex(0xffb782);$('anchors').textContent=anchors.map(a=>a.charged?'◆':'◇').join(' ');say('Anchor '+anchors.filter(a=>a.charged).length+' awakened. Your new checkpoint.',5);}
 const end=platforms.at(-1);if(!won&&grounded&&Math.hypot(player.x-end.x,player.z-end.z)<2){if(anchors.every(a=>a.charged)){won=true;running=false;document.exitPointerLock?.();$('overlay').classList.remove('hidden');$('start').textContent='Explore this world again ↗';document.querySelector('h1').innerHTML='You crossed<br>the impossible.';$('description').textContent='Three anchors. Two realities. '+falls+' falls. Generate another world to try a new route.';}else say('The gateway needs all three anchors. Seek the floating lights.',4);}
}
function animate(now){requestAnimationFrame(animate);const dt=Math.min((now-last)/1000,.04);last=now;elapsed+=dt;if(running){let remaining=dt;while(remaining>0){const step=Math.min(remaining,1/120);physics(step);remaining-=step;}}
 for(const a of anchors){a.orb.rotation.y=elapsed*.6;a.orb.position.y=a.p.y+1.65+Math.sin(elapsed*2+a.p.index)*.13;a.ring.rotation.y=elapsed*.4;a.ring.rotation.x=Math.sin(elapsed*.5)*.3;}
 camera.position.set(player.x,player.y+1.6,player.z);if(!running&&!won&&!$('overlay').classList.contains('hidden')){camera.position.set(10,8,12);camera.lookAt(0,0,-22);}else camera.rotation.set(pitch,yaw,Math.max(-.035,Math.min(.035,-player.vx*.002)),'YXZ');
 if(running&&elapsed>toastUntil)$('message').textContent=grounded?'Look for the next solid island. Jump across the gaps.':'Shift midair to land in the other reality.';
 $('dash').style.opacity=dashCooldown>0?'.45':'1';seaMaterial.uniforms.time.value=elapsed;renderer.render(scene,camera);
}
function enter(){if(won)generate();running=true;$('overlay').classList.add('hidden');if(matchMedia('(pointer:fine)').matches)canvas.requestPointerLock?.();last=performance.now();}
function pause(){running=false;keys.clear();stick.x=stick.y=0;jumpQueued=dashQueued=false;jumpBuffer=coyoteTime=0;lookPointer=stickPointer=null;$('thumb').style.transform='';document.exitPointerLock?.();$('overlay').classList.remove('hidden');$('start').textContent='Continue exploring ↗';}
$('start').onclick=enter;$('menu').onclick=pause;$('new-world').onclick=()=>{generate();document.querySelector('h1').innerHTML='One place.<br>Two realities.';$('description').textContent='Shift between realities. Wake three anchors to open the way home.';$('start').textContent='Enter the afterlight ↗';};
// Secondary fingers do not reliably produce click events on touch browsers.
// Act on pointerdown instead, leaving the joystick's pointer captured.
function actionButton(id,action){const b=$(id);b.addEventListener('pointerdown',e=>{e.preventDefault();action();});b.addEventListener('click',e=>{if(e.detail===0)action();});}
actionButton('shift',shift);actionButton('jump',()=>{if(running)jumpQueued=true;});actionButton('dash',()=>{if(running)dashQueued=true;});
async function fullscreen(){try{if(document.fullscreenElement||document.webkitFullscreenElement){await (document.exitFullscreen?.()??document.webkitExitFullscreen?.());}else{const request=document.documentElement.requestFullscreen||document.documentElement.webkitRequestFullscreen;if(!request){const message='On iPhone: Share → Add to Home Screen, then open it for fullscreen.';$('screen-help').textContent=message;say(message,10);return;}await request.call(document.documentElement);}}catch(e){say('Fullscreen unavailable here. Open the game directly in your browser.',6);}}
$('fullscreen').onclick=fullscreen;
window.addEventListener('keydown',e=>{if(['Space','ArrowUp','ArrowDown','ArrowLeft','ArrowRight'].includes(e.code))e.preventDefault();if(!running)return;keys.add(e.code);if(e.repeat)return;if(e.code==='Space')jumpQueued=true;if(e.code==='ShiftLeft'||e.code==='ShiftRight')dashQueued=true;if(e.code==='KeyE')shift();});window.addEventListener('keyup',e=>keys.delete(e.code));window.addEventListener('blur',()=>{keys.clear();stick.x=stick.y=0;});document.addEventListener('visibilitychange',()=>{if(document.hidden&&running)pause();});
function look(dx,dy){yaw-=dx*.004;pitch=Math.max(-1.25,Math.min(1.25,pitch-dy*.004));}document.addEventListener('mousemove',e=>{if(running&&document.pointerLockElement===canvas)look(e.movementX,e.movementY);});
let lookPointer=null,lookX=0,lookY=0;canvas.addEventListener('pointerdown',e=>{if(!running)return;if(e.pointerType==='mouse'){canvas.requestPointerLock?.();return;}if(lookPointer!==null)return;lookPointer=e.pointerId;lookX=e.clientX;lookY=e.clientY;canvas.setPointerCapture(e.pointerId);});canvas.addEventListener('pointermove',e=>{if(e.pointerId===lookPointer){look(e.clientX-lookX,e.clientY-lookY);lookX=e.clientX;lookY=e.clientY;}});function endLook(e){if(e.pointerId===lookPointer)lookPointer=null;}canvas.addEventListener('pointerup',endLook);canvas.addEventListener('pointercancel',endLook);
let stickPointer=null;function moveStick(e){const r=$('stick').getBoundingClientRect(),radius=r.width*.36,dx=e.clientX-r.left-r.width/2,dy=e.clientY-r.top-r.height/2,length=Math.max(radius,Math.hypot(dx,dy));stick.x=dx/length;stick.y=dy/length;$('thumb').style.transform='translate('+stick.x*radius+'px,'+stick.y*radius+'px)';}
$('stick').addEventListener('pointerdown',e=>{if(stickPointer!==null)return;stickPointer=e.pointerId;$('stick').setPointerCapture(e.pointerId);moveStick(e);});$('stick').addEventListener('pointermove',e=>{if(e.pointerId===stickPointer)moveStick(e);});function endStick(e){if(e.pointerId!==stickPointer)return;stickPointer=null;stick.x=stick.y=0;$('thumb').style.transform='';}$('stick').addEventListener('pointerup',endStick);$('stick').addEventListener('pointercancel',endStick);
window.addEventListener('resize',()=>{camera.aspect=innerWidth/innerHeight;camera.updateProjectionMatrix();renderer.setSize(innerWidth,innerHeight);});canvas.addEventListener('webglcontextlost',e=>{e.preventDefault();pause();$('start').disabled=true;$('start').textContent='Graphics interrupted — reload to continue';});
generate();requestAnimationFrame(animate);
window.afterlightState=()=>({seed,phase,running,won,falls,grounded,yaw,pitch,player:{...player},anchors:anchors.map(a=>a.charged),platforms:platforms.map(p=>({x:p.x,y:p.y,z:p.z,type:p.type})),drawCalls:renderer.info.render.calls});
