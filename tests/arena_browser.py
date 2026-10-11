import functools
import shutil
import threading
from pathlib import Path
from http.server import SimpleHTTPRequestHandler,ThreadingHTTPServer
from playwright.sync_api import sync_playwright
ROOT=Path(__file__).resolve().parents[1]
class QuietHandler(SimpleHTTPRequestHandler):
 def log_message(self,*args): pass
server=ThreadingHTTPServer(('127.0.0.1',0),functools.partial(QuietHandler,directory=str(ROOT)))
threading.Thread(target=server.serve_forever,daemon=True).start()
base='http://127.0.0.1:'+str(server.server_port)
try:
 with sync_playwright() as p:
  browser=p.chromium.launch(executable_path=shutil.which('chromium') or shutil.which('google-chrome'),args=['--no-sandbox','--enable-unsafe-swiftshader'])
  page=browser.new_page(viewport={'width':390,'height':844},is_mobile=True,has_touch=True)
  errors=[];page.on('pageerror',lambda e:errors.append(str(e)))
  page.goto(base+'/arena.html');page.wait_for_function('window.impactState && impactState().ready && impactState().drawCalls>0')
  page.locator('#start').tap();page.locator('#fire').tap();page.wait_for_function('impactState().hits>0');assert page.evaluate('impactState().bodies[0].vz')< -15
  page.wait_for_timeout(500)
  page.screenshot(path='/tmp/impact-mobile.png')
  cdp=page.context.new_cdp_session(page);r=page.locator('#stick').bounding_box();x=r['x']+r['width']/2;y=r['y']+r['height']/2
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y,'id':1}]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchMove','touchPoints':[{'x':x,'y':y-30,'id':1}]})
  page.wait_for_function('impactState().player.z<9.3')
  jr=page.locator('#jump').bounding_box();jx=jr['x']+jr['width']/2;jy=jr['y']+jr['height']/2
  before=page.evaluate('impactState().player.z')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y-30,'id':1},{'x':jx,'y':jy,'id':2}]})
  page.wait_for_function('(z)=>impactState().player.y>.6 && impactState().player.z<z-.3',arg=before)
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[{'x':x,'y':y-30,'id':1}]})
  page.wait_for_function('impactState().grounded')
  dr=page.locator('#dash').bounding_box();dx=dr['x']+dr['width']/2;dy=dr['y']+dr['height']/2
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y-30,'id':1},{'x':dx,'y':dy,'id':3}]})
  page.wait_for_function('Math.abs(impactState().player.vz)>20')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  page.locator('#slow').tap();assert page.evaluate('impactState().slow');page.locator('#slow').tap();assert not page.evaluate('impactState().slow')
  page.locator('#weapon').tap();assert page.evaluate('impactState().weapon')=='gun'
  page.locator('#pause').tap();assert not page.evaluate('impactState().running')
  page.locator('#reset').tap();page.locator('#start').tap();page.locator('#fire').tap();page.wait_for_function('impactState().hits>0');assert page.evaluate('impactState().shots')>=1
  page.locator('#fullscreen').tap();page.wait_for_function('!!document.fullscreenElement');page.evaluate('document.exitFullscreen()')
  yaw=page.evaluate('impactState().yaw')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':270,'y':350,'id':4}]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchMove','touchPoints':[{'x':310,'y':360,'id':4}]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  assert abs(page.evaluate('impactState().yaw')-yaw)>.1
  fr=page.locator('#fire').bounding_box();fx=fr['x']+fr['width']/2;fy=fr['y']+fr['height']/2
  yaw=page.evaluate('impactState().yaw');shots=page.evaluate('impactState().shots')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':fx,'y':fy,'id':5}]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchMove','touchPoints':[{'x':fx-25,'y':fy-10,'id':5}]})
  page.wait_for_function('(n)=>impactState().shots>n',arg=shots)
  assert abs(page.evaluate('impactState().yaw')-yaw)>.05
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  page.set_viewport_size({'width':844,'height':390});page.wait_for_timeout(300)
  assert page.evaluate('document.documentElement.scrollWidth<=innerWidth')
  desktop=browser.new_page(viewport={'width':1280,'height':800});desktop.on('pageerror',lambda e:errors.append(str(e)));desktop.goto(base+'/arena.html');desktop.wait_for_function('window.impactState && impactState().ready');desktop.locator('#start').click();desktop.mouse.down();desktop.mouse.up();desktop.wait_for_function('impactState().hits>0')
  desktop.keyboard.down('KeyW');desktop.wait_for_function('impactState().player.z<9');desktop.keyboard.press('Space');desktop.wait_for_function('impactState().player.y>.8 && impactState().player.vz < -5');desktop.keyboard.up('KeyW');desktop.keyboard.press('KeyQ');assert desktop.evaluate('impactState().weapon')=='gun';desktop.keyboard.press('KeyE');assert desktop.evaluate('impactState().slow');desktop.screenshot(path='/tmp/impact-desktop.png')
  desktop.keyboard.press('Escape');desktop.wait_for_function('!impactState().running')
  assert not errors,errors
  print('Passed real WASM loading, bat launch and blaster hit, simultaneous touch movement/jump/dash, slow motion, pause/reset, fullscreen, touch aiming, landscape, desktop mouse attack and running jump; no browser errors.')
  browser.close()
finally:server.shutdown();server.server_close()
