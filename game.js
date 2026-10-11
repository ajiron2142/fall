/* Pure generators are also exported for validation with Node. */
function shuffle(items, random = Math.random) {
  const out = [...items];
  for (let i = out.length - 1; i > 0; i--) {
    const j = Math.floor(random() * (i + 1));
    [out[i], out[j]] = [out[j], out[i]];
  }
  return out;
}
function makeSignal(random = Math.random) {
  const symbols = ['△', '○', '◇', '☆', '□', '⌁'];
  return { buttons: shuffle(symbols, random), sequence: shuffle(symbols, random).slice(0, 4) };
}
function makeMap(random = Math.random) {
  // Random depth-first search creates a simple, guaranteed walkable route.
  const route = [0], visited = new Set([0]);
  function search(pos) {
    if (pos === 24) return true;
    const x = pos % 5, y = Math.floor(pos / 5);
    const next = [];
    if (x > 0) next.push(pos - 1);
    if (x < 4) next.push(pos + 1);
    if (y > 0) next.push(pos - 5);
    if (y < 4) next.push(pos + 5);
    for (const n of shuffle(next, random)) {
      if (visited.has(n)) continue;
      visited.add(n); route.push(n);
      if (search(n)) return true;
      route.pop();
    }
    return false;
  }
  search(0);
  const open = new Set(route);
  for (let i = 0; i < 25; i++) if (random() < .22) open.add(i);
  return { route, open };
}
if (typeof module !== 'undefined') module.exports = { makeSignal, makeMap };
if (typeof document !== 'undefined') {
  const $ = id => document.getElementById(id);
  let active = 'signal', frame = 0, generation = 0;
  const names = { signal: ['CLUES + COMMUNICATION', 'Signal Room', 'Your partner knows the code. You control the panel. Enter four symbols in order.'], map: ['EXPLORATION + TRUST', 'Two Sides', 'You move. Your partner has the map. Reach the beacon in the bottom-right corner.'], pulse: ['TIMING + TEAMWORK', 'In Sync', 'Your simulated partner opens a green window. Tap when the orange marker is inside it. Land five hits.'] };
  function start() {
    cancelAnimationFrame(frame); generation++;
    const [tag,title,intro] = names[active];
    $('tag').textContent=tag; $('game-title').textContent=title; $('intro').textContent=intro;
    $('board').replaceChildren(); $('status').textContent='';
    document.querySelectorAll('[data-game]').forEach(b=>b.classList.toggle('selected',b.dataset.game===active));
    ({signal:signalGame,map:mapGame,pulse:pulseGame})[active]();
  }
  function signalGame() {
    const {buttons,sequence}=makeSignal(); let entered=0, peeking=false, won=false;
    $('board').innerHTML='<button class="full" id="peek">Peek at partner’s clue</button><div class="clue" id="clue"></div><div class="progress" id="progress"></div><div class="symbols" id="symbols"></div>';
    function clue() { $('clue').innerHTML=peeking ? '<strong>PARTNER’S SCREEN</strong>'+sequence.join(' → ') : '<strong>YOUR SCREEN</strong>The sequence is hidden. Ask your partner for the next symbol.'; }
    function progress() { $('progress').innerHTML=sequence.map((_,i)=>'<span class="dot '+(i<entered?'done':'')+'"></span>').join(''); }
    $('peek').onclick=()=>{peeking=!peeking;$('peek').textContent=peeking?'Back to your panel':'Peek at partner’s clue';clue();};
    buttons.forEach(symbol=>{const b=document.createElement('button');b.className='symbol';b.textContent=symbol;b.setAttribute('aria-label','Enter '+symbol);b.onclick=()=>{if(won)return;if(symbol===sequence[entered]){entered++;$('status').textContent=entered===4?'Door unlocked. You did it together!':'Correct. What comes next?';if(entered===4){won=true;document.querySelectorAll('.symbol').forEach(x=>x.disabled=true);}}else{entered=0;$('status').textContent='The panel reset. Check the sequence and try again.';}progress();};$('symbols').append(b);});
    clue();progress();
  }
  function mapGame() {
    const {open}=makeMap();let pos=0,peeking=false,steps=0,won=false;const trail=new Set([0]);
    $('board').innerHTML='<button class="full" id="peek">Peek at partner’s map</button><div class="grid" id="grid" aria-label="Island map"></div><div class="directions"><button data-dir="-5" aria-label="Move north">↑</button><button data-dir="-1" aria-label="Move west">←</button><button data-dir="5" aria-label="Move south">↓</button><button data-dir="1" aria-label="Move east">→</button></div>';
    function draw(){ $('grid').replaceChildren();for(let i=0;i<25;i++){const tile=document.createElement('div');tile.className='tile'+(peeking&&!open.has(i)?' wall':'')+(trail.has(i)?' trail':'')+(i===pos?' player':'')+(i===24?' goal':'');tile.textContent=i===pos?'●':i===24?'✦':peeking?(open.has(i)?'·':''):trail.has(i)?'·':'?';$('grid').append(tile);} }
    $('peek').onclick=()=>{peeking=!peeking;$('peek').textContent=peeking?'Back to your view':'Peek at partner’s map';draw();};
    document.querySelectorAll('[data-dir]').forEach(b=>b.onclick=()=>{if(won)return;const d=Number(b.dataset.dir),next=pos+d;if(next<0||next>24||(d===1&&pos%5===4)||(d===-1&&pos%5===0))return;if(!open.has(next)){$('status').textContent='Blocked. Your partner can see a way around.';return;}pos=next;steps++;trail.add(pos);if(pos===24){won=true;$('status').textContent='Beacon found in '+steps+' moves!';document.querySelectorAll('[data-dir]').forEach(x=>x.disabled=true);}else $('status').textContent=steps+' moves · Keep guiding each other.';draw();});draw();
  }
  function pulseGame() {
    const current=generation;let hits=0,misses=0,center=.5,speed=.00038,offset=Math.random(),position=0,lastTap=-Infinity;const width=.22;
    $('board').innerHTML='<div class="readout"><span id="score">0 / 5 connected</span><span id="misses">0 misses</span></div><div class="track"><div class="zone" id="zone"></div><div class="needle" id="needle"></div></div><button class="hit" id="hit">Sync!</button><div class="clue"><strong>SIMULATED PARTNER</strong>I’ll move the window after each hit. Watch the rhythm, then tap.</div>';
    function target(){center=.2+Math.random()*.6;$('zone').style.left=(center-width/2)*100+'%';$('zone').style.width=width*100+'%';}target();
    function tick(now){if(current!==generation)return;const phase=(now*speed+offset)%2;position=phase<=1?phase:2-phase;$('needle').style.left='calc('+position*100+'% - 2px)';frame=requestAnimationFrame(tick);}frame=requestAnimationFrame(tick);
    $('hit').onclick=()=>{const now=performance.now();if(hits>=5||now-lastTap<250)return;lastTap=now;if(Math.abs(position-center)<=width/2){hits++;$('score').textContent=hits+' / 5 connected';if(hits===5){cancelAnimationFrame(frame);$('hit').disabled=true;$('status').textContent='In sync! Five connections, '+misses+' misses.';}else{$('status').textContent='Connected! Your partner is moving the window.';speed+=.000025;target();}}else{misses++;$('misses').textContent=misses+' misses';$('status').textContent='Just missed. Wait for the marker to return.';}};
  }
  document.querySelectorAll('[data-game]').forEach(b=>b.onclick=()=>{active=b.dataset.game;start();});
  $('reset').onclick=start;start();
}
