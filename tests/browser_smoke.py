import functools
import shutil
import threading
from http.server import ThreadingHTTPServer, SimpleHTTPRequestHandler
from pathlib import Path
from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parents[1]
class QuietHandler(SimpleHTTPRequestHandler):
 def log_message(self, *args): pass
server = ThreadingHTTPServer(('127.0.0.1', 0), functools.partial(QuietHandler, directory=str(ROOT)))
threading.Thread(target=server.serve_forever, daemon=True).start()
BASE = 'http://127.0.0.1:' + str(server.server_port)
try:
 with sync_playwright() as p:
  browser=p.chromium.launch(executable_path=shutil.which('chromium') or shutil.which('google-chrome'),args=['--no-sandbox','--enable-unsafe-swiftshader'])
  page=browser.new_page(viewport={'width':1000,'height':700});errors=[];page.on('pageerror',lambda e:errors.append(str(e)))
  page.goto(BASE+'/afterlight.html');page.wait_for_function('window.afterlightState && afterlightState().drawCalls>0')
  page.locator('#start').click();page.wait_for_function('afterlightState().grounded')
  page.keyboard.press('Space');page.wait_for_function('afterlightState().player.y>.5');page.wait_for_function('afterlightState().grounded')
  page.keyboard.press('KeyE');assert page.evaluate('afterlightState().phase')==1
  page.keyboard.down('KeyW');page.wait_for_function('afterlightState().player.z < -1');page.keyboard.up('KeyW')
  page.keyboard.press('Shift');page.wait_for_function('Math.abs(afterlightState().player.vz)>8')
  page.screenshot(path='/tmp/afterlight-desktop.png')
  page.keyboard.down('KeyD');page.wait_for_function('afterlightState().falls>0',timeout=30000);page.keyboard.up('KeyD')
  page.evaluate('document.exitPointerLock()');page.locator('#menu').click();assert not page.evaluate('afterlightState().running')
  old=page.evaluate('afterlightState().seed');page.locator('#new-world').click();assert old!=page.evaluate('afterlightState().seed')
  assert not errors,errors
  mobile=browser.new_page(viewport={'width':390,'height':844},is_mobile=True,has_touch=True)
  mobile.on('pageerror',lambda e:errors.append(str(e)));mobile.goto(BASE+'/afterlight.html');mobile.wait_for_function('window.afterlightState && afterlightState().drawCalls>0');mobile.locator('#start').tap();mobile.wait_for_function('afterlightState().grounded');mobile.locator('#jump').tap();mobile.wait_for_function('afterlightState().player.y>.5');mobile.locator('#shift').tap();assert mobile.evaluate('afterlightState().phase')==1
  mobile.screenshot(path='/tmp/afterlight-mobile.png');assert mobile.evaluate('document.documentElement.scrollWidth<=innerWidth');assert not errors,errors
  mobile.wait_for_function('afterlightState().grounded')
  cdp=mobile.context.new_cdp_session(mobile)
  r=mobile.locator('#stick').bounding_box();x=r['x']+r['width']/2;y=r['y']+r['height']/2
  before=mobile.evaluate('afterlightState().player.z')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y,'id':1}]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchMove','touchPoints':[{'x':x,'y':y-30,'id':1}]})
  mobile.wait_for_function('afterlightState().player.z < -1.8')
  jr=mobile.locator('#jump').bounding_box();jx=jr['x']+jr['width']/2;jy=jr['y']+jr['height']/2
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y-30,'id':1},{'x':jx,'y':jy,'id':2}]})
  mobile.wait_for_function('afterlightState().player.y>.5 && afterlightState().player.vz < -3')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[{'x':x,'y':y-30,'id':1}]})
  mobile.wait_for_function('afterlightState().grounded && afterlightState().player.z < -3.5')
  assert mobile.evaluate('afterlightState().falls')==0
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  yaw=mobile.evaluate('afterlightState().yaw')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':300,'y':350,'id':2}]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchMove','touchPoints':[{'x':340,'y':360,'id':2}]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  assert abs(mobile.evaluate('afterlightState().yaw')-yaw)>.1
  mobile.locator('#fullscreen').tap();mobile.wait_for_function('!!document.fullscreenElement');mobile.evaluate('document.exitFullscreen()')
  # Instrument only the intercepted test response, never the shipped file.
  sim=browser.new_page()
  hook='''
window.testRoutes=()=>{
 const results=[];
 for(let run=0;run<30;run++){
  generate();running=true;keys.clear();
  for(let k=0;k<60;k++)physics(1/120);
  for(let i=1;i<platforms.length;i++){
   const target=platforms[i];let jumped=false,reached=false;
   for(let step=0;step<900;step++){
    const dx=target.x-player.x,dz=target.z-player.z,dist=Math.hypot(dx,dz);
    yaw=Math.atan2(-dx,-dz);keys.clear();
    if(dist>.7)keys.add('KeyW');
    if(!jumped&&grounded&&dist<5.7){jumpQueued=true;jumped=true;}
    physics(1/120);
    if(jumped&&!grounded&&target.type!==-1&&phase!==target.type){shiftCooldown=0;shift();}
    if(grounded&&Math.hypot(target.x-player.x,target.z-player.z)<1.7){reached=true;break;}
    if(falls>0)throw Error('Fall on generated route '+run+' island '+i);
   }
   if(!reached)throw Error('Could not reach island '+i);
  }
  if(!won||!anchors.every(a=>a.charged))throw Error('Win or checkpoints failed');
  results.push(seed);
 }
 return results.length;
};'''
  source=(ROOT / 'echo.js').read_text()
  sim.route('**/echo.js',lambda route:route.fulfill(body=source+hook,content_type='text/javascript'))
  sim.goto(BASE+'/afterlight.html');sim.wait_for_function('window.testRoutes')
  count=sim.evaluate('testRoutes()')
  print('Passed desktop and mobile browser controls, rendering, fall recovery, pause and layout; completed',count,'generated worlds through real physics, all anchors and win condition.')
  browser.close()
finally:server.shutdown();server.server_close()
