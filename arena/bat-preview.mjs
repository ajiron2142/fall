import * as THREE from '../vendor/three.module.js';
import {createBatViewmodel,BAT_STYLES,BAT_CYCLE} from './bat-viewmodel.mjs';
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
const viewScene=new THREE.Scene(),viewCamera=new THREE.PerspectiveCamera(88,1,.06,10);
viewScene.add(new THREE.HemisphereLight(0xd9edff,0x384531,2.8));
const viewLight=new THREE.DirectionalLight(0xffdca5,3);viewLight.position.set(-3,7,2);viewScene.add(viewLight);
const rig=createBatViewmodel();viewScene.add(rig.root);renderer.autoClear=false;
const styles=BAT_STYLES.map((style,i)=>({...style,name:String.fromCharCode(65+i)+' · '+style.name}));
let selected=0,phase=0,playing=true,loop=true,slow=false,rest=0;
function pose(){rig.update(phase*BAT_CYCLE,selected,viewCamera);$('scrub').value=phase;}
function choose(index){selected=index;phase=0;rest=0;playing=true;$('title').textContent=styles[index].name;$('description').textContent=styles[index].description;document.querySelectorAll('[data-style]').forEach(button=>button.setAttribute('aria-pressed',Number(button.dataset.style)===index));resize();}
document.querySelectorAll('[data-style]').forEach(button=>button.onclick=()=>choose(Number(button.dataset.style)));
$('swing').onclick=()=>{phase=0;rest=0;playing=true;};
$('loop').onclick=()=>{loop=!loop;$('loop').textContent=`Loop: ${loop?'on':'off'}`;$('loop').setAttribute('aria-pressed',loop);};
$('slow').onclick=()=>{slow=!slow;$('slow').textContent=`Slow: ${slow?'on':'off'}`;$('slow').setAttribute('aria-pressed',slow);};
$('scrub').oninput=()=>{playing=false;phase=Number($('scrub').value);pose();};
function resize(){const height=Math.max(240,innerHeight-document.querySelector("main").offsetHeight);renderer.domElement.style.height=`${height}px`;renderer.setSize(innerWidth,height,false);camera.aspect=innerWidth/height;camera.updateProjectionMatrix();viewCamera.aspect=camera.aspect;viewCamera.updateProjectionMatrix();pose();}addEventListener('resize',resize);
let previous=performance.now();
function frame(now){const dt=Math.min((now-previous)/1000,.05);previous=now;if(playing){if(rest>0){rest=Math.max(0,rest-dt);phase=0;}else{phase=Math.min(1,phase+dt/(slow?2.4:BAT_CYCLE));if(phase===1){if(loop){phase=0;rest=.8;}else playing=false;}}}pose();renderer.clear();renderer.render(scene,camera);renderer.clearDepth();renderer.render(viewScene,viewCamera);requestAnimationFrame(frame);}
choose(0);resize();requestAnimationFrame(frame);
window.batPreviewState=()=>({ready:true,selected,phase,playing,loop,slow,...rig.state()});
