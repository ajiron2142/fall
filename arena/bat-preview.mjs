import * as THREE from '../vendor/three.module.js';
const $=id=>document.getElementById(id);
const renderer=new THREE.WebGLRenderer({canvas:$('preview'),antialias:true});
renderer.setPixelRatio(Math.min(devicePixelRatio,2));
renderer.setClearColor(0x101c23);renderer.outputColorSpace=THREE.SRGBColorSpace;
const scene=new THREE.Scene();scene.fog=new THREE.Fog(0x101c23,12,40);
const camera=new THREE.PerspectiveCamera(78,1,.05,80);camera.position.set(0,1.7,0);scene.add(camera);
scene.add(new THREE.HemisphereLight(0xd9edff,0x384531,2.8));
const light=new THREE.DirectionalLight(0xffdca5,3);light.position.set(-3,7,2);scene.add(light);
const floor=new THREE.Mesh(new THREE.PlaneGeometry(70,70),new THREE.MeshStandardMaterial({color:0x26363c,roughness:1}));floor.rotation.x=-Math.PI/2;scene.add(floor);
const grid=new THREE.GridHelper(70,70,0x49616c,0x31474f);grid.position.y=.005;scene.add(grid);
for(let i=0;i<5;i++){const box=new THREE.Mesh(new THREE.BoxGeometry(1.2,1.2,1.2),new THREE.MeshStandardMaterial({color:i===2?0xedac6a:0x527483}));box.position.set((i-2)*3,.6,-7-Math.abs(i-2)*1.5);scene.add(box);}
const root=new THREE.Group();camera.add(root);
const bat=new THREE.Group();root.add(bat);
const wood=new THREE.MeshStandardMaterial({color:0xd7ac77,roughness:.58});
const grip=new THREE.MeshStandardMaterial({color:0x26313c,roughness:.9});
const skin=new THREE.MeshStandardMaterial({color:0xc18d69,roughness:.85});
const sleeve=new THREE.MeshStandardMaterial({color:0x507080,roughness:.9});
const profile=[[0,-.035],[.04,-.03],[.042,0],[.027,.02],[.025,.25],[.04,.38],[.068,.55],[.077,.8],[.074,1.02],[.06,1.06],[0,1.08]].map(([x,y])=>new THREE.Vector2(x,y));
bat.add(new THREE.Mesh(new THREE.LatheGeometry(profile,24),wood));
const tape=new THREE.Mesh(new THREE.CylinderGeometry(.029,.029,.23,16),grip);tape.position.y=.12;bat.add(tape);
const hand=new THREE.Mesh(new THREE.SphereGeometry(.07,16,12),skin);hand.scale.set(1,1.25,.95);hand.position.set(0,.11,.035);bat.add(hand);
const thumb=new THREE.Mesh(new THREE.SphereGeometry(.033,12,8),skin);thumb.position.set(-.045,.13,.065);bat.add(thumb);
const forearm=new THREE.Mesh(new THREE.CylinderGeometry(.052,.068,1,16),skin);root.add(forearm);
const cuff=new THREE.Mesh(new THREE.CylinderGeometry(.07,.095,1,16),sleeve);root.add(cuff);
root.traverse(object=>{if(object.isMesh){object.material=object.material.clone();object.material.depthTest=false;object.renderOrder=10;}});
// Keys specify actual grip position and shaft orientation; interpolation never stretches the bat.
const styles=[
 {name:'A · Low side sweep',description:'Tucked beside the hip. A wide, casual swipe across the screen.',keys:[
 [0,.52,-.73,-.46,.86,.42,-.28],[.18,.68,-.68,-.35,.72,.62,.13],[.42,.18,-.39,-.83,-.62,.52,-.59],[.62,-.43,-.58,-.65,-.95,.24,-.19],[.80,.03,-.82,-.49,.12,.91,-.4],[1,.52,-.73,-.46,.86,.42,-.28]]},
 {name:'B · Cross-body snap',description:'A relaxed diagonal carry. Big wind-up, flat sweep, long follow-through.',keys:[
 [0,.35,-.70,-.53,-.48,.87,-.1],[.22,.68,-.43,-.34,.75,.64,.16],[.44,.04,-.34,-.85,-.68,.08,-.73],[.66,-.60,-.39,-.43,-.97,.14,.1],[.84,-.08,-.79,-.55,-.84,.52,-.1],[1,.35,-.70,-.53,-.48,.87,-.1]]},
 {name:'C · Wrist flick',description:'Low, loose carry. An exaggerated upward smack, then an easy wrist roll.',keys:[
 [0,.49,-.91,-.44,.68,.67,-.28],[.22,.57,-1.02,-.38,.88,.11,-.47],[.44,.18,-.37,-.84,-.2,.92,-.32],[.62,-.12,-.22,-.63,-.65,.75,.11],[.82,.39,-.55,-.40,.91,.37,.18],[1,.49,-.91,-.44,.68,.67,-.28]]}
];
let selected=0,phase=0,playing=true,loop=true,slow=false,rest=0;
const up=new THREE.Vector3(0,1,0),direction=new THREE.Vector3(),handPosition=new THREE.Vector3();
function segment(mesh,a,b){mesh.position.copy(a).add(b).multiplyScalar(.5);mesh.scale.y=a.distanceTo(b);mesh.quaternion.setFromUnitVectors(up,b.clone().sub(a).normalize());}
function pose(){
 const keys=styles[selected].keys;let index=0;while(index<keys.length-2&&phase>keys[index+1][0])index++;
 const a=keys[index],b=keys[index+1];let t=THREE.MathUtils.clamp((phase-a[0])/(b[0]-a[0]),0,1);t=t*t*(3-2*t);
 const v=a.slice(1).map((value,i)=>THREE.MathUtils.lerp(value,b[i+1],t));
 const portrait=camera.aspect<.8;root.scale.setScalar(portrait?.65:1);root.position.set(portrait?-.30:0,.16,portrait?-.80:-.55);
 bat.position.set(v[0],v[1],v[2]);direction.set(v[3],v[4],v[5]).normalize();bat.quaternion.setFromUnitVectors(up,direction);bat.updateMatrix();
 handPosition.set(0,.09,.035).applyMatrix4(bat.matrix);
 const elbow=new THREE.Vector3(.64,-1.15,.05),wrist=handPosition.clone(),mid=elbow.clone().lerp(wrist,.40);
 segment(cuff,elbow,mid);segment(forearm,mid,wrist);
 $('scrub').value=phase;
}
function choose(index){selected=index;phase=0;rest=0;playing=true;$('title').textContent=styles[index].name;$('description').textContent=styles[index].description;document.querySelectorAll('[data-style]').forEach(button=>button.setAttribute('aria-pressed',Number(button.dataset.style)===index));resize();}
document.querySelectorAll('[data-style]').forEach(button=>button.onclick=()=>choose(Number(button.dataset.style)));
$('swing').onclick=()=>{phase=0;rest=0;playing=true;};
$('loop').onclick=()=>{loop=!loop;$('loop').textContent=`Loop: ${loop?'on':'off'}`;$('loop').setAttribute('aria-pressed',loop);};
$('slow').onclick=()=>{slow=!slow;$('slow').textContent=`Slow: ${slow?'on':'off'}`;$('slow').setAttribute('aria-pressed',slow);};
$('scrub').oninput=()=>{playing=false;phase=Number($('scrub').value);pose();};
function resize(){const height=Math.max(240,innerHeight-document.querySelector("main").offsetHeight);renderer.domElement.style.height=`${height}px`;renderer.setSize(innerWidth,height,false);camera.aspect=innerWidth/height;camera.updateProjectionMatrix();pose();}addEventListener('resize',resize);
let previous=performance.now();
function frame(now){const dt=Math.min((now-previous)/1000,.05);previous=now;if(playing){if(rest>0){rest=Math.max(0,rest-dt);phase=0;}else{phase=Math.min(1,phase+dt/(slow?2.4:.68));if(phase===1){if(loop){phase=0;rest=.8;}else playing=false;}}}pose();renderer.render(scene,camera);requestAnimationFrame(frame);}
choose(0);resize();requestAnimationFrame(frame);
window.batPreviewState=()=>({ready:true,selected,phase,playing,loop,slow,grip:bat.position.toArray(),direction:direction.toArray(),hands:1});
