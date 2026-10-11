//! Dependency-free first-person movement and toy-body physics.
//! The browser supplies inputs and renders a read-only snapshot.
use std::cell::RefCell;

const COUNT: usize = 10;
const STRIDE: usize = 9;
const HEADER: usize = 16;
const SNAPSHOT: usize = HEADER + COUNT * STRIDE;

#[derive(Clone, Copy, Debug, Default)]
struct V {
    x: f32,
    y: f32,
    z: f32,
}
impl V {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    fn add(self, b: Self) -> Self {
        Self::new(self.x + b.x, self.y + b.y, self.z + b.z)
    }
    fn sub(self, b: Self) -> Self {
        Self::new(self.x - b.x, self.y - b.y, self.z - b.z)
    }
    fn scale(self, a: f32) -> Self {
        Self::new(self.x * a, self.y * a, self.z * a)
    }
    fn dot(self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn unit(self) -> Self {
        self.scale(1.0 / self.length().max(0.0001))
    }
}
#[derive(Clone, Copy)]
struct Block {
    x: f32,
    z: f32,
    w: f32,
    d: f32,
    h: f32,
}
const BLOCKS: [Block; 5] = [
    Block {
        x: -6.0,
        z: 0.0,
        w: 4.0,
        d: 4.0,
        h: 1.3,
    },
    Block {
        x: 6.0,
        z: -4.0,
        w: 4.0,
        d: 4.0,
        h: 2.0,
    },
    Block {
        x: -11.0,
        z: -8.0,
        w: 3.0,
        d: 6.0,
        h: 2.8,
    },
    Block {
        x: 0.0,
        z: -13.0,
        w: 7.0,
        d: 2.0,
        h: 0.9,
    },
    Block {
        x: 11.0,
        z: 7.0,
        w: 4.0,
        d: 4.0,
        h: 1.1,
    },
];
#[derive(Clone, Copy, Default)]
struct Body {
    p: V,
    v: V,
    home: V,
    r: f32,
    kind: u32,
    glow: f32,
    scored: f32,
}
struct Game {
    p: V,
    v: V,
    grounded: bool,
    coyote: f32,
    jump_buffer: f32,
    dash_cd: f32,
    dash_time: f32,
    attack_cd: f32,
    bodies: [Body; COUNT],
    score: u32,
    hits: u32,
    shots: u32,
    combo: u32,
    best: u32,
    combo_time: f32,
    time: f32,
    hoops: u32,
    slow: bool,
    last_hit: i32,
    last_distance: f32,
}
impl Game {
    fn new(seed: u32) -> Self {
        let positions = [
            V::new(0.0, 0.6, 7.3),
            V::new(-4.0, 0.6, 4.0),
            V::new(4.0, 0.6, 4.0),
            V::new(0.0, 0.6, -3.0),
            V::new(-8.0, 0.6, 8.0),
            V::new(8.0, 0.6, 8.0),
            V::new(-6.0, 2.6, -7.0),
            V::new(0.0, 3.1, -8.0),
            V::new(6.0, 2.6, -7.0),
            V::new(-1.0, 4.0, -11.0),
        ];
        let mut bodies = [Body::default(); COUNT];
        for (i, p) in positions.into_iter().enumerate() {
            let offset = if i == 0 {
                0.0
            } else {
                ((seed
                    .wrapping_mul(1664525)
                    .wrapping_add((i as u32).wrapping_mul(1013904223)))
                    % 100) as f32
                    / 250.0
                    - 0.2
            };
            let p = p.add(V::new(offset, 0.0, 0.0));
            bodies[i] = Body {
                p,
                home: p,
                r: if i < 6 { 0.6 } else { 0.48 },
                kind: if i < 6 { 0 } else { 1 },
                ..Body::default()
            };
        }
        Self {
            p: V::new(0.0, 0.0, 10.0),
            v: V::default(),
            grounded: true,
            coyote: 0.14,
            jump_buffer: 0.0,
            dash_cd: 0.0,
            dash_time: 0.0,
            attack_cd: 0.0,
            bodies,
            score: 0,
            hits: 0,
            shots: 0,
            combo: 0,
            best: 0,
            combo_time: 0.0,
            time: 0.0,
            hoops: 0,
            slow: false,
            last_hit: -1,
            last_distance: 35.0,
        }
    }
    fn step(
        &mut self,
        dt: f32,
        forward: f32,
        side: f32,
        yaw: f32,
        jump: bool,
        dash: bool,
        slow: bool,
    ) {
        let dt = dt.clamp(0.0, 1.0 / 60.0);
        if !dt.is_finite() || !yaw.is_finite() {
            return;
        }
        self.slow = slow;
        self.time += dt;
        self.attack_cd = (self.attack_cd - dt).max(0.0);
        self.dash_cd = (self.dash_cd - dt).max(0.0);
        self.dash_time = (self.dash_time - dt).max(0.0);
        self.combo_time = (self.combo_time - dt).max(0.0);
        if self.combo_time == 0.0 {
            self.combo = 0;
        }
        self.coyote = if self.grounded {
            0.14
        } else {
            (self.coyote - dt).max(0.0)
        };
        self.jump_buffer = if jump {
            0.18
        } else {
            (self.jump_buffer - dt).max(0.0)
        };
        let pdt = dt * if slow { 0.8 } else { 1.0 };
        let f = if forward.is_finite() {
            forward.clamp(-1.0, 1.0)
        } else {
            0.0
        };
        let s = if side.is_finite() {
            side.clamp(-1.0, 1.0)
        } else {
            0.0
        };
        let len = (f * f + s * s).sqrt().max(1.0);
        let wish = V::new(
            (-yaw.sin() * f + yaw.cos() * s) / len,
            0.0,
            (-yaw.cos() * f - yaw.sin() * s) / len,
        );
        if self.dash_time <= 0.0 {
            if self.grounded {
                let blend = 1.0 - (-18.0 * pdt).exp();
                self.v.x += (wish.x * 15.0 - self.v.x) * blend;
                self.v.z += (wish.z * 15.0 - self.v.z) * blend;
            } else if wish.length() > 0.01 {
                // Accelerate along the requested direction without deleting jump momentum.
                let remaining = (15.0 - self.v.dot(wish)).max(0.0);
                let acceleration = remaining.min(32.0 * pdt);
                self.v = self.v.add(wish.scale(acceleration));
                let horizontal = (self.v.x * self.v.x + self.v.z * self.v.z).sqrt();
                if horizontal > 32.0 {
                    self.v.x *= 32.0 / horizontal;
                    self.v.z *= 32.0 / horizontal;
                }
            }
        }
        if self.jump_buffer > 0.0 && self.coyote > 0.0 {
            self.v.y = 10.5;
            self.grounded = false;
            self.coyote = 0.0;
            self.jump_buffer = 0.0;
        }
        if dash && self.dash_cd <= 0.0 {
            let dir = if wish.length() > 0.1 {
                wish.unit()
            } else {
                V::new(-yaw.sin(), 0.0, -yaw.cos())
            };
            self.v.x = dir.x * 30.0;
            self.v.z = dir.z * 30.0;
            self.dash_time = 0.16;
            self.dash_cd = 0.55;
        }
        let old_y = self.p.y;
        self.v.y -= 25.0 * pdt;
        self.p = self.p.add(self.v.scale(pdt));
        self.grounded = false;
        if self.p.y <= 0.0 {
            self.p.y = 0.0;
            self.v.y = 0.0;
            self.grounded = true;
        }
        for b in BLOCKS {
            if (self.p.x - b.x).abs() < b.w / 2.0 + 0.32
                && (self.p.z - b.z).abs() < b.d / 2.0 + 0.32
            {
                if old_y >= b.h - 0.03 && self.p.y <= b.h && self.v.y <= 0.0 {
                    self.p.y = b.h;
                    self.v.y = 0.0;
                    self.grounded = true;
                } else if self.p.y < b.h && self.p.y + 1.7 > 0.0 {
                    push_box(&mut self.p, &mut self.v, b, 0.32);
                }
            }
        }
        for axis in [0, 2] {
            if axis == 0 && self.p.x.abs() > 17.5 {
                self.p.x = self.p.x.clamp(-17.5, 17.5);
                self.v.x = 0.0;
            }
            if axis == 2 && self.p.z.abs() > 17.5 {
                self.p.z = self.p.z.clamp(-17.5, 17.5);
                self.v.z = 0.0;
            }
        }
        let wdt = dt * if slow { 0.24 } else { 1.0 };
        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.glow = (body.glow - dt * 2.0).max(0.0);
            body.scored = (body.scored - dt).max(0.0);
            let old = body.p;
            if body.kind == 0 {
                body.v.y -= 18.0 * wdt;
            } else {
                let home = body.home.add(V::new(
                    (self.time * 0.6 + i as f32).sin() * 1.0,
                    (self.time * 1.2 + i as f32).sin() * 0.25,
                    0.0,
                ));
                let spring = home.sub(body.p).scale(3.5);
                body.v = body.v.add(spring.scale(wdt));
            }
            body.v = body
                .v
                .scale((-(if body.kind == 0 { 0.32 } else { 0.8 }) * wdt).exp());
            body.p = body.p.add(body.v.scale(wdt));
            if body.p.y < body.r {
                body.p.y = body.r;
                if body.v.y < 0.0 {
                    body.v.y *= -0.65;
                }
                if body.v.y.abs() < 0.4 {
                    body.v.y = 0.0;
                }
                body.v.x *= (-1.5 * wdt).exp();
                body.v.z *= (-1.5 * wdt).exp();
            }
            for b in BLOCKS {
                if body.p.y - body.r < b.h
                    && (body.p.x - b.x).abs() < b.w / 2.0 + body.r
                    && (body.p.z - b.z).abs() < b.d / 2.0 + body.r
                {
                    if old.y - body.r >= b.h - 0.02 && body.v.y < 0.0 {
                        body.p.y = b.h + body.r;
                        body.v.y *= -0.55;
                    } else {
                        push_box(&mut body.p, &mut body.v, b, body.r);
                    }
                }
            }
            for axis in [0, 2] {
                let (p, v) = if axis == 0 {
                    (&mut body.p.x, &mut body.v.x)
                } else {
                    (&mut body.p.z, &mut body.v.z)
                };
                let edge = 17.7 - body.r;
                if p.abs() > edge {
                    *p = p.clamp(-edge, edge);
                    *v *= -0.7;
                }
            }
            if body.p.y > 0.0
                && body.p.y < 2.0
                && (body.p.x - self.p.x).hypot(body.p.z - self.p.z) < body.r + 0.35
            {
                let delta = V::new(body.p.x - self.p.x, 0.0, body.p.z - self.p.z).unit();
                body.v = body.v.add(delta.scale(8.0 * wdt));
            }
            for hx in [-12.0_f32, 0.0, 12.0] {
                if body.scored <= 0.0 && old.z > -15.7 && body.p.z <= -15.7 {
                    let fraction = ((old.z + 15.7) / (old.z - body.p.z)).clamp(0.0, 1.0);
                    let cross = old.add(body.p.sub(old).scale(fraction));
                    if (cross.x - hx).hypot(cross.y - 2.6) < 1.8 {
                        self.score += 75;
                        self.hoops += 1;
                        body.scored = 1.5;
                        body.glow = 1.0;
                    }
                }
            }
        }
        // Equal-mass sphere contacts let a bat strike produce a chain reaction.
        for a in 0..COUNT {
            for b in a + 1..COUNT {
                let delta = self.bodies[b].p.sub(self.bodies[a].p);
                let distance = delta.length();
                let radius = self.bodies[a].r + self.bodies[b].r;
                if distance > 0.001 && distance < radius {
                    let n = delta.scale(1.0 / distance);
                    let push = n.scale((radius - distance) * 0.5);
                    self.bodies[a].p = self.bodies[a].p.sub(push);
                    self.bodies[b].p = self.bodies[b].p.add(push);
                    let closing = self.bodies[b].v.sub(self.bodies[a].v).dot(n);
                    if closing < 0.0 {
                        let impulse = n.scale(-closing * 0.8);
                        self.bodies[a].v = self.bodies[a].v.sub(impulse);
                        self.bodies[b].v = self.bodies[b].v.add(impulse);
                    }
                }
            }
        }
    }
    fn attack(&mut self, yaw: f32, pitch: f32, weapon: u32) -> i32 {
        if self.attack_cd > 0.0 || !yaw.is_finite() || !pitch.is_finite() {
            return -2;
        }
        self.attack_cd = if weapon == 0 { 0.32 } else { 0.13 };
        self.shots += 1;
        self.last_hit = -1;
        self.last_distance = 35.0;
        let eye = self.p.add(V::new(0.0, 1.65, 0.0));
        let dir = V::new(
            -yaw.sin() * pitch.cos(),
            pitch.sin(),
            -yaw.cos() * pitch.cos(),
        );
        let wall_distance = BLOCKS
            .iter()
            .map(|b| ray_box(eye, dir, *b))
            .fold(35.0, f32::min);
        if weapon == 0 {
            let mut count = 0;
            for (i, b) in self.bodies.iter_mut().enumerate() {
                let delta = b.p.sub(eye);
                let dist = delta.length();
                if dist < 3.7 + b.r
                    && delta.unit().dot(dir) > 0.62
                    && BLOCKS
                        .iter()
                        .all(|block| ray_box(eye, delta.unit(), *block) > dist - b.r)
                {
                    // A driving sideways strike: forward power, modest lift, carried momentum.
                    let carry = b.v.scale(0.2).add(self.v.scale(0.45));
                    b.v = V::new(-yaw.sin(), 0.0, -yaw.cos()).scale(32.0).add(carry);
                    b.v.y = 3.2 + pitch.max(0.0) * 16.0 + self.v.y.max(0.0) * 0.2;
                    b.glow = 1.0;
                    count += 1;
                    self.last_hit = i as i32;
                    self.last_distance = dist;
                }
            }
            if count > 0 {
                self.reward(count, 12);
            }
            return count as i32;
        }
        let mut nearest = wall_distance;
        let mut hit = None;
        for (i, b) in self.bodies.iter().enumerate() {
            let oc = eye.sub(b.p);
            let d = oc.dot(dir);
            let c = oc.dot(oc) - (b.r + 0.12).powi(2);
            let disc = d * d - c;
            if disc >= 0.0 {
                let t = -d - disc.sqrt();
                if t >= 0.0 && t < nearest {
                    nearest = t;
                    hit = Some(i);
                }
            }
        }
        self.last_distance = nearest;
        if let Some(i) = hit {
            let b = &mut self.bodies[i];
            b.v = b.v.add(dir.scale(10.0));
            b.v.y += 2.5;
            b.glow = 1.0;
            self.last_hit = i as i32;
            self.reward(1, 6);
            1
        } else {
            0
        }
    }
    fn reward(&mut self, count: u32, base: u32) {
        self.hits += count;
        self.combo += count;
        self.best = self.best.max(self.combo);
        self.combo_time = 3.0;
        self.score += base * count + self.combo.min(10) * 2;
    }
    fn snapshot(&self, out: &mut [f32; SNAPSHOT]) {
        out[..HEADER].copy_from_slice(&[
            self.p.x,
            self.p.y,
            self.p.z,
            self.v.x,
            self.v.y,
            self.v.z,
            if self.grounded { 1.0 } else { 0.0 },
            self.dash_cd,
            self.score as f32,
            self.hits as f32,
            self.shots as f32,
            self.best as f32,
            self.combo as f32,
            self.attack_cd,
            self.time,
            self.hoops as f32,
        ]);
        for (i, b) in self.bodies.iter().enumerate() {
            let start = HEADER + i * STRIDE;
            out[start..start + STRIDE].copy_from_slice(&[
                b.p.x,
                b.p.y,
                b.p.z,
                b.r,
                b.kind as f32,
                b.glow,
                b.v.x,
                b.v.y,
                b.v.z,
            ]);
        }
    }
}
fn push_box(p: &mut V, v: &mut V, b: Block, r: f32) {
    let dx = b.w / 2.0 + r - (p.x - b.x).abs();
    let dz = b.d / 2.0 + r - (p.z - b.z).abs();
    if dx < dz {
        let sign = if p.x >= b.x { 1.0 } else { -1.0 };
        p.x = b.x + sign * (b.w / 2.0 + r);
        v.x *= -0.2;
    } else {
        let sign = if p.z >= b.z { 1.0 } else { -1.0 };
        p.z = b.z + sign * (b.d / 2.0 + r);
        v.z *= -0.2;
    }
}
fn ray_box(origin: V, dir: V, b: Block) -> f32 {
    let mut near = 0.0_f32;
    let mut far = 100.0_f32;
    for (o, d, min, max) in [
        (origin.x, dir.x, b.x - b.w / 2.0, b.x + b.w / 2.0),
        (origin.y, dir.y, 0.0, b.h),
        (origin.z, dir.z, b.z - b.d / 2.0, b.z + b.d / 2.0),
    ] {
        if d.abs() < 0.00001 {
            if o < min || o > max {
                return f32::INFINITY;
            }
        } else {
            let a = (min - o) / d;
            let c = (max - o) / d;
            near = near.max(a.min(c));
            far = far.min(a.max(c));
            if near > far {
                return f32::INFINITY;
            }
        }
    }
    near
}

thread_local! {static GAME:RefCell<Game>=RefCell::new(Game::new(1));static BUFFER:RefCell<[f32;SNAPSHOT]>=const{RefCell::new([0.0;SNAPSHOT])};}
#[no_mangle]
pub extern "C" fn reset(seed: u32) {
    GAME.with(|g| *g.borrow_mut() = Game::new(seed));
}
#[no_mangle]
pub extern "C" fn tick(
    dt: f32,
    forward: f32,
    side: f32,
    yaw: f32,
    jump: u32,
    dash: u32,
    slow: u32,
) {
    GAME.with(|g| {
        g.borrow_mut()
            .step(dt, forward, side, yaw, jump != 0, dash != 0, slow != 0)
    });
}
#[no_mangle]
pub extern "C" fn attack(yaw: f32, pitch: f32, weapon: u32) -> i32 {
    GAME.with(|g| g.borrow_mut().attack(yaw, pitch, weapon))
}
#[no_mangle]
pub extern "C" fn snapshot_ptr() -> *const f32 {
    GAME.with(|g| {
        BUFFER.with(|b| {
            let mut out = b.borrow_mut();
            g.borrow().snapshot(&mut out);
            out.as_ptr()
        })
    })
}
#[no_mangle]
pub extern "C" fn snapshot_len() -> usize {
    SNAPSHOT
}
#[no_mangle]
pub extern "C" fn last_hit() -> i32 {
    GAME.with(|g| g.borrow().last_hit)
}
#[no_mangle]
pub extern "C" fn last_distance() -> f32 {
    GAME.with(|g| g.borrow().last_distance)
}
#[no_mangle]
pub extern "C" fn obstacle_count() -> usize {
    BLOCKS.len()
}
#[no_mangle]
pub extern "C" fn obstacle(index: usize, field: usize) -> f32 {
    BLOCKS
        .get(index)
        .map(|b| match field {
            0 => b.x,
            1 => b.z,
            2 => b.w,
            3 => b.d,
            4 => b.h,
            _ => 0.0,
        })
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run(g: &mut Game, seconds: f32, f: f32, s: f32) {
        for _ in 0..(seconds * 120.0) as usize {
            g.step(1.0 / 120.0, f, s, 0.0, false, false, false);
        }
    }
    #[test]
    fn movement_jump_dash_and_solid_floor() {
        let mut g = Game::new(1);
        run(&mut g, 0.3, 1.0, 0.0);
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, true, false, false);
        let z = g.p.z;
        run(&mut g, 0.15, 1.0, 0.0);
        assert!(g.p.y > 0.8 && g.p.z < z - 0.7);
        run(&mut g, 1.0, 0.0, 0.0);
        assert!(g.grounded);
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, true, false);
        assert!(g.v.z < -24.0 && g.dash_cd > 0.0);
    }
    #[test]
    fn bat_hits_front_ball_and_launches_it() {
        let mut g = Game::new(1);
        assert_eq!(g.attack(0.0, -0.15, 0), 1);
        assert!(
            g.bodies[0].v.z < -20.0
                && g.bodies[0].v.y > 0.0
                && g.bodies[0].v.y < 4.0
                && g.score > 0
        );
        assert_eq!(g.attack(0.0, 0.0, 0), -2);
    }
    #[test]
    fn jump_preserves_dash_momentum_without_input() {
        let mut g = Game::new(1);
        g.step(0.01, 1.0, 0.0, 0.0, true, true, false);
        run(&mut g, 0.25, 0.0, 0.0);
        assert!(!g.grounded && g.v.z < -29.0);
    }
    #[test]
    fn moving_swing_carries_player_momentum() {
        let mut still = Game::new(1);
        let mut moving = Game::new(1);
        moving.v.z = -15.0;
        still.attack(0.0, 0.0, 0);
        moving.attack(0.0, 0.0, 0);
        assert!(moving.bodies[0].v.z < still.bodies[0].v.z - 6.0);
    }
    #[test]
    fn gun_hits_aimed_target_and_misses_away() {
        let mut g = Game::new(1);
        let delta = g.bodies[0].p.sub(g.p.add(V::new(0.0, 1.65, 0.0)));
        let pitch = (delta.y / delta.length()).asin();
        assert_eq!(g.attack(0.0, pitch, 1), 1);
        run(&mut g, 0.2, 0.0, 0.0);
        assert_eq!(g.attack(std::f32::consts::PI, 0.0, 1), 0);
    }
    #[test]
    fn wall_blocks_a_shot() {
        let mut g = Game::new(2);
        g.p = V::new(6.0, 0.0, 0.0);
        for b in &mut g.bodies {
            b.p = V::new(15.0, 0.6, 14.0);
        }
        g.bodies[0].p = V::new(6.0, 1.65, -8.0);
        assert_eq!(g.attack(0.0, 0.0, 1), 0);
    }
    #[test]
    fn slow_motion_reduces_object_travel() {
        let mut normal = Game::new(3);
        let mut slow = Game::new(3);
        normal.bodies[0].v.x = 8.0;
        slow.bodies[0].v.x = 8.0;
        for _ in 0..60 {
            normal.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, false);
            slow.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, true);
        }
        assert!(normal.bodies[0].p.x > slow.bodies[0].p.x * 2.0);
    }
    #[test]
    fn hoop_awards_points_when_ball_crosses_ring() {
        let mut g = Game::new(1);
        g.bodies[0].p = V::new(12.0, 2.6, -15.6);
        g.bodies[0].v = V::new(0.0, 0.0, -20.0);
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, false);
        assert_eq!(g.hoops, 1);
        assert_eq!(g.score, 75);
    }
    #[test]
    fn reset_snapshot_and_seed_are_reproducible() {
        let a = Game::new(42);
        let b = Game::new(42);
        let mut x = [0.0; SNAPSHOT];
        let mut y = [0.0; SNAPSHOT];
        a.snapshot(&mut x);
        b.snapshot(&mut y);
        assert_eq!(x, y);
        assert_eq!(x.len(), 106);
    }
    #[test]
    fn long_simulation_stays_finite_and_inside_bounds() {
        let mut g = Game::new(4);
        for i in 0..5000 {
            g.step(
                1.0 / 120.0,
                1.0,
                0.5,
                i as f32 * 0.01,
                i % 100 == 0,
                i % 80 == 0,
                i % 500 < 200,
            );
            if i % 60 == 0 {
                g.attack(i as f32 * 0.01, -0.2, 0);
            }
        }
        assert!(g.p.x.abs() <= 17.5 && g.p.z.abs() <= 17.5);
        for b in g.bodies {
            assert!(b.p.x.is_finite() && b.p.y.is_finite() && b.p.z.is_finite());
        }
    }
}
