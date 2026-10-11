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
  assert page.evaluate('impactState().weapon')=='shotgun'
  assert page.locator('#controls').is_hidden()
  page.locator('#touch-controls').tap()
  assert page.locator('#controls').is_visible()
  page.reload();page.wait_for_function('window.impactState && impactState().ready')
  assert page.locator('#controls').is_visible()
  page.locator('#touch-controls').tap()
  assert page.locator('#controls').is_hidden()
  page.locator('#touch-controls').tap()
  page.locator('#start').tap();page.locator('#weapon').tap();
  initial=page.evaluate('''()=>{const before=impactState().shots;document.getElementById('fire').dispatchEvent(new MouseEvent('click',{detail:0}));return {before,after:impactState().shots,pending:impactState().batPending};}''');assert initial['before']==initial['after'] and initial['pending'],initial
  page.wait_for_function('impactState().hits>0');assert page.evaluate('impactState().bodies[0].vz')< -15
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
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y-30,'id':1}]})
  page.wait_for_function('!impactState().grounded && impactState().player.vy<0 && impactState().player.y>.4')
  height=page.evaluate('impactState().player.y')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y-30,'id':1},{'x':jx,'y':jy,'id':2}]})
  page.wait_for_function('(h)=>!impactState().airJumpAvailable && impactState().player.vy>4 && impactState().player.y>h',arg=height)
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  page.wait_for_function('impactState().grounded && impactState().airJumpAvailable')
  page.wait_for_function(r'/^\d+ FPS$/.test(document.getElementById("fps").textContent)')
  # Reset to a clear runway: the faster running jump can land near an obstacle.
  page.locator('#pause').tap();page.locator('#reset').tap();page.locator('#start').tap()
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y-30,'id':1}]})
  dr=page.locator('#dash').bounding_box();dx=dr['x']+dr['width']/2;dy=dr['y']+dr['height']/2
  cdp.send('Input.dispatchTouchEvent',{'type':'touchStart','touchPoints':[{'x':x,'y':y-30,'id':1},{'x':dx,'y':dy,'id':3}]})
  page.wait_for_function('Math.abs(impactState().player.vz)>20')
  cdp.send('Input.dispatchTouchEvent',{'type':'touchEnd','touchPoints':[]})
  page.locator('#slow').tap();assert page.evaluate('impactState().slow');page.locator('#slow').tap();assert not page.evaluate('impactState().slow')
  assert page.evaluate('impactState().weapon')=='shotgun'
  page.locator('#pause').tap();assert not page.evaluate('impactState().running')
  page.locator('#reset').tap();page.locator('#start').tap();page.locator('#slow').tap();page.locator('#fire').tap()
  page.wait_for_function('impactState().projectiles.length>0 && impactState().hits===0')
  page.screenshot(path='/tmp/impact-bullet-time.png')
  page.locator('#slow').tap();page.wait_for_function('impactState().hits>0');assert page.evaluate('impactState().shots')>=1
  assert not page.evaluate('impactState().slow')
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
  desktop=browser.new_page(viewport={'width':1280,'height':800});desktop.on('pageerror',lambda e:errors.append(str(e)));desktop.goto(base+'/arena.html');desktop.wait_for_function('window.impactState && impactState().ready');assert desktop.evaluate('impactState().weapon')=='shotgun';desktop.locator('#start').click();desktop.mouse.down();desktop.mouse.up();desktop.wait_for_function('impactState().hits>0')
  desktop.evaluate('''()=>{window.walkSamples=[];const sample=()=>{const s=impactState();walkSamples.push(s.camera.y-s.player.y);if(walkSamples.length<12)requestAnimationFrame(sample);};requestAnimationFrame(sample);}''');desktop.keyboard.down('KeyW');desktop.wait_for_function('walkSamples.length>=12 && impactState().player.z<9');assert desktop.evaluate('walkSamples.every(y=>Math.abs(y-1.65)<.00001)'),desktop.evaluate('walkSamples');desktop.keyboard.press('Space');desktop.wait_for_function('impactState().player.y>.8 && impactState().player.vz < -5');desktop.keyboard.up('KeyW');desktop.wait_for_function('!impactState().grounded && impactState().player.vy<0');desktop.keyboard.press('Space');desktop.wait_for_function('!impactState().airJumpAvailable && impactState().player.vy>4');desktop.keyboard.press('KeyQ');assert desktop.evaluate('impactState().weapon')=='bat';desktop.mouse.wheel(0,120);desktop.wait_for_function("impactState().weapon==='shotgun'");desktop.keyboard.press('KeyE');assert desktop.evaluate('impactState().slow');desktop.evaluate('''()=>{window.hoverSamples=[];const sample=()=>{hoverSamples.push(impactState().player.y);if(hoverSamples.length<8)requestAnimationFrame(sample);};requestAnimationFrame(sample);}''');desktop.wait_for_function('hoverSamples.length===8');assert desktop.evaluate('Math.abs(hoverSamples[7]-hoverSamples[0])<.35 && !impactState().grounded');desktop.screenshot(path='/tmp/impact-desktop.png')
  desktop.keyboard.press('Escape');desktop.wait_for_function('!impactState().running')
  desktop.locator('#start').click();desktop.keyboard.press('KeyR');desktop.keyboard.down('KeyW');desktop.wait_for_function('Math.hypot(impactState().player.vx,impactState().player.vz)>8');desktop.keyboard.press('Space');desktop.wait_for_function('!impactState().grounded && impactState().player.vy>1');desktop.keyboard.press('KeyE');desktop.keyboard.up('KeyW');desktop.keyboard.down('KeyD');desktop.keyboard.press('Space');desktop.wait_for_function('!impactState().airJumpAvailable && impactState().player.vx>5');desktop.keyboard.up('KeyD');state=desktop.evaluate('impactState()');assert state['slow'] and state['player']['vy']>4 and state['player']['vz']>0,state;desktop.keyboard.press('Escape')
  desktop.locator('#playground-3').click();desktop.locator('#start').click();desktop.keyboard.down('KeyW');desktop.wait_for_function('impactState().wallRunning');state=desktop.evaluate('impactState()');assert state['player']['y']>5 and state['player']['vz']<-17;desktop.keyboard.press('Space');desktop.wait_for_function('impactState().player.vx < -5 && impactState().player.vy>5');desktop.keyboard.up('KeyW');desktop.keyboard.press('Escape')
  page.locator('#pause').tap()
  for preset in range(3):
   page.locator('#preset-'+str(preset)).tap();state=page.evaluate('impactState()')
   assert state['movementPreset']==preset and state['courseProgress']==1 and state['player']['y']==state['obstacles'][5]['h']
   assert page.locator('#preset-'+str(preset)).get_attribute('aria-pressed')=='true'
  page.locator('#start').tap();page.locator('#jump').tap();page.wait_for_function('impactState().player.y>1.4')
  page.locator('#course').tap();assert page.evaluate('impactState().courseProgress')==1
  page.locator('#pause').tap();page.reload();page.wait_for_function('window.impactState && impactState().ready');assert page.evaluate('impactState().movementPreset')==2
  for playground in range(7):
   page.locator('#playground-'+str(playground)).tap();state=page.evaluate('impactState()');section=[b for b in state['obstacles'] if b['group']==playground]
   assert state['playground']==playground and state['courseProgress']==1 and len(section)==(10 if playground<3 else 3 if playground<6 else 7)
   assert sum(b['wall'] for b in section)==(0 if playground<3 else 1 if playground<6 else 3)
   assert page.locator('#playground-'+str(playground)).get_attribute('aria-pressed')=='true'
  page.locator('#playground-2').tap();page.locator('#preset-0').tap();page.locator('#start').tap();page.screenshot(path='/tmp/impact-course-mobile.png')
  page.locator('#runway').tap();state=page.evaluate('impactState()')
  assert state['arena']=={'width':160,'length':320} and state['runwayActive']
  assert state['player']['x']==-50 and state['player']['z']==140
  page.keyboard.down('KeyW');page.wait_for_function('impactState().player.z<139');page.keyboard.press('Space');page.wait_for_function('impactState().player.y>.5');page.keyboard.up('KeyW')
  page.screenshot(path='/tmp/impact-runway.png')
  assert not errors,errors
  print('Passed real WASM loading, bat launch and shotgun hit, simultaneous touch movement/jump/dash, bullet-time airborne suspension and delayed pellet hits after resume, pause/reset, fullscreen, touch aiming, landscape, desktop mouse attack, stable walking camera, touch/keyboard double jumps and FPS counter, mouse wheel switching and restored Q shortcut, default shotgun, three saved movement presets, seven difficulty-tiered playgrounds, automatic wall runs and wall kicks, expanded 160x320 arena and runway movement; no browser errors.')
  browser.close()
finally:server.shutdown();server.server_close()
