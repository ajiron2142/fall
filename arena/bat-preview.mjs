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
// Render the viewmodel separately so hands and bat occlude each other correctly.
const viewScene=new THREE.Scene();
const viewCamera=new THREE.PerspectiveCamera(78,1,.05,10);
viewScene.add(new THREE.HemisphereLight(0xd9edff,0x384531,2.8));
const viewLight=new THREE.DirectionalLight(0xffdca5,3);viewLight.position.set(-3,7,2);viewScene.add(viewLight);
const root=new THREE.Group();viewScene.add(root);
renderer.autoClear=false;
const bat=new THREE.Group();root.add(bat);
const wood=new THREE.MeshStandardMaterial({color:0xd7ac77,roughness:.58});
const grip=new THREE.MeshStandardMaterial({color:0x26313c,roughness:.9});
const skin=new THREE.MeshStandardMaterial({color:0xc18d69,roughness:.85});
const sleeve=new THREE.MeshStandardMaterial({color:0x507080,roughness:.9});
const profile=[[0,-.035],[.035,-.03],[.035,0],[.021,.025],[.023,.22],[.034,.35],[.054,.49],[.061,.67],[.058,.84],[.047,.88],[0,.9]].map(([x,y])=>new THREE.Vector2(x,y));
bat.add(new THREE.Mesh(new THREE.LatheGeometry(profile,24),wood));
const tape=new THREE.Mesh(new THREE.CylinderGeometry(.025,.025,.19,16),grip);tape.position.y=.115;bat.add(tape);
// A compact palm and curled fingers replace the old ball-shaped mitten.
const palm=new THREE.Mesh(new THREE.CapsuleGeometry(.038,.025,6,12),skin);palm.scale.set(1.15,1,.75);palm.position.set(.018,.115,.043);bat.add(palm);
for(let i=0;i<4;i++){
 const finger=new THREE.Mesh(new THREE.CapsuleGeometry(.012,.045,4,8),skin);
 finger.rotation.z=Math.PI/2;finger.position.set(.008,.078+i*.024,-.009);bat.add(finger);
 const curl=new THREE.Mesh(new THREE.SphereGeometry(.014,12,8),skin);curl.position.set(-.025,.078+i*.024,.02);bat.add(curl);
}
const thumb=new THREE.Mesh(new THREE.CapsuleGeometry(.018,.038,4,8),skin);thumb.rotation.z=-.55;thumb.position.set(.059,.13,.024);bat.add(thumb);
const forearm=new THREE.Mesh(new THREE.CylinderGeometry(.039,.053,1,16),skin);root.add(forearm);
const cuff=new THREE.Mesh(new THREE.CylinderGeometry(.055,.071,1,16),sleeve);root.add(cuff);
const elbowJoint=new THREE.Mesh(new THREE.SphereGeometry(.053,16,12),skin);root.add(elbowJoint);
// Original motion studies: compact grip travel, a sideways snap, then a relaxed recovery.
// Grip position and shaft direction are authored separately; arm lengths remain constant.
const styles=[
 {name:'A · Compact snap',description:'Upright carry. Short wind-up, quick sideways smack, loose recovery.',keys:[
 [0,.35,-.48,-.60,-.25,.94,-.12],[.26,.44,-.42,-.52,.25,.94,.20],
 [.40,.27,-.38,-.73,-.60,.30,-.74],[.50,.07,-.46,-.66,-.96,-.04,-.26],
 [.60,.09,-.50,-.58,-.90,-.30,.25],[.80,.27,-.51,-.63,-.65,.71,-.15],
 [1,.35,-.48,-.60,-.25,.94,-.12]]},
 {name:'B · Shoulder whip',description:'Bat tipped back beside the shoulder. A bigger downward diagonal snap.',keys:[
 [0,.35,-.45,-.60,.04,.98,.17],[.27,.44,-.38,-.50,.50,.71,.49],
 [.42,.24,-.32,-.71,-.50,.48,-.72],[.52,.04,-.46,-.65,-.90,-.30,-.31],
 [.63,.08,-.53,-.58,-.66,-.64,.39],[.82,.29,-.49,-.61,-.34,.84,.43],
 [1,.35,-.45,-.60,.04,.98,.17]]},
 {name:'C · Loose backhand',description:'An inward diagonal carry. Almost no wind-up; a quick, low backhand.',keys:[
 [0,.35,-.51,-.61,-.57,.81,-.14],[.25,.41,-.48,-.54,-.19,.97,.15],
 [.40,.25,-.46,-.71,-.75,.06,-.66],[.49,.09,-.51,-.66,-.95,-.22,-.20],
 [.62,.15,-.56,-.59,-.88,-.38,.28],[.81,.30,-.55,-.62,-.81,.54,.20],
 [1,.35,-.51,-.61,-.57,.81,-.14]]}
];
let selected=0,phase=0,playing=true,loop=true,slow=false,rest=0;
const up=new THREE.Vector3(0,1,0),direction=new THREE.Vector3(),handPosition=new THREE.Vector3();
const shoulder=new THREE.Vector3(.59,-.88,-.06),elbow=new THREE.Vector3();
const upperLength=.43,forearmLength=.43;
function solveElbow(wrist){
 const delta=wrist.clone().sub(shoulder),axis=delta.clone().normalize();
 const reach=upperLength+forearmLength-.015;
 const distance=Math.min(delta.length(),reach);
 wrist.copy(shoulder).addScaledVector(axis,distance);
 const along=(upperLength**2-forearmLength**2+distance**2)/(2*distance);
 const height=Math.sqrt(Math.max(0,upperLength**2-along**2));
 const pole=new THREE.Vector3(.8,-.6,.5);pole.addScaledVector(axis,-pole.dot(axis)).normalize();
 elbow.copy(shoulder).addScaledVector(axis,along).addScaledVector(pole,height);
}
function segment(mesh,a,b){mesh.position.copy(a).add(b).multiplyScalar(.5);mesh.scale.y=a.distanceTo(b);mesh.quaternion.setFromUnitVectors(up,b.clone().sub(a).normalize());}
function pose(){
 const keys=styles[selected].keys;let index=0;while(index<keys.length-2&&phase>keys[index+1][0])index++;
 const a=keys[index],b=keys[index+1],span=b[0]-a[0];
 const t=THREE.MathUtils.clamp((phase-a[0])/span,0,1);
 // Continuous velocity through contact; easing every key made the old swing hesitate.
 const tangent=(key,component)=>{
  if(key===0||key===keys.length-1)return 0;
  return (keys[key+1][component]-keys[key-1][component])/(keys[key+1][0]-keys[key-1][0]);
 };
 const v=a.slice(1).map((value,i)=>{
  const k=i+1;
  return (2*t**3-3*t**2+1)*value+(t**3-2*t**2+t)*span*tangent(index,k)
   +(-2*t**3+3*t**2)*b[k]+(t**3-t**2)*span*tangent(index+1,k);
 });
 const portrait=camera.aspect<.8;
 root.scale.setScalar(portrait?.82:1);root.position.set(portrait?-.16:0,.12,portrait?-.48:-.27);
 handPosition.set(v[0],v[1],v[2]);direction.set(v[3],v[4],v[5]).normalize();
 bat.quaternion.setFromUnitVectors(up,direction);
 // Place the hand at the authored grip anchor, not the bottom of the bat.
 const gripOffset=new THREE.Vector3(.018,.115,.025).applyQuaternion(bat.quaternion);
 solveElbow(handPosition);
 bat.position.copy(handPosition).sub(gripOffset);
 segment(cuff,shoulder,elbow);segment(forearm,elbow,handPosition);
 elbowJoint.position.copy(elbow);
 $('scrub').value=phase;
}
function choose(index){selected=index;phase=0;rest=0;playing=true;$('title').textContent=styles[index].name;$('description').textContent=styles[index].description;document.querySelectorAll('[data-style]').forEach(button=>button.setAttribute('aria-pressed',Number(button.dataset.style)===index));resize();}
document.querySelectorAll('[data-style]').forEach(button=>button.onclick=()=>choose(Number(button.dataset.style)));
$('swing').onclick=()=>{phase=0;rest=0;playing=true;};
$('loop').onclick=()=>{loop=!loop;$('loop').textContent=`Loop: ${loop?'on':'off'}`;$('loop').setAttribute('aria-pressed',loop);};
$('slow').onclick=()=>{slow=!slow;$('slow').textContent=`Slow: ${slow?'on':'off'}`;$('slow').setAttribute('aria-pressed',slow);};
$('scrub').oninput=()=>{playing=false;phase=Number($('scrub').value);pose();};
function resize(){const height=Math.max(240,innerHeight-document.querySelector("main").offsetHeight);renderer.domElement.style.height=`${height}px`;renderer.setSize(innerWidth,height,false);camera.aspect=innerWidth/height;camera.updateProjectionMatrix();viewCamera.aspect=camera.aspect;viewCamera.updateProjectionMatrix();pose();}addEventListener('resize',resize);
let previous=performance.now();
function frame(now){const dt=Math.min((now-previous)/1000,.05);previous=now;if(playing){if(rest>0){rest=Math.max(0,rest-dt);phase=0;}else{phase=Math.min(1,phase+dt/(slow?2.4:.68));if(phase===1){if(loop){phase=0;rest=.8;}else playing=false;}}}pose();renderer.clear();renderer.render(scene,camera);renderer.clearDepth();renderer.render(viewScene,viewCamera);requestAnimationFrame(frame);}
choose(0);resize();requestAnimationFrame(frame);
window.batPreviewState=()=>({ready:true,selected,phase,playing,loop,slow,grip:bat.position.toArray(),direction:direction.toArray(),hands:1,upperArmLength:shoulder.distanceTo(elbow),forearmLength:elbow.distanceTo(handPosition)});
