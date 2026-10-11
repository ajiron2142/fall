import functools
import shutil
import threading
from pathlib import Path
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
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
  source=(ROOT/'worlds.mjs').read_text()
  hook='''
window.testProject=(x,y,z)=>{const p=new THREE.Vector3(x,y,z).project(camera),r=canvas.getBoundingClientRect();return {x:r.left+(p.x+1)*r.width/2,y:r.top+(1-p.y)*r.height/2};};
window.testMoving=()=>!!objects.walking?.route.length;
'''
  page.route('**/worlds.mjs',lambda route:route.fulfill(body=source+hook,content_type='text/javascript'))
  page.goto(base);page.wait_for_function('window.littleWorldsState && window.testProject')
  state=page.evaluate('littleWorldsState()')
  for i,v in enumerate(state['solution']):
   if state['folds'][i]!=v:page.locator('[data-fold="'+str(i)+'"]').tap()
  page.wait_for_timeout(750)
  point=page.evaluate('testProject(-1,.54,2)');page.touchscreen.tap(point['x'],point['y'])
  assert page.evaluate('littleWorldsState().pawn.z')==2, page.evaluate('littleWorldsState()')
  page.wait_for_function('!testMoving()')
  page.screenshot(path='/tmp/little-worlds-fold.png',full_page=True)
  for _ in range(5):
   if page.evaluate('littleWorldsState().won'):break
   page.locator('#walk').tap()
   page.wait_for_function('!testMoving()')
  assert page.evaluate('littleWorldsState().won');assert page.locator('#victory').is_visible()
  page.locator('#again').tap();assert not page.evaluate('littleWorldsState().won')
  before=page.evaluate('littleWorldsState().folds');page.locator('[data-fold="0"]').tap();page.locator('#undo').tap();assert before==page.evaluate('littleWorldsState().folds')
  page.locator('[data-game="rooms"]').tap()
  state=page.evaluate('littleWorldsState()')
  for action in reversed(state['scramble']):page.locator('[data-row="'+str(action['row'])+'"][data-direction="'+str(-action['direction'])+'"]').tap()
  for _ in range(3):
   page.locator('#room-walk').tap();page.wait_for_function('!testMoving()')
  assert page.evaluate('littleWorldsState().pawns.A')==4
  page.locator('#pawn-B').tap();page.locator('#room-walk').tap();page.wait_for_function('!testMoving()');assert page.evaluate('littleWorldsState().won')
  page.locator('#again').tap();page.screenshot(path='/tmp/little-worlds-rooms.png',full_page=True)
  page.locator('#room-hint').tap();assert page.locator('#toast').inner_text()
  page.locator('[data-game="gravity"]').tap();page.screenshot(path='/tmp/little-worlds-gravity.png',full_page=True)
  cdp=page.context.new_cdp_session(page);r=page.locator('#tilt').bounding_box();x=r['x']+r['width']/2;y=r['y']+r['height']/2
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y,'id':1}]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchMove','touchPoints':[{'x':x+20,'y':y-20,'id':1}]})
  before=page.evaluate('littleWorldsState().ball')
  gate=page.locator('#gate').bounding_box();gx=gate['x']+gate['width']/2;gy=gate['y']+gate['height']/2
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x+20,'y':y-20,'id':1},{'x':gx,'y':gy,'id':2}]})
  assert page.evaluate('littleWorldsState().gateOpen')
  assert page.evaluate('littleWorldsState().tilt.x')>0
  page.wait_for_function('(b)=>Math.hypot(littleWorldsState().ball.x-b.x,littleWorldsState().ball.z-b.z)>.1',arg=before)
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[{'x':x+20,'y':y-20,'id':1}]})
  brake=page.locator('#brake').bounding_box();bx=brake['x']+brake['width']/2;by=brake['y']+brake['height']/2
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x+20,'y':y-20,'id':1},{'x':bx,'y':by,'id':3}]})
  assert page.evaluate('littleWorldsState().brake');assert page.evaluate('littleWorldsState().tilt.x')>0
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  assert not page.evaluate('littleWorldsState().brake');assert page.evaluate('littleWorldsState().tilt.x')==0
  page.locator('#fullscreen').tap();page.wait_for_function('!!document.fullscreenElement');page.evaluate('document.exitFullscreen()')
  page.set_viewport_size({'width':844,'height':390});page.wait_for_timeout(300)
  assert page.evaluate('document.documentElement.scrollWidth<=innerWidth')
  desktop=browser.new_page(viewport={'width':1200,'height':850});desktop.on('pageerror',lambda e:errors.append(str(e)));desktop.goto(base);desktop.wait_for_function('window.littleWorldsState');desktop.locator('[data-game="gravity"]').click();desktop.keyboard.down('ArrowRight');desktop.wait_for_function('littleWorldsState().tilt.x===1');desktop.keyboard.up('ArrowRight');assert desktop.evaluate('littleWorldsState().tilt.x')==0
  desktop.screenshot(path='/tmp/little-worlds-desktop.png',full_page=True)
  assert not errors,errors
  print('Passed mobile canvas tap, folding, undo, both puzzle win screens, simultaneous tilt/gate/brake, release cleanup, fullscreen, landscape layout and desktop keyboard; no browser errors.')
  browser.close()
finally:server.shutdown();server.server_close()
