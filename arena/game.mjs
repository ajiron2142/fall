import * as THREE from '../vendor/three.module.js';

const $=id=>document.getElementById(id),canvas=$('view');
let core,data,ready=false,running=false,weapon=1,slow=false,assist=matchMedia('(pointer:coarse)').matches,sound=true,touchControls=false;
let yaw=0,pitch=-.14,jumpQueued=false,dashQueued=false,fireHeld=false,firePointer=null,stickPointer=null,lookPointer=null,lookX=0,lookY=0;
let batPending=false,landing=0,lastGrounded=true,lastVy=0;
let swingAge=10,recoil=0,hitAge=10,hitstop=0,noticeUntil=0,elapsed=0,last=performance.now(),lastHoops=0,seenHits=0,fireLookX=0,fireLookY=0;
const keys=new Set(),stick={x:0,z:0},trails=[],waves=[],particles=[],stats={bat:0,gun:0};
let fpsTime=0,fpsFrames=0,courseActive=false,runwayActive=false,lastCourseProgress=0;
const presetNames=['Snappy','Momentum','Floaty'];
let playground=0;
const playgroundNames=['Jump · Easy','Jump · Medium','Jump · Hard','Wall run · Easy','Wall run · Medium','Wall run · Hard','Combined · Expert'],playgroundTotals=[10,10,10,2,2,2,4];
let audio;
function activateAudio(){try{audio??=new (window.AudioContext||window.webkitAudioContext)();audio.resume().catch(()=>{});}catch{sound=false;}}
function tone(frequency,end,duration,type='sine',volume=.12){if(!sound||!audio)return;const osc=audio.createOscillator(),gain=audio.createGain(),now=audio.currentTime;osc.type=type;osc.frequency.setValueAtTime(frequency,now);osc.frequency.exponentialRampToValueAtTime(end,now+duration);gain.gain.setValueAtTime(volume,now);gain.gain.exponentialRampToValueAtTime(.001,now+duration);osc.connect(gain);gain.connect(audio.destination);osc.start();osc.stop(now+duration);}
function shotCrack(){if(!sound||!audio)return;const buffer=audio.createBuffer(1,Math.ceil(audio.sampleRate*.12),audio.sampleRate),samples=buffer.getChannelData(0);for(let i=0;i<samples.length;i++)samples[i]=(Math.random()*2-1)*Math.pow(1-i/samples.length,3);const source=audio.createBufferSource(),gain=audio.createGain(),filter=audio.createBiquadFilter();source.buffer=buffer;filter.type='lowpass';filter.frequency.value=2300;gain.gain.value=.25;source.connect(filter);filter.connect(gain);gain.connect(audio.destination);source.start();}
function notify(text,seconds=4){$('notice').textContent=text;noticeUntil=elapsed+seconds;}
function read(){data=new Float32Array(core.memory.buffer,core.snapshot_ptr(),core.snapshot_len());return data;}
let renderer;
try{renderer=new THREE.WebGLRenderer({canvas,antialias:true,powerPreference:'high-performance'});}catch(error){$('start').textContent='WebGL is unavailable in this browser';$('notice').textContent='Try opening the demo directly in Chrome.';throw error;}
renderer.setPixelRatio(Math.min(devicePixelRatio,1.6));renderer.setSize(innerWidth,innerHeight);renderer.shadowMap.enabled=true;renderer.shadowMap.type=THREE.PCFSoftShadowMap;renderer.toneMapping=THREE.ACESFilmicToneMapping;renderer.toneMappingExposure=1.25;
const scene=new THREE.Scene();scene.background=new THREE.Color(0x101b2d);scene.fog=new THREE.Fog(0x34485f,110,420);
const skyCanvas=document.createElement('canvas');skyCanvas.width=512;skyCanvas.height=256;const sky=skyCanvas.getContext('2d'),skyGradient=sky.createLinearGradient(0,0,0,256);skyGradient.addColorStop(0,'#162640');skyGradient.addColorStop(.45,'#546a8e');skyGradient.addColorStop(.55,'#bc8c8a');skyGradient.addColorStop(.7,'#101b2d');skyGradient.addColorStop(1,'#101b2d');sky.fillStyle=skyGradient;sky.fillRect(0,0,512,256);for(let i=0;i<140;i++){sky.globalAlpha=.15+(i%7)*.09;sky.fillStyle='#c8dfff';sky.fillRect((i*137)%512,(i*47)%108,1,1);}sky.globalAlpha=1;const skyTexture=new THREE.CanvasTexture(skyCanvas);skyTexture.mapping=THREE.EquirectangularReflectionMapping;skyTexture.colorSpace=THREE.SRGBColorSpace;scene.background=skyTexture;const pmrem=new THREE.PMREMGenerator(renderer);const environment=pmrem.fromEquirectangular(skyTexture);scene.environment=environment.texture;pmrem.dispose();
const camera=new THREE.PerspectiveCamera(88,innerWidth/innerHeight,.06,600);camera.rotation.order='YXZ';scene.add(camera);
scene.add(new THREE.HemisphereLight(0xffead8,0x456584,2.2));const sun=new THREE.DirectionalLight(0xffd5a0,3.5);sun.position.set(-28,32,22);sun.castShadow=true;sun.shadow.mapSize.set(1024,1024);sun.shadow.camera.left=-40;sun.shadow.camera.right=40;sun.shadow.camera.top=40;sun.shadow.camera.bottom=-40;sun.shadow.bias=-.001;sun.shadow.normalBias=.035;scene.add(sun);const rim=new THREE.DirectionalLight(0x78c9ff,1.1);rim.position.set(30,16,-24);scene.add(rim);
const geo={box:new THREE.BoxGeometry(1,1,1),sphere:new THREE.SphereGeometry(1,20,14),cylinder:new THREE.CylinderGeometry(1,1,1,16),ring:new THREE.TorusGeometry(1,.075,8,48)};
function mat(color,extra={}){return new THREE.MeshStandardMaterial({color,roughness:.65,...extra});}
function make(type,material,x,y,z,sx=1,sy=1,sz=1,parent=scene){const m=new THREE.Mesh(geo[type],material);m.position.set(x,y,z);m.scale.set(sx,sy,sz);m.castShadow=true;m.receiveShadow=true;parent.add(m);return m;}
const stone=mat(0x263649,{metalness:.25}),lightStone=mat(0x526579,{metalness:.25}),dark=mat(0x101a27),gold=mat(0xffa443,{emissive:0xff7020,emissiveIntensity:1.1});
const floorTexture=(()=>{const c=document.createElement('canvas');c.width=c.height=256;const ctx=c.getContext('2d');ctx.fillStyle='#253445';ctx.fillRect(0,0,256,256);ctx.strokeStyle='#101b29';ctx.lineWidth=5;ctx.strokeRect(3,3,250,250);ctx.strokeStyle='#43596a';ctx.lineWidth=1;ctx.strokeRect(8,8,240,240);for(let i=0;i<12;i++){ctx.fillStyle=i%2?'#29394a':'#2d3e50';ctx.fillRect(16,18+i*18,224,2);}for(const x of [15,241])for(const y of [15,241]){ctx.fillStyle='#7b8a93';ctx.fillRect(x-2,y-2,4,4);}ctx.fillStyle='#f5a546';ctx.fillRect(12,118,5,20);const t=new THREE.CanvasTexture(c);t.wrapS=t.wrapT=THREE.RepeatWrapping;t.repeat.set(80,160);t.anisotropy=Math.min(8,renderer.capabilities.getMaxAnisotropy());t.colorSpace=THREE.SRGBColorSpace;return t;})();
make('box',mat(0xffffff,{map:floorTexture}),0,-.3,0,160,.6,320);
for(const x of [-80,80]){make('box',stone,x,.6,0,.5,1.2,320);make('box',gold,x,1.23,0,.52,.06,320);}
for(const z of [-160,160]){make('box',stone,0,.6,z,160,1.2,.5);make('box',gold,0,1.23,z,160,.06,.52);}
for(const x of [-12,0,12]){const ring=make('ring',gold,x,2.6,-15.7,1.8,1.8,1.8);make('cylinder',stone,x,1.15,-15.7,.12,2.3,.12);make('box',lightStone,x,.12,-15.7,2,.24,1.3);ring.castShadow=false;}
function skylineRadius(a,i){return Math.min(80/Math.max(.001,Math.abs(Math.sin(a))),160/Math.max(.001,Math.abs(Math.cos(a))))+24+(i%4)*4;}
const outside=new THREE.InstancedMesh(geo.box,mat(0x223249),32),transform=new THREE.Object3D();
for(let i=0;i<32;i++){const a=i*Math.PI/16,r=skylineRadius(a,i),h=4+(i*7%15);transform.position.set(Math.sin(a)*r,h/2-2,Math.cos(a)*r);transform.scale.set(2+(i%3),h,2+(i%4));transform.updateMatrix();outside.setMatrixAt(i,transform.matrix);}scene.add(outside);
const cyan=mat(0x65ecff,{emissive:0x30c9ff,emissiveIntensity:1.8,metalness:.4});
for(const x of [-78.8,78.8])make('box',cyan,x,.025,0,.06,.04,318);
for(const z of [-158.8,158.8])make('box',cyan,0,.025,z,158,.04,.06);
// Instanced service lights keep the skyline detailed without a mesh per fixture.
const lights=new THREE.InstancedMesh(geo.box,cyan,96);
for(let i=0;i<96;i++){const n=Math.floor(i/3),a=n*Math.PI/16,r=skylineRadius(a,n),h=4+(n*7%15);transform.position.set(Math.sin(a)*r,h*.35+(i%3)*.8,Math.cos(a)*r+1.6);transform.scale.set(1.3,.08,.08);transform.updateMatrix();lights.setMatrixAt(i,transform.matrix);}scene.add(lights);
for(const x of [-12,0,12]){const inner=make('ring',cyan,x,2.6,-15.68,1.66,1.66,1.66);inner.castShadow=false;}
// A 280-metre clear runway makes the new scale visible and useful for movement tests.
for(const x of [-62,-38])make('box',cyan,x,.015,0,.08,.025,280);
const runwayMarks=new THREE.InstancedMesh(geo.box,new THREE.MeshBasicMaterial({color:0xa7c7da}),30);
for(let i=0;i<30;i++){transform.position.set(i%2?-39:-61,.015,140-Math.floor(i/2)*20);transform.scale.set(2,.025,.2);transform.updateMatrix();runwayMarks.setMatrixAt(i,transform.matrix);}scene.add(runwayMarks);
for(let distance=20;distance<=280;distance+=20){const c=document.createElement('canvas');c.width=256;c.height=128;const ctx=c.getContext('2d');ctx.fillStyle='#bbd9e8';ctx.font='bold 65px sans-serif';ctx.textAlign='center';ctx.fillText(distance+'m',128,90);const texture=new THREE.CanvasTexture(c);texture.colorSpace=THREE.SRGBColorSpace;const label=new THREE.Mesh(new THREE.PlaneGeometry(4,2),new THREE.MeshBasicMaterial({map:texture,transparent:true,depthWrite:false}));label.rotation.x=-Math.PI/2;label.position.set(-50,.025,140-distance);scene.add(label);}
for(const x of [-55,-45])make('box',gold,x,4,-145,.25,8,.3);make('box',gold,-50,8,-145,10,.25,.3);
const bodies=[],bodyMats=[],rings=[];
const streaks=new THREE.InstancedMesh(geo.box,new THREE.MeshBasicMaterial({color:0xffc980,transparent:true,opacity:.45,depthWrite:false}),10),zAxis=new THREE.Vector3(0,0,1),velocity=new THREE.Vector3();streaks.frustumCulled=false;scene.add(streaks);
const flyingPellets=new THREE.InstancedMesh(geo.box,new THREE.MeshBasicMaterial({color:0xffefaa}),72);flyingPellets.frustumCulled=false;scene.add(flyingPellets);
const weaponRoot=new THREE.Group();camera.add(weaponRoot);
const bat=new THREE.Group();weaponRoot.add(bat);
const wood=mat(0xc6d0db,{metalness:.85,roughness:.25}),wrap=mat(0x172334);
const maple=mat(0xd8a16b,{roughness:.4,metalness:.05}),batGrip=mat(0x223039,{roughness:.95}),gloveLeather=mat(0x354957,{roughness:.8});
const batProfile=[[0,-.045],[.036,-.042],[.046,-.025],[.046,-.008],[.03,.008],[.026,.05],[.025,.23],[.03,.33],[.045,.43],[.061,.53],[.075,.64],[.081,.82],[.08,1.01],[.076,1.06],[.062,1.09],[.035,1.11],[0,1.12]].map(([r,y])=>new THREE.Vector2(r,y));
const batBody=new THREE.Mesh(new THREE.LatheGeometry(batProfile,24),maple);bat.add(batBody);
for(let i=0;i<9;i++){const tape=make('cylinder',batGrip,0,.035+i*.023,0,.028,.022,.028,bat);tape.rotation.y=i*.45;}
make('cylinder',gold,0,.31,0,.032,.024,.032,bat);make('cylinder',wrap,0,.9,0,.082,.075,.082,bat);
const emblem=make('box',gold,.002,.9,.083,.04,.04,.006,bat);emblem.rotation.z=Math.PI/4;
for(const y of [.065,.195]){const hand=new THREE.Group();hand.position.set(.014,y,.045);bat.add(hand);make('sphere',gloveLeather,0,0,0,.063,.075,.073,hand);make('box',batGrip,.008,.012,.056,.082,.06,.018,hand);for(let i=0;i<3;i++)make('box',lightStone,-.022+i*.021,.012,.069,.009,.032,.008,hand);make('sphere',gloveLeather,-.038,-.036,-.018,.035,.049,.04,hand);}
const batArms=new THREE.Group();weaponRoot.add(batArms);const batSleeves=[];
for(let i=0;i<2;i++){const sleeve=new THREE.Mesh(new THREE.CylinderGeometry(.052,.072,1,12),wrap);batArms.add(sleeve);batSleeves.push(sleeve);}
const gun=new THREE.Group();weaponRoot.add(gun);
make('box',dark,0,0,0,.23,.22,.42,gun);make('box',stone,0,.065,-.05,.24,.1,.32,gun);
for(const y of [.055,-.065]){const barrel=make('cylinder',wood,0,y,-.4,.058,.7,.058,gun);barrel.rotation.x=Math.PI/2;make('ring',dark,0,y,-.755,.058,.058,.058,gun);}
make('box',wrap,0,-.17,.13,.14,.27,.18,gun).rotation.x=-.25;make('box',gold,.122,.01,.02,.015,.065,.1,gun);
const pump=new THREE.Group();gun.add(pump);make('box',wrap,0,-.085,-.33,.23,.16,.26,pump);for(let i=0;i<5;i++)make('box',stone,0,-.085,-.44+i*.045,.24,.17,.012,pump);
make('box',gold,0,.125,-.67,.035,.035,.04,gun);make('box',dark,0,-.04,.34,.2,.21,.3,gun);
const muzzle=make('sphere',mat(0xffd28d,{emissive:0xffbe64,emissiveIntensity:3}),0,.02,-.83,.12,.12,.2,gun);muzzle.visible=false;
weaponRoot.traverse(o=>{if(o.isMesh){o.castShadow=false;o.receiveShadow=false;o.material=o.material.clone();o.material.depthTest=false;o.renderOrder=10;}});
const handLight=new THREE.PointLight(0xe0eeff,3,4);handLight.position.set(-.4,.4,.1);camera.add(handLight);
const sparkPositions=new Float32Array(160*3),sparkGeo=new THREE.BufferGeometry();sparkGeo.setAttribute('position',new THREE.BufferAttribute(sparkPositions,3));const sparks=new THREE.Points(sparkGeo,new THREE.PointsMaterial({color:0xffdeaa,size:.075,transparent:true,opacity:.9,depthWrite:false}));sparks.frustumCulled=false;scene.add(sparks);
function burst(point,count=18){for(let i=0;i<count;i++){if(particles.length>=160)particles.shift();particles.push({p:point.clone(),v:new THREE.Vector3((Math.random()-.5)*7,Math.random()*5,(Math.random()-.5)*7),life:.3+Math.random()*.25});}}
function impactWave(point){const mesh=new THREE.Mesh(geo.ring,new THREE.MeshBasicMaterial({color:0xffcf7c,transparent:true,opacity:.9,depthWrite:false}));mesh.position.copy(point);mesh.quaternion.copy(camera.quaternion);scene.add(mesh);waves.push({mesh,life:.22});tone(85,30,.18,'sine',.3);}
// Grip and barrel targets trace a forward arc around the shoulders.
// Orient the bat toward its moving tip, rather than rolling a vertical cylinder.
const BAT_CONTACT=.12,BAT_CYCLE=.42;
const batPoses=[
 [0,.38,-.45,-.68,.62,.48,-1.05],
 [.065,.57,-.42,-.48,.91,.48,-.33],
 [.12,.08,-.34,-.99,-.12,.05,-2.08],
 [.20,-.37,-.39,-.87,-1.13,-.04,-1.43],
 [.28,-.28,-.46,-.73,-.67,.34,-1.25],
 [.42,.38,-.45,-.68,.62,.48,-1.05]
];
const batUp=new THREE.Vector3(0,1,0),batAim=new THREE.Vector3();
function sampleBat(age){let a=batPoses[0],b=a;if(age<BAT_CYCLE){for(let i=1;i<batPoses.length;i++)if(age<=batPoses[i][0]){a=batPoses[i-1];b=batPoses[i];break;}}const t=a===b?0:THREE.MathUtils.smoothstep(age,a[0],b[0]);return a.map((n,i)=>THREE.MathUtils.lerp(n,b[i],t));}
const batTrailPositions=new Float32Array(8*6*3),batTrailGeometry=new THREE.BufferGeometry();batTrailGeometry.setAttribute('position',new THREE.BufferAttribute(batTrailPositions,3));
const batTrail=new THREE.Mesh(batTrailGeometry,new THREE.MeshBasicMaterial({color:0xffcc86,transparent:true,opacity:.15,side:THREE.DoubleSide,depthTest:false,depthWrite:false}));batTrail.renderOrder=9;batTrail.frustumCulled=false;weaponRoot.add(batTrail);
function poseBat(narrow){const v=sampleBat(swingAge),scale=narrow?.8:1;bat.scale.setScalar(scale);bat.position.set(v[1]*(narrow?.55:1)-(narrow?.04:0),v[2],v[3]-(narrow?.18:0));batAim.set(v[4]-v[1],v[5]-v[2],v[6]-v[3]).normalize();bat.quaternion.setFromUnitVectors(batUp,batAim);
 bat.updateMatrix();batArms.visible=weapon===0;
 for(let i=0;i<2;i++){const hand=new THREE.Vector3(.014,i?.195:.065,.045).applyMatrix4(bat.matrix),anchor=new THREE.Vector3(i?-.18:.38,-1.0,-.24),axis=hand.clone().sub(anchor),arm=batSleeves[i];arm.position.copy(anchor).addScaledVector(axis,.5);arm.quaternion.setFromUnitVectors(batUp,axis.clone().normalize());arm.scale.set(1,axis.length(),1);}

 batTrail.visible=running&&weapon===0&&swingAge>.065&&swingAge<.23;
 if(batTrail.visible){let offset=0;for(let i=0;i<8;i++){const vertices=[];for(const t of [i/8,(i+1)/8]){const p=sampleBat(Math.max(.065,swingAge-.05+t*.05)),grip=new THREE.Vector3(p[1]*(narrow?.55:1)-(narrow?.04:0),p[2],p[3]-(narrow?.18:0)),aim=new THREE.Vector3(p[4]-p[1],p[5]-p[2],p[6]-p[3]).normalize();vertices.push(grip.clone().addScaledVector(aim,.65*scale),grip.clone().addScaledVector(aim,1.10*scale));}for(const index of [0,1,2,2,1,3]){const p=vertices[index];batTrailPositions[offset++]=p.x;batTrailPositions[offset++]=p.y;batTrailPositions[offset++]=p.z;}}batTrailGeometry.attributes.position.needsUpdate=true;}
}
function direction(a=yaw,b=pitch){return new THREE.Vector3(-Math.sin(a)*Math.cos(b),Math.sin(b),-Math.cos(a)*Math.cos(b));}
function assistedAim(){
 if(!assist)return {yaw,pitch};const eye=new THREE.Vector3(data[0],data[1]+1.65,data[2]),forward=direction();let best=.95,chosen=null;
 for(let i=0;i<10;i++){const o=16+i*9,target=new THREE.Vector3(data[o],data[o+1],data[o+2]).sub(eye);const distance=target.length();if(distance>(weapon===0?4.2:9))continue;const dot=target.normalize().dot(forward);if(dot>best){best=dot;chosen=target;}}
 return chosen?{yaw:Math.atan2(-chosen.x,-chosen.z),pitch:Math.asin(chosen.y)}:{yaw,pitch};
}
function fire(){
 if(!ready||!running)return;
 if(weapon===0){if(batPending||swingAge<BAT_CYCLE)return;read();if(data[13]>0)return;batPending=true;swingAge=0;tone(350,80,.12,'triangle',.09);return;}
 strike();
}
function strike(){
 if(!ready||!running)return;read();const aim=assistedAim(),result=core.attack(aim.yaw,aim.pitch,weapon);if(result===-2)return;
 if(weapon===1){swingAge=0;shotCrack();}recoil=weapon===1||result>0?1:0;tone(weapon===0?210:90,weapon===0?65:30,weapon===0?.14:.2,weapon===0?'triangle':'sine',weapon===0?.12:.3);
}
function impactFeedback(count,id,source){
 stats[source===0?'bat':'gun']+=count;hitAge=0;hitstop=source===0?.045:0;
 if(id>=0){const o=16+id*9;const point=new THREE.Vector3(data[o],data[o+1],data[o+2]);burst(point,32);impactWave(point);}
 tone(source===0?430:700,120,.08,'square',.045);navigator.vibrate?.(source===0?18:24);notify(source===0?'CRACK!':'BOOM! Close range, big impact.',2);
}
function clearInput(){batPending=false;swingAge=10;keys.clear();stick.x=stick.z=0;fireHeld=false;firePointer=stickPointer=lookPointer=null;jumpQueued=dashQueued=false;$('thumb').style.transform='';}
function updateTouchControls(){ $('controls').hidden=!touchControls;$('touch-controls').textContent='Touch controls: '+(touchControls?'on':'off');$('touch-controls').setAttribute('aria-pressed',String(touchControls));}
try{touchControls=localStorage.getItem('impact-touch-controls')==='true';}catch{}
updateTouchControls();
$('touch-controls').onclick=()=>{clearInput();touchControls=!touchControls;updateTouchControls();try{localStorage.setItem('impact-touch-controls',String(touchControls));}catch{}};
function switchWeapon(){weapon=1-weapon;batPending=false;swingAge=10;fireHeld=false;updateWeapon();notify(weapon===0?'BAT · swing through. Your speed adds power.':'SHOTGUN · close the distance. Nine pellets, heavy punch.',3);}
function updateWeapon(){bat.visible=weapon===0;gun.visible=weapon===1;$('weapon').innerHTML=weapon===0?'01 / BAT <span>⇄ SHOTGUN</span>':'02 / SHOTGUN <span>⇄ BAT</span>';$('fire').innerHTML=weapon===0?'SWING<span>hold to attack</span>':'FIRE<span>hold to attack</span>';$('weapon').style.background=weapon===0?'#e3a373':'#9bdcc6';}
function toggleSlow(){if(!running)return;slow=!slow;document.body.classList.toggle('slow',slow);$('slow').classList.toggle('active',slow);notify(slow?'BULLET TIME · you, targets, and pellets at 8% speed. Aim, fire, then resume.':'Back to full speed.',3);tone(slow?400:160,slow?100:440,.18,'sine',.06);}
function pause(){running=false;clearInput();slow=false;document.body.classList.remove('slow');$('slow').classList.remove('active');document.exitPointerLock?.();$('overlay').classList.remove('hidden');$('start').textContent='Back to the arena ↗';$('capability').textContent=`This session: ${stats.bat} bat hits · ${stats.gun} shotgun hits. Which feels better?`;}
function enter(){if(!ready)return;activateAudio();running=true;fpsTime=fpsFrames=0;$('overlay').classList.add('hidden');last=performance.now();if(matchMedia('(pointer:fine)').matches)canvas.requestPointerLock?.();notify(courseActive?'COURSE · follow the numbers. R / Course restarts the route.':weapon===0?'The orange marble ahead is in bat range. Try a swing.':'Shotgun ready. Q / mouse wheel switches weapons. Course starts the jump route.',5);}
function reset(){if(!ready)return;clearInput();weapon=1;updateWeapon();courseActive=false;runwayActive=false;lastCourseProgress=0;landing=0;lastGrounded=true;lastVy=0;core.reset(crypto.getRandomValues(new Uint32Array(1))[0]);yaw=0;pitch=-.14;lastHoops=0;seenHits=0;stats.bat=stats.gun=0;read();notify('Fresh arena. Your first marble is just ahead.',4);}
function goCourse(){if(!ready)return;clearInput();slow=false;document.body.classList.remove('slow');$('slow').classList.remove('active');core.start_playground(playground);courseActive=true;runwayActive=false;lastCourseProgress=1;landing=0;lastGrounded=true;lastVy=0;const route=Array.from({length:core.obstacle_count()},(_,i)=>i).filter(i=>core.obstacle(i,6)===playground&&!core.obstacle(i,7));yaw=playground>=3&&playground<=5?0:-Math.atan2(core.obstacle(route[1],0)-core.obstacle(route[0],0),core.obstacle(route[0],1)-core.obstacle(route[1],1));pitch=-.1;read();notify(playgroundNames[playground]+' · '+(playground<3?'Climb higher. Time your second jump.':playground<6?'Run off the ledge alongside the glowing wall. No jumping needed.':'Jump to the walls, run along them, then jump away to land higher.'),7);}
function choosePlayground(index){playground=index;for(let i=0;i<7;i++)$('playground-'+i).setAttribute('aria-pressed',String(i===index));goCourse();}
function goRunway(){if(!ready)return;clearInput();slow=false;document.body.classList.remove('slow');$('slow').classList.remove('active');core.start_runway();courseActive=false;runwayActive=true;landing=0;lastGrounded=true;lastVy=0;yaw=0;pitch=-.08;read();notify('RUNWAY · 24 m/s cruise. Dash once for 30 m/s, then hold your direction to keep it.',6);}
function choosePreset(index){if(!ready)return;core.set_movement_preset(index);try{localStorage.setItem('impact-movement-preset',String(index));}catch{}updatePreset();goCourse();}
function updatePreset(){const index=core.movement_preset();for(let i=0;i<3;i++)$('preset-'+i).setAttribute('aria-pressed',String(i===index));$('feel').textContent=presetNames[index].toUpperCase();}
async function fullscreen(){try{if(document.fullscreenElement)await document.exitFullscreen();else if(document.documentElement.requestFullscreen)await document.documentElement.requestFullscreen();else notify('Use Share → Add to Home Screen for fullscreen.',6);}catch{notify('Open directly in your browser to enable fullscreen.',6);}}
for(let i=0;i<7;i++)$('playground-'+i).onclick=()=>choosePlayground(i);
$('runway').onclick=goRunway;$('course').onclick=goCourse;for(let i=0;i<3;i++)$('preset-'+i).onclick=()=>choosePreset(i);
$('start').onclick=enter;$('pause').onclick=pause;$('fullscreen').onclick=fullscreen;$('weapon').onclick=switchWeapon;$('reset').onclick=reset;
$('assist').onclick=()=>{assist=!assist;$('assist').textContent='Aim assist: '+(assist?'on':'off');};$('assist').textContent='Aim assist: '+(assist?'on':'off');$('sound').onclick=()=>{sound=!sound;$('sound').textContent='Sound: '+(sound?'on':'off');if(sound)activateAudio();};
function actionButton(id,action){$(id).addEventListener('pointerdown',e=>{e.preventDefault();action();});$(id).addEventListener('click',e=>{if(e.detail===0)action();});}
actionButton('jump',()=>{if(running)jumpQueued=true;});actionButton('dash',()=>{if(running)dashQueued=true;});actionButton('slow',toggleSlow);
$('fire').addEventListener('pointerdown',e=>{e.preventDefault();if(!running)return;firePointer=e.pointerId;fireLookX=e.clientX;fireLookY=e.clientY;$('fire').setPointerCapture(e.pointerId);fireHeld=true;fire();});$('fire').addEventListener('pointermove',e=>{if(e.pointerId===firePointer&&e.pointerType!=='mouse'){look(e.clientX-fireLookX,e.clientY-fireLookY,.004);fireLookX=e.clientX;fireLookY=e.clientY;}});for(const event of ['pointerup','pointercancel','lostpointercapture'])$('fire').addEventListener(event,e=>{if(e.pointerId===firePointer){firePointer=null;fireHeld=false;}});$('fire').addEventListener('click',e=>{if(e.detail===0)fire();});
function moveStick(e){const r=$('stick').getBoundingClientRect(),radius=r.width*.36,dx=e.clientX-r.left-r.width/2,dz=e.clientY-r.top-r.height/2,n=Math.max(radius,Math.hypot(dx,dz));stick.x=dx/n;stick.z=dz/n;$('thumb').style.transform=`translate(${stick.x*radius}px,${stick.z*radius}px)`;}
$('stick').addEventListener('pointerdown',e=>{if(!running||stickPointer!==null)return;stickPointer=e.pointerId;$('stick').setPointerCapture(e.pointerId);moveStick(e);});$('stick').addEventListener('pointermove',e=>{if(e.pointerId===stickPointer)moveStick(e);});for(const event of ['pointerup','pointercancel','lostpointercapture'])$('stick').addEventListener(event,e=>{if(e.pointerId===stickPointer){stickPointer=null;stick.x=stick.z=0;$('thumb').style.transform='';}});
function look(dx,dy,sensitivity){yaw-=dx*sensitivity;pitch=Math.max(-1.3,Math.min(1.3,pitch-dy*sensitivity));}
document.addEventListener('mousemove',e=>{if(running&&document.pointerLockElement===canvas)look(e.movementX,e.movementY,.0023);});
canvas.addEventListener('pointerdown',e=>{if(!running)return;if(e.pointerType==='mouse'){if(document.pointerLockElement!==canvas)canvas.requestPointerLock?.();fireHeld=true;fire();return;}if(!touchControls||lookPointer!==null)return;lookPointer=e.pointerId;lookX=e.clientX;lookY=e.clientY;canvas.setPointerCapture(e.pointerId);});canvas.addEventListener('pointermove',e=>{if(e.pointerId===lookPointer){look(e.clientX-lookX,e.clientY-lookY,.004);lookX=e.clientX;lookY=e.clientY;}});for(const event of ['pointerup','pointercancel'])canvas.addEventListener(event,e=>{if(e.pointerId===lookPointer)lookPointer=null;if(e.pointerType==='mouse')fireHeld=false;});document.addEventListener('mouseup',()=>{if(firePointer===null)fireHeld=false;});canvas.addEventListener('contextmenu',e=>e.preventDefault());
let lastWheelSwitch=-Infinity;
window.addEventListener('wheel',e=>{if(!running||e.ctrlKey||Math.abs(e.deltaY)<1)return;e.preventDefault();const now=performance.now();if(now-lastWheelSwitch<220)return;lastWheelSwitch=now;switchWeapon();},{passive:false});
document.addEventListener('pointerlockchange',()=>{if(running&&matchMedia('(pointer:fine)').matches&&document.pointerLockElement!==canvas)pause();});
window.addEventListener('keydown',e=>{if(!running)return;if(e.code==='Escape'){pause();return;}if(['Space','ArrowUp','ArrowDown','ArrowLeft','ArrowRight'].includes(e.code))e.preventDefault();keys.add(e.code);if(e.repeat)return;if(e.code==='Space')jumpQueued=true;if(e.code==='ShiftLeft'||e.code==='ShiftRight')dashQueued=true;if(e.code==='KeyQ')switchWeapon();if(e.code==='KeyR')goCourse();if(e.code==='KeyE')toggleSlow();if(e.code==='KeyF')fullscreen();});window.addEventListener('keyup',e=>keys.delete(e.code));window.addEventListener('blur',clearInput);document.addEventListener('visibilitychange',()=>{if(document.hidden&&running)pause();});
window.addEventListener('resize',()=>{camera.aspect=innerWidth/innerHeight;camera.updateProjectionMatrix();renderer.setSize(innerWidth,innerHeight);});canvas.addEventListener('webglcontextlost',e=>{e.preventDefault();pause();$('start').disabled=true;$('start').textContent='Graphics interrupted — reload to continue';});
function frame(now){requestAnimationFrame(frame);const frameTime=Math.max(0,(now-last)/1000);let dt=Math.min(frameTime,.04);last=now;if(running){fpsTime+=frameTime;fpsFrames++;if(fpsTime>=.5){$('fps').textContent=Math.round(fpsFrames/fpsTime)+' FPS';fpsTime=0;fpsFrames=0;}}elapsed+=dt;const worldDt=dt*(slow?.08:1);swingAge+=worldDt;recoil=Math.max(0,recoil-worldDt*(weapon===1?5:7));hitAge+=worldDt;hitstop=Math.max(0,hitstop-worldDt);landing=THREE.MathUtils.damp(landing,0,12,dt);
 if(batPending&&running&&swingAge>=BAT_CONTACT){batPending=false;strike();}
 if(ready){
  if(running){const forward=(keys.has('KeyW')||keys.has('ArrowUp')?1:0)-(keys.has('KeyS')||keys.has('ArrowDown')?1:0)-stick.z,side=(keys.has('KeyD')||keys.has('ArrowRight')?1:0)-(keys.has('KeyA')||keys.has('ArrowLeft')?1:0)+stick.x;let remain=dt*(hitstop>0?.15:1);while(remain>0){const step=Math.min(remain,1/120);core.tick(step,forward,side,yaw,jumpQueued?1:0,dashQueued?1:0,slow?1:0);jumpQueued=dashQueued=false;remain-=step;}if(fireHeld)fire();}
  read();if(data[9]>seenHits){impactFeedback(data[9]-seenHits,core.last_hit(),core.last_impact_weapon());seenHits=data[9];}
  for(let i=0;i<10;i++){const o=16+i*9;bodies[i].position.set(data[o],data[o+1],data[o+2]);velocity.set(data[o+6],data[o+7],data[o+8]);const trailLength=Math.max(0,velocity.length()-8)*.035;transform.position.copy(bodies[i].position);if(trailLength>0){velocity.normalize();transform.position.addScaledVector(velocity,-data[o+3]-trailLength*.5);transform.quaternion.setFromUnitVectors(zAxis,velocity);}transform.scale.set(.035,.035,trailLength);transform.updateMatrix();streaks.setMatrixAt(i,transform.matrix);bodyMats[i].emissive.setHex(data[o+5]>.01?0xf5b674:i<6?0x000000:0x3e9a9d);bodyMats[i].emissiveIntensity=data[o+5]*2.5+(i<6?.12:.6);if(i<6){bodies[i].rotation.x+=data[o+8]*worldDt/.6;bodies[i].rotation.z-=data[o+6]*worldDt/.6;}if(rings[i]){rings[i].rotation.x=data[14]*.4;rings[i].rotation.z=data[14]*.7;}}
  for(let i=0;i<core.projectile_count();i++){
   const active=core.projectile(i,0)>0;transform.scale.set(0,0,0);
   if(active){velocity.set(core.projectile(i,4),core.projectile(i,5),core.projectile(i,6));transform.position.set(core.projectile(i,1),core.projectile(i,2),core.projectile(i,3)).addScaledVector(velocity,-.08);transform.quaternion.setFromUnitVectors(zAxis,velocity);transform.scale.set(.045,.045,.2);}
   transform.updateMatrix();flyingPellets.setMatrixAt(i,transform.matrix);
  }
  flyingPellets.instanceMatrix.needsUpdate=true;
  streaks.instanceMatrix.needsUpdate=true;
  const speed=Math.hypot(data[3],data[5]);if(running){if(data[6]&&!lastGrounded)landing=Math.min(.16,Math.abs(lastVy)*.009);camera.position.set(data[0],data[1]+1.65-landing,data[2]);camera.rotation.set(pitch,yaw,-stick.x*.025,'YXZ');}else{camera.position.set(17,14,19);camera.lookAt(0,1,0);}
  lastGrounded=!!data[6];lastVy=data[4];
  weaponRoot.visible=running;const narrow=camera.aspect<.75;poseBat(narrow);gun.position.set(narrow?.15:.36,-.35,(narrow?-.84:-.66)+recoil*.19);gun.rotation.set(-recoil*.18,0,0);pump.position.z=swingAge>.12&&swingAge<.43?Math.sin((swingAge-.12)/.31*Math.PI)*.15:0;muzzle.visible=weapon===1&&swingAge<.05;
  $('score').textContent=Math.round(data[8]);$('score').style.transform=`scale(${1+Math.max(0,1-hitAge/.18)*.2})`;$('combo').textContent=data[12]>1?`CHAIN ×${Math.round(data[12])}`:'FREE PLAY';$('speed').textContent=speed.toFixed(1)+' m/s';const progress=core.course_progress();$('progress').textContent=core.wall_running()?'WALL RUN · SPACE TO KICK OFF':courseActive?playgroundNames[playground].toUpperCase()+' '+progress+'/'+playgroundTotals[playground]+' · '+data[1].toFixed(1)+'m HIGH':runwayActive?'RUNWAY · '+Math.max(0,140-data[2]).toFixed(0)+'/280m':'160 × 320m ARENA';if(courseActive&&progress>lastCourseProgress){lastCourseProgress=progress;notify(progress===playgroundTotals[playground]?'SECTION COMPLETE! Choose another playground in Pause.':'CHECKPOINT '+progress+'/'+playgroundTotals[playground],3);tone(440+progress*40,800,.1,'sine',.05);}$('hoops').textContent=Math.round(data[15])+' RING SHOTS';$('dash').style.opacity=data[7]>0?'.45':'1';$('jump').style.opacity=core.air_jump_available()?'1':'.6';$('hitmarker').style.opacity=hitAge<.17?'1':'0';
  if(data[15]>lastHoops){lastHoops=data[15];notify('RING SHOT! +75 · Try another angle.',4);tone(520,1100,.2,'sine',.13);}
  if(running&&elapsed>noticeUntil)$('notice').textContent=slow?'BULLET TIME · aim freely · E / TIME resumes':weapon===0?'Close the distance. Swing, dash, repeat.':'Aim, fire, keep moving.';
 }
 gold.emissiveIntensity=1.1+Math.max(0,1-hitAge/.25)*1.8;
 for(let i=waves.length-1;i>=0;i--){const w=waves[i];w.life-=worldDt;w.mesh.scale.setScalar(.3+(1-w.life/.22)*2.4);w.mesh.material.opacity=Math.max(0,w.life/.22);if(w.life<=0){scene.remove(w.mesh);w.mesh.material.dispose();waves.splice(i,1);}}
 for(let i=trails.length-1;i>=0;i--){const t=trails[i];t.life-=worldDt;t.line.material.opacity=Math.max(0,t.life/.07);if(t.life<=0){scene.remove(t.line);t.line.geometry.dispose();t.line.material.dispose();trails.splice(i,1);}}
 for(let i=particles.length-1;i>=0;i--){const p=particles[i];p.life-=worldDt;p.v.y-=15*worldDt;p.p.addScaledVector(p.v,worldDt);if(p.life<=0)particles.splice(i,1);}
 for(let i=0;i<160;i++){const p=particles[i]?.p;sparkPositions[i*3]=p?.x??0;sparkPositions[i*3+1]=p?.y??-100;sparkPositions[i*3+2]=p?.z??0;}sparkGeo.attributes.position.needsUpdate=true;
 renderer.render(scene,camera);
}
requestAnimationFrame(frame);
try{
 const response=await fetch(new URL('./core.wasm',import.meta.url));if(!response.ok)throw Error('Physics download returned '+response.status);const result=await WebAssembly.instantiate(await response.arrayBuffer(),{});core=result.instance.exports;core.reset(1);read();
 const coursePoints=Array.from({length:7},()=>[]),groupCounts=Array(7).fill(0);
 for(let i=0;i<core.obstacle_count();i++){
  const x=core.obstacle(i,0),z=core.obstacle(i,1),w=core.obstacle(i,2),d=core.obstacle(i,3),h=core.obstacle(i,4),base=core.obstacle(i,5),height=h-base;
  const group=core.obstacle(i,6),isWall=!!core.obstacle(i,7);
  make('box',isWall?stone:i>=5?lightStone:i%2?lightStone:stone,x,base+height/2,z,w,height,d);make('box',dark,x,h+.025,z,w+.04,.05,d+.04);
  for(const side of [-1,1]){make('box',cyan,x+side*(w/2-.07),base+height/2,z,.055,height-.12,d+.035);make('box',gold,x,h-.15,z+side*(d/2+.025),w-.2,.06,.045);}
  if(i<5){for(let n=0;n<5;n++)make('box',dark,x-w*.34+n*w*.17,h*.48,z+d/2+.035,w*.08,h*.34,.035);}
  else if(isWall){
   for(let n=0;n<3;n++)make('box',cyan,x-w/2-.025,2+n*2,z,.06,.09,d-.3);
  }else{
   coursePoints[group].push(new THREE.Vector3(x,h+.4,z));groupCounts[group]++;
   const label=document.createElement('canvas');label.width=label.height=128;const ctx=label.getContext('2d');ctx.fillStyle='#d7eeff';ctx.font='bold 80px sans-serif';ctx.textAlign='center';ctx.textBaseline='middle';ctx.fillText(String(groupCounts[group]).padStart(2,'0'),64,64);const texture=new THREE.CanvasTexture(label);texture.colorSpace=THREE.SRGBColorSpace;const decal=new THREE.Mesh(new THREE.PlaneGeometry(1.7,1.7),new THREE.MeshBasicMaterial({map:texture,transparent:true,depthWrite:false}));decal.rotation.x=-Math.PI/2;decal.position.set(x,h+.055,z);scene.add(decal);
  }
 }
 for(let group=0;group<7;group++){
  const points=coursePoints[group],color=group===6?0xf2acdc:group>=3?0x73f2cd:[0x91edb8,0xffd38d,0xff9292][group];
  const path=new THREE.Line(new THREE.BufferGeometry().setFromPoints(points),new THREE.LineDashedMaterial({color,transparent:true,opacity:.5,dashSize:.5,gapSize:.5}));path.computeLineDistances();scene.add(path);
  const finish=points.at(-1);make('ring',gold,finish.x,finish.y+1.2,finish.z,1.5,1.5,1.5);
  const label=document.createElement('canvas');label.width=512;label.height=128;const ctx=label.getContext('2d');ctx.fillStyle='#'+color.toString(16);ctx.font='bold 42px sans-serif';ctx.textAlign='center';ctx.fillText(playgroundNames[group].toUpperCase(),256,70);const tex=new THREE.CanvasTexture(label);tex.colorSpace=THREE.SRGBColorSpace;const sign=new THREE.Mesh(new THREE.PlaneGeometry(10,2.5),new THREE.MeshBasicMaterial({map:tex,transparent:true,side:THREE.DoubleSide}));sign.position.copy(points[0]).add(new THREE.Vector3(0,2.8,4.5));scene.add(sign);
 }
 for(let i=0;i<10;i++){const o=16+i*9,m=mat(i<6?0xff9138:0x64e6ff,{roughness:i<6?.25:.35,metalness:i<6?.25:.2});bodyMats.push(m);const g=new THREE.Group();scene.add(g);make('sphere',m,0,0,0,data[o+3],data[o+3],data[o+3],g);if(i<6){const seam=make('ring',dark,0,0,0,data[o+3]*.99,data[o+3]*.99,data[o+3]*.99,g);seam.rotation.x=Math.PI/2;const band=make('ring',gold,0,0,0,data[o+3]*.99,data[o+3]*.99,data[o+3]*.99,g);band.rotation.y=Math.PI/2;}if(i>=6){const ring=make('ring',dark,0,0,0,.67,.67,.67,g);rings[i]=ring;}bodies.push(g);}
 ready=true;let savedPreset=0;try{savedPreset=Number(localStorage.getItem('impact-movement-preset'))||0;}catch{}core.set_movement_preset(Math.max(0,Math.min(2,savedPreset)));updatePreset();updateWeapon();$('start').disabled=false;$('start').textContent='Enter the arena ↗';notify('Rust physics loaded. Bat and shotgun share the same movement.',5);
}catch(error){$('start').textContent='Could not load the physics';$('notice').textContent=error.message;$('capability').textContent='Open through the hosted site or a local HTTP server; file:// cannot load WASM.';console.error(error);}
window.impactState=()=>{if(!ready)return {ready:false};read();return {ready,running,swingAge,batPending,batVisual:{position:bat.position.toArray(),tip:bat.localToWorld(new THREE.Vector3(0,1.12,0)).toArray(),trailVisible:batTrail.visible},movementPreset:core.movement_preset(),playground,wallRunning:!!core.wall_running(),arena:{width:core.arena_extent(0),length:core.arena_extent(1)},courseActive,runwayActive,courseProgress:core.course_progress(),obstacles:Array.from({length:core.obstacle_count()},(_,i)=>({x:core.obstacle(i,0),z:core.obstacle(i,1),w:core.obstacle(i,2),d:core.obstacle(i,3),h:core.obstacle(i,4),base:core.obstacle(i,5),group:core.obstacle(i,6),wall:!!core.obstacle(i,7)})),projectiles:Array.from({length:core.projectile_count()},(_,i)=>core.projectile(i,0)>0?{x:core.projectile(i,1),y:core.projectile(i,2),z:core.projectile(i,3),travel:core.projectile(i,7)}:null).filter(Boolean),touchControls,weapon:weapon===0?'bat':'shotgun',slow,assist,yaw,pitch,grounded:!!data[6],airJumpAvailable:!!core.air_jump_available(),score:data[8],hits:data[9],shots:data[10],hoops:data[15],player:{x:data[0],y:data[1],z:data[2],vx:data[3],vy:data[4],vz:data[5]},bodies:Array.from({length:10},(_,i)=>{const o=16+i*9;return {x:data[o],y:data[o+1],z:data[o+2],vx:data[o+6],vy:data[o+7],vz:data[o+8]};}),camera:{fov:camera.fov,x:camera.position.x,y:camera.position.y,z:camera.position.z},drawCalls:renderer.info.render.calls};};
