import * as THREE from '../vendor/three.module.js';

export const BAT_CONTACT=.12, BAT_CYCLE=.44;
// Right-hand studies. Contact is at the same world-time instant for every style.
// The shaft cuts out to the right rather than passing across the aiming area.
export const BAT_STYLES=[
 {name:'Low sweep',description:'Loose diagonal carry; a low, outward smack.',keys:[
 [0,.34,-.34,-.70,.44,.88,.13,0],
 [.065,.43,-.25,-.57,.52,.78,.35,-.18],
 [.12,.43,-.33,-.91,.93,-.06,-.35,.12],
 [.19,.43,-.51,-.80,.60,-.77,-.20,.22],
 [.28,.36,-.45,-.72,.72,.65,.22,.10],
 [.44,.34,-.34,-.70,.44,.88,.13,0]]},
 {name:'Sidearm whip',description:'A wider horizontal whip; the follow-through leaves the screen.',keys:[
 [0,.35,-.40,-.71,.65,.75,.06,-.12],
 [.065,.49,-.31,-.52,.79,.45,.42,-.35],
 [.12,.42,-.36,-.94,.91,-.12,-.39,.18],
 [.19,.53,-.48,-.76,.94,-.31,.14,.40],
 [.28,.45,-.43,-.65,.80,.51,.31,.08],
 [.44,.35,-.40,-.71,.65,.75,.06,-.12]]},
 {name:'Short snap',description:'A more upright hold; a quick diagonal snap and wrist recovery.',keys:[
 [0,.33,-.28,-.70,.19,.97,.14,.12],
 [.065,.41,-.19,-.58,.41,.83,.37,-.15],
 [.12,.36,-.30,-.89,.83,-.27,-.49,.32],
 [.19,.39,-.50,-.78,.38,-.89,-.25,.45],
 [.28,.35,-.39,-.72,.45,.83,.32,.24],
 [.44,.33,-.28,-.70,.19,.97,.14,.12]]}
];

function sample(keys,age){
 if(age>=BAT_CYCLE)return keys[0].slice(1);
 let i=0;while(i<keys.length-2&&age>keys[i+1][0])i++;
 const a=keys[i],b=keys[i+1],span=b[0]-a[0],t=THREE.MathUtils.clamp((age-a[0])/span,0,1);
 const slope=(n,k)=>n===0||n===keys.length-1?0:(keys[n+1][k]-keys[n-1][k])/(keys[n+1][0]-keys[n-1][0]);
 return a.slice(1).map((v,j)=>{const k=j+1;return (2*t**3-3*t*t+1)*v+(t**3-2*t*t+t)*span*slope(i,k)+(-2*t**3+3*t*t)*b[k]+(t**3-t*t)*span*slope(i+1,k);});
}

export function createBatViewmodel(){
 const root=new THREE.Group(),bat=new THREE.Group();root.add(bat);
 const wood=new THREE.MeshStandardMaterial({color:0xd0a475,roughness:.48});
 const tape=new THREE.MeshStandardMaterial({color:0x24323d,roughness:.94});
 const skin=new THREE.MeshStandardMaterial({color:0xc79b7e,roughness:.85});
 const fabric=new THREE.MeshStandardMaterial({color:0x304957,roughness:.95});
 const mesh=(geometry,material,parent=root)=>{const m=new THREE.Mesh(geometry,material);parent.add(m);return m;};
 const profile=[[0,-.034],[.036,-.027],[.036,-.005],[.022,.012],[.023,.20],[.031,.31],[.045,.43],[.060,.59],[.065,.76],[.061,.82],[.046,.855],[0,.87]].map(([r,y])=>new THREE.Vector2(r,y));
 mesh(new THREE.LatheGeometry(profile,24),wood,bat);
 const handle=mesh(new THREE.CylinderGeometry(.026,.026,.20,16),tape,bat);handle.position.y=.12;
 for(let i=0;i<6;i++){const seam=mesh(new THREE.TorusGeometry(.0265,.0015,4,16),tape,bat);seam.rotation.x=Math.PI/2;seam.position.y=.04+i*.03;}
 const palm=mesh(new THREE.SphereGeometry(1,20,14),skin,bat);palm.position.set(.025,.115,.046);palm.scale.set(.044,.058,.031);
 for(let i=0;i<4;i++){
  const y=.076+i*.024,curve=new THREE.CatmullRomCurve3([
   new THREE.Vector3(.055,y,.035),new THREE.Vector3(.025,y,.062),new THREE.Vector3(-.019,y,.045),new THREE.Vector3(-.031,y,.004),new THREE.Vector3(-.008,y,-.025)]);
  mesh(new THREE.TubeGeometry(curve,10,.009,6,false),skin,bat);
 }
 const thumbCurve=new THREE.CatmullRomCurve3([new THREE.Vector3(.055,.14,.041),new THREE.Vector3(.056,.112,.029),new THREE.Vector3(.035,.09,.013),new THREE.Vector3(.002,.084,.020)]);
 mesh(new THREE.TubeGeometry(thumbCurve,10,.014,8,false),skin,bat);
 const forearm=mesh(new THREE.CylinderGeometry(.039,.055,1,16),skin);
 const upper=mesh(new THREE.CylinderGeometry(.056,.072,1,16),fabric);
 const joint=mesh(new THREE.SphereGeometry(.055,16,12),skin);
 const wristCap=mesh(new THREE.SphereGeometry(.039,16,12),skin);
 const shoulder=new THREE.Vector3(.53,-.79,-.18),wrist=new THREE.Vector3(),elbow=new THREE.Vector3();
 const upperLength=.43,forearmLength=.43,up=new THREE.Vector3(0,1,0),shaft=new THREE.Vector3(),offset=new THREE.Vector3(),axis=new THREE.Vector3(),pole=new THREE.Vector3();
 const wristLocal=new THREE.Vector3(.025,.045,.047);
 const corners=[];
 root.traverse(o=>{if(o.isMesh){o.geometry.computeBoundingBox();const b=o.geometry.boundingBox;for(const x of [b.min.x,b.max.x])for(const y of [b.min.y,b.max.y])for(const z of [b.min.z,b.max.z])corners.push({mesh:o,point:new THREE.Vector3(x,y,z)});}});
 let bounds={minX:1,maxX:1},variant=0;
 const point=new THREE.Vector3();
 function projectBounds(camera){let minX=Infinity,maxX=-Infinity,minY=Infinity,maxY=-Infinity;for(const c of corners){point.copy(c.point).applyMatrix4(c.mesh.matrixWorld).project(camera);minX=Math.min(minX,point.x);maxX=Math.max(maxX,point.x);minY=Math.min(minY,point.y);maxY=Math.max(maxY,point.y);}return {minX,maxX,minY,maxY};}
 function segment(m,a,b){axis.copy(b).sub(a);m.position.copy(a).addScaledVector(axis,.5);m.scale.y=axis.length();m.quaternion.setFromUnitVectors(up,axis.normalize());}
 function update(age,index,camera,time=0,idle=false){
  variant=Math.max(0,Math.min(2,index));const v=sample(BAT_STYLES[variant].keys,age);
  const portrait=camera.aspect<.8;
  root.scale.setScalar(portrait?.60:1);root.position.set(0,portrait?-.04:0,portrait?-.50:0);
  wrist.set(v[0],v[1],v[2]);
  if(idle&&age>=BAT_CYCLE){wrist.y+=Math.sin(time*1.5)*.003;wrist.x+=Math.sin(time*1.5+.7)*.002;}
  axis.copy(wrist).sub(shoulder).normalize();const distance=Math.min(wrist.distanceTo(shoulder),upperLength+forearmLength-.025);
  wrist.copy(shoulder).addScaledVector(axis,distance);
  const along=(upperLength**2-forearmLength**2+distance**2)/(2*distance),height=Math.sqrt(Math.max(0,upperLength**2-along**2));
  pole.set(.9,-.45,.5).addScaledVector(axis,-pole.dot(axis)).normalize();
  elbow.copy(shoulder).addScaledVector(axis,along).addScaledVector(pole,height);
  shaft.set(v[3],v[4],v[5]).normalize();bat.quaternion.setFromUnitVectors(up,shaft);bat.rotateY(v[6]);
  offset.copy(wristLocal).applyQuaternion(bat.quaternion);bat.position.copy(wrist).sub(offset);
  segment(upper,shoulder,elbow);segment(forearm,elbow,wrist);joint.position.copy(elbow);wristCap.position.copy(wrist);
  root.updateMatrixWorld(true);camera.updateMatrixWorld(true);
  // Keep a protected aiming corridor in every aspect ratio. The whole arm is
  // translated as one rig, so this never stretches bones or rotates the camera.
  bounds=projectBounds(camera);
  const shiftNdc=Math.max(0,.24-bounds.minX);
  const furthestDepth=Math.max(...corners.map(c=>-(point.copy(c.point).applyMatrix4(c.mesh.matrixWorld).z)));
  root.position.x+=shiftNdc*furthestDepth/camera.projectionMatrix.elements[0];
  root.updateMatrixWorld(true);bounds=projectBounds(camera);
 }
 function state(){return {hands:1,style:variant,name:BAT_STYLES[variant].name,position:bat.position.toArray(),upperArmLength:shoulder.distanceTo(elbow),forearmLength:elbow.distanceTo(wrist),screenBounds:bounds,tip:bat.localToWorld(new THREE.Vector3(0,.87,0)).toArray(),trailVisible:false};}
 return {root,bat,update,state};
}
