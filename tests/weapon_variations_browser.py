"""Exercise actual WASM, the right-hand animation corridor, and saved choices."""
import functools
import shutil
import threading
from pathlib import Path
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from playwright.sync_api import sync_playwright
ROOT=Path(__file__).resolve().parents[1]
class Quiet(SimpleHTTPRequestHandler):
 def log_message(self,*args): pass
server=ThreadingHTTPServer(('127.0.0.1',0),functools.partial(Quiet,directory=str(ROOT)))
threading.Thread(target=server.serve_forever,daemon=True).start()
try:
 with sync_playwright() as p:
  browser=p.chromium.launch(executable_path=shutil.which('chromium'),args=['--no-sandbox','--enable-unsafe-swiftshader'])
  for mobile in (False,True):
   page=browser.new_page(viewport={'width':390 if mobile else 960,'height':844 if mobile else 600},is_mobile=mobile,has_touch=mobile)
   page.set_default_timeout(60000)
   errors=[];page.on('pageerror',lambda e:errors.append(str(e)))
   page.goto(f'http://127.0.0.1:{server.server_port}/arena.html')
   page.wait_for_function('window.impactState?.().ready')
   assert page.evaluate('impactState().physicsMode')==1
   assert page.evaluate('impactState().bodies[0].health')==220
   # Check the rendered geometry, including arm and hand, at every stage.
   result=page.evaluate('''async()=>{
    const {createBatViewmodel,BAT_CYCLE}=await import('./arena/bat-viewmodel.mjs');
    const THREE=await import('./vendor/three.module.js');
    const rig=createBatViewmodel();let minX=Infinity,error=0;
    for(const aspect of [.46,.75,1,1.6,2.16]){
     const camera=new THREE.PerspectiveCamera(88,aspect,.06,10);
     for(let style=0;style<3;style++)for(let i=0;i<=100;i++){
      rig.update(BAT_CYCLE*i/100,style,camera);
      const s=rig.state();minX=Math.min(minX,s.screenBounds.minX);
      error=Math.max(error,Math.abs(s.upperArmLength-.43),Math.abs(s.forearmLength-.43));
     }
    }return {minX,error};}''')
   assert result['minX']>=.23999 and result['error']<1e-6,result
   for i in range(3):
    page.locator(f'#bat-style-{i}').click()
    assert page.evaluate('impactState().weapon')=='bat'
    page.locator('#start').click()
    page.wait_for_function('(i)=>impactState().batVisual.style===i',arg=i)
    page.screenshot(path=f'/tmp/bat-game-{mobile}-{i}-rest.png')
    page.locator('#slow').dispatch_event('click') # pause motion to inspect a real swing
    before=page.evaluate('impactState().shots')
    page.locator('#fire').dispatch_event('click',{'detail':0})
    page.wait_for_function('impactState().swingAge>.07 && impactState().swingAge<.17')
    page.screenshot(path=f'/tmp/bat-game-{mobile}-{i}-swing.png')
    page.wait_for_function('(n)=>impactState().shots>n',arg=before)
    assert page.evaluate('impactState().batVisual.screenBounds.minX')>=.23999
    if i==0:
     page.keyboard.press('KeyB');page.wait_for_function('impactState().batVisual.style===1')
     page.locator('#bat-cycle').dispatch_event('click');page.wait_for_function('impactState().batVisual.style===2')
    page.locator('#pause').dispatch_event('click')
    page.locator('#range').dispatch_event('click')
   # In Playful, the same bat contact launches the block without deleting it.
   page.locator('#physics-1').click();page.locator('#start').click()
   page.locator('#fire').dispatch_event('click',{'detail':0})
   page.wait_for_function('impactState().hits>0')
   assert page.evaluate('impactState().bodies[0].health>0 && impactState().bodies[0].health<220 && impactState().bodies[0].vz < -15')
   page.locator('#pause').dispatch_event('click');page.locator('#physics-2').click()
   assert page.evaluate('impactState().physicsMode')==2
   page.reload();page.wait_for_function('window.impactState?.().ready && impactState().batVisual.style===2')
   assert page.evaluate('impactState().physicsMode===2 && impactState().batVisual.style===2')
   assert page.locator('#bat-style-2').get_attribute('aria-pressed')=='true'
   page.locator('#physics-0').click();page.locator('#bat-style-0').click();page.locator('#start').click()
   page.locator('#fire').dispatch_event('click',{'detail':0})
   page.wait_for_function('impactState().breaks===1 && !impactState().bodies[0].visible')
   page.locator('#pause').dispatch_event('click');page.goto(f'http://127.0.0.1:{server.server_port}/bat-preview.html')
   page.wait_for_function('window.batPreviewState?.().ready')
   for i in range(3):
    page.locator(f'[data-style="{i}"]').click()
    page.locator('#scrub').evaluate('(e)=>{e.value="0.27";e.dispatchEvent(new Event("input"))}')
    s=page.evaluate('batPreviewState()');assert s['selected']==i and s['screenBounds']['minX']>=.23999
   assert not errors,errors
   print(('mobile' if mobile else 'desktop')+': all bat styles, 1,515 geometry samples, clear aiming corridor, fixed arms, slow motion contact, durable launch, Classic break, saved choices, shared preview; no browser errors',flush=True)
   page.close()
  browser.close()
finally:
 server.shutdown();server.server_close()
