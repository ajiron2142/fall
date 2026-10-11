//! Dependency-free first-person movement and toy-body physics.
//! The browser supplies inputs and renders a read-only snapshot.
use std::cell::RefCell;

const COUNT: usize = 10;
const HALF_WIDTH: f32 = 80.0;
const HALF_LENGTH: f32 = 160.0;
const PELLETS: usize = 72;
const PELLET_SPEED: f32 = 45.0;
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
    base: f32,
}
const BLOCKS: [Block; 51] = [
    Block {
        x: -6.00,
        z: 0.00,
        w: 4.00,
        d: 4.00,
        h: 1.30,
        base: 0.00,
    },
    Block {
        x: 6.00,
        z: -4.00,
        w: 4.00,
        d: 4.00,
        h: 2.00,
        base: 0.00,
    },
    Block {
        x: -11.00,
        z: -8.00,
        w: 3.00,
        d: 6.00,
        h: 2.80,
        base: 0.00,
    },
    Block {
        x: 0.00,
        z: -13.00,
        w: 7.00,
        d: 2.00,
        h: 0.90,
        base: 0.00,
    },
    Block {
        x: 11.00,
        z: 7.00,
        w: 4.00,
        d: 4.00,
        h: 1.10,
        base: 0.00,
    },
    Block {
        x: -22.00,
        z: 110.00,
        w: 7.00,
        d: 8.00,
        h: 0.80,
        base: 0.35,
    },
    Block {
        x: -20.00,
        z: 99.00,
        w: 7.00,
        d: 8.00,
        h: 2.10,
        base: 1.65,
    },
    Block {
        x: -18.00,
        z: 88.00,
        w: 7.00,
        d: 8.00,
        h: 3.40,
        base: 2.95,
    },
    Block {
        x: -20.00,
        z: 77.00,
        w: 7.00,
        d: 8.00,
        h: 4.70,
        base: 4.25,
    },
    Block {
        x: -22.00,
        z: 66.00,
        w: 7.00,
        d: 8.00,
        h: 6.00,
        base: 5.55,
    },
    Block {
        x: -24.00,
        z: 55.00,
        w: 7.00,
        d: 8.00,
        h: 7.30,
        base: 6.85,
    },
    Block {
        x: -26.00,
        z: 44.00,
        w: 7.00,
        d: 8.00,
        h: 8.60,
        base: 8.15,
    },
    Block {
        x: -24.00,
        z: 33.00,
        w: 7.00,
        d: 8.00,
        h: 9.90,
        base: 9.45,
    },
    Block {
        x: -22.00,
        z: 22.00,
        w: 7.00,
        d: 8.00,
        h: 11.20,
        base: 10.75,
    },
    Block {
        x: -20.00,
        z: 11.00,
        w: 7.00,
        d: 8.00,
        h: 12.50,
        base: 12.05,
    },
    Block {
        x: 3.00,
        z: 110.00,
        w: 6.00,
        d: 7.00,
        h: 0.80,
        base: 0.35,
    },
    Block {
        x: 5.00,
        z: 97.00,
        w: 6.00,
        d: 7.00,
        h: 2.70,
        base: 2.25,
    },
    Block {
        x: 7.00,
        z: 84.00,
        w: 6.00,
        d: 7.00,
        h: 4.60,
        base: 4.15,
    },
    Block {
        x: 5.00,
        z: 71.00,
        w: 6.00,
        d: 7.00,
        h: 6.50,
        base: 6.05,
    },
    Block {
        x: 3.00,
        z: 58.00,
        w: 6.00,
        d: 7.00,
        h: 8.40,
        base: 7.95,
    },
    Block {
        x: 1.00,
        z: 45.00,
        w: 6.00,
        d: 7.00,
        h: 10.30,
        base: 9.85,
    },
    Block {
        x: -1.00,
        z: 32.00,
        w: 6.00,
        d: 7.00,
        h: 12.20,
        base: 11.75,
    },
    Block {
        x: 1.00,
        z: 19.00,
        w: 6.00,
        d: 7.00,
        h: 14.10,
        base: 13.65,
    },
    Block {
        x: 3.00,
        z: 6.00,
        w: 6.00,
        d: 7.00,
        h: 16.00,
        base: 15.55,
    },
    Block {
        x: 5.00,
        z: -7.00,
        w: 6.00,
        d: 7.00,
        h: 17.90,
        base: 17.45,
    },
    Block {
        x: 30.00,
        z: 110.00,
        w: 5.50,
        d: 6.50,
        h: 0.80,
        base: 0.35,
    },
    Block {
        x: 32.00,
        z: 95.00,
        w: 5.50,
        d: 6.50,
        h: 3.30,
        base: 2.85,
    },
    Block {
        x: 34.00,
        z: 80.00,
        w: 5.50,
        d: 6.50,
        h: 5.80,
        base: 5.35,
    },
    Block {
        x: 32.00,
        z: 65.00,
        w: 5.50,
        d: 6.50,
        h: 8.30,
        base: 7.85,
    },
    Block {
        x: 30.00,
        z: 50.00,
        w: 5.50,
        d: 6.50,
        h: 10.80,
        base: 10.35,
    },
    Block {
        x: 28.00,
        z: 35.00,
        w: 5.50,
        d: 6.50,
        h: 13.30,
        base: 12.85,
    },
    Block {
        x: 26.00,
        z: 20.00,
        w: 5.50,
        d: 6.50,
        h: 15.80,
        base: 15.35,
    },
    Block {
        x: 28.00,
        z: 5.00,
        w: 5.50,
        d: 6.50,
        h: 18.30,
        base: 17.85,
    },
    Block {
        x: 30.00,
        z: -10.00,
        w: 5.50,
        d: 6.50,
        h: 20.80,
        base: 20.35,
    },
    Block {
        x: 32.00,
        z: -25.00,
        w: 5.50,
        d: 6.50,
        h: 23.30,
        base: 22.85,
    },
    Block {
        x: 63.00,
        z: 125.00,
        w: 8.00,
        d: 10.00,
        h: 6.00,
        base: 5.55,
    },
    Block {
        x: 66.00,
        z: 114.00,
        w: 1.00,
        d: 24.00,
        h: 14.00,
        base: 0.00,
    },
    Block {
        x: 63.00,
        z: 99.00,
        w: 8.00,
        d: 8.00,
        h: 5.20,
        base: 4.75,
    },
    Block {
        x: 63.00,
        z: 35.00,
        w: 6.00,
        d: 10.00,
        h: 6.00,
        base: 5.55,
    },
    Block {
        x: 66.00,
        z: 18.00,
        w: 1.00,
        d: 36.00,
        h: 14.00,
        base: 0.00,
    },
    Block {
        x: 63.00,
        z: -3.00,
        w: 6.00,
        d: 8.00,
        h: 5.20,
        base: 4.75,
    },
    Block {
        x: 63.00,
        z: -55.00,
        w: 4.00,
        d: 10.00,
        h: 6.00,
        base: 5.55,
    },
    Block {
        x: 66.00,
        z: -78.00,
        w: 1.00,
        d: 48.00,
        h: 14.00,
        base: 0.00,
    },
    Block {
        x: 63.00,
        z: -105.00,
        w: 4.00,
        d: 8.00,
        h: 5.20,
        base: 4.75,
    },
    Block {
        x: 0.00,
        z: -55.00,
        w: 8.00,
        d: 8.00,
        h: 5.00,
        base: 4.55,
    },
    Block {
        x: 0.00,
        z: -90.00,
        w: 6.00,
        d: 7.00,
        h: 8.00,
        base: 7.55,
    },
    Block {
        x: 0.00,
        z: -123.00,
        w: 5.00,
        d: 6.00,
        h: 11.00,
        base: 10.55,
    },
    Block {
        x: 0.00,
        z: -148.00,
        w: 7.00,
        d: 8.00,
        h: 14.00,
        base: 13.55,
    },
    Block {
        x: 4.00,
        z: -73.00,
        w: 1.00,
        d: 38.00,
        h: 18.00,
        base: 0.00,
    },
    Block {
        x: 4.00,
        z: -106.00,
        w: 1.00,
        d: 34.00,
        h: 21.00,
        base: 0.00,
    },
    Block {
        x: 4.00,
        z: -136.00,
        w: 1.00,
        d: 26.00,
        h: 24.00,
        base: 0.00,
    },
];
const GROUPS: [i32; 51] = [
    -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6,
];
const WALLS: [bool; 51] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, true, false, false, true,
    false, false, true, false, false, false, false, false, true, true, true,
];
const STARTS: [usize; 7] = [5, 15, 25, 35, 38, 41, 44];
#[derive(Clone, Copy)]
struct Movement {
    speed: f32,
    ground_accel: f32,
    air_accel: f32,
    gravity: f32,
    jump: f32,
    double_jump: f32,
    dash: f32,
    max_air_speed: f32,
}
const MOVEMENT: [Movement; 3] = [
    Movement {
        speed: 15.0,
        ground_accel: 18.0,
        air_accel: 32.0,
        gravity: 25.0,
        jump: 10.5,
        double_jump: 9.8,
        dash: 30.0,
        max_air_speed: 32.0,
    },
    Movement {
        speed: 18.0,
        ground_accel: 7.0,
        air_accel: 16.0,
        gravity: 30.0,
        jump: 11.7,
        double_jump: 11.0,
        dash: 34.0,
        max_air_speed: 38.0,
    },
    Movement {
        speed: 12.0,
        ground_accel: 12.0,
        air_accel: 65.0,
        gravity: 15.0,
        jump: 10.8,
        double_jump: 10.0,
        dash: 26.0,
        max_air_speed: 30.0,
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
    last_shot: u32,
}
#[derive(Clone, Copy, Debug, Default)]
struct Pellet {
    p: V,
    dir: V,
    travel: f32,
    shot: u32,
    alive: bool,
}
struct Game {
    p: V,
    v: V,
    grounded: bool,
    movement_preset: usize,
    course_progress: usize,
    playground: usize,
    wall_running: bool,
    wall_time: f32,
    wall_lock: f32,
    wall_normal: f32,
    used_wall: bool,
    used_jump: bool,
    coyote: f32,
    air_jump_available: bool,
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
    pellets: [Pellet; PELLETS],
    last_impact_weapon: u32,
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
            movement_preset: 0,
            course_progress: 0,
            playground: 0,
            wall_running: false,
            wall_time: 0.0,
            wall_lock: 0.0,
            wall_normal: 0.0,
            used_wall: false,
            used_jump: false,
            coyote: 0.14,
            air_jump_available: true,
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
            pellets: [Pellet::default(); PELLETS],
            last_impact_weapon: 0,
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
        let dt = dt.clamp(0.0, 1.0 / 60.0) * if slow { 0.08 } else { 1.0 };
        if !dt.is_finite() || !yaw.is_finite() {
            return;
        }
        self.wall_lock = (self.wall_lock - dt).max(0.0);
        if self.grounded {
            self.wall_time = 0.0;
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
        if self.grounded {
            self.air_jump_available = true;
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
        let pdt = dt;
        let tuning = MOVEMENT[self.movement_preset];
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
                let blend = 1.0 - (-tuning.ground_accel * pdt).exp();
                self.v.x += (wish.x * tuning.speed - self.v.x) * blend;
                self.v.z += (wish.z * tuning.speed - self.v.z) * blend;
            } else if wish.length() > 0.01 {
                // Accelerate along the requested direction without deleting jump momentum.
                let direction = wish.unit();
                let remaining = (tuning.speed * wish.length() - self.v.dot(direction)).max(0.0);
                let acceleration = remaining.min(tuning.air_accel * pdt);
                self.v = self.v.add(direction.scale(acceleration));
                let horizontal = (self.v.x * self.v.x + self.v.z * self.v.z).sqrt();
                if horizontal > tuning.max_air_speed {
                    self.v.x *= tuning.max_air_speed / horizontal;
                    self.v.z *= tuning.max_air_speed / horizontal;
                }
            }
        }
        self.wall_running = false;
        if !self.grounded && self.wall_lock == 0.0 && self.wall_time < 3.4 && wish.z.abs() > 0.25 {
            for (index, wall) in BLOCKS.into_iter().enumerate() {
                if !WALLS[index] {
                    continue;
                }
                let normal = if self.p.x >= wall.x { 1.0 } else { -1.0 };
                let face = wall.x + normal * (wall.w / 2.0 + 0.32);
                if (self.p.x - face).abs() < 0.75
                    && (self.p.z - wall.z).abs() < wall.d / 2.0
                    && self.p.y > wall.base + 0.3
                    && self.p.y + 1.7 < wall.h
                    && (self.v.x * self.v.x + self.v.z * self.v.z).sqrt() > 5.0
                {
                    self.wall_running = true;
                    self.used_wall = true;
                    self.wall_normal = normal;
                    self.wall_time += dt;
                    self.p.x = face;
                    self.v.x = 0.0;
                    self.v.z = wish.z.signum() * tuning.speed.max(18.0);
                    self.v.y = self.v.y.clamp(-0.2, 0.0);
                    break;
                }
            }
        }
        if jump {
            self.used_jump = true;
        }
        if jump && self.wall_running {
            self.v.x = self.wall_normal * 8.0;
            self.v.y = tuning.jump;
            self.wall_running = false;
            self.wall_lock = 0.25;
            self.air_jump_available = true;
            self.jump_buffer = 0.0;
            self.dash_time = 0.0;
        } else if self.jump_buffer > 0.0 && self.coyote > 0.0 {
            self.v.y = tuning.jump;
            self.grounded = false;
            self.coyote = 0.0;
            self.jump_buffer = 0.0;
        } else if jump && self.air_jump_available {
            // Restart a slightly softer arc even while falling. Do not stack upward
            // velocity: timing changes the height, not the strength of the kick.
            // A deliberate air jump redirects momentum instantly, including in
            // bullet time. With no movement input it keeps the existing heading.
            if wish.length() > 0.1 {
                let direction = wish.unit();
                let speed = (self.v.x * self.v.x + self.v.z * self.v.z)
                    .sqrt()
                    .max(tuning.speed * wish.length())
                    .min(tuning.max_air_speed);
                self.v.x = direction.x * speed;
                self.v.z = direction.z * speed;
                self.dash_time = 0.0;
            }
            self.v.y = tuning.double_jump;
            self.air_jump_available = false;
            self.jump_buffer = 0.0;
        }
        if dash && self.dash_cd <= 0.0 {
            let dir = if wish.length() > 0.1 {
                wish.unit()
            } else {
                V::new(-yaw.sin(), 0.0, -yaw.cos())
            };
            self.v.x = dir.x * tuning.dash;
            self.v.z = dir.z * tuning.dash;
            self.dash_time = 0.16;
            self.dash_cd = 0.55;
        }
        let old_y = self.p.y;
        self.v.y -= if self.wall_running {
            0.0
        } else {
            tuning.gravity
        } * pdt;
        self.p = self.p.add(self.v.scale(pdt));
        self.grounded = false;
        if self.p.y <= 0.0 {
            self.p.y = 0.0;
            self.v.y = 0.0;
            self.grounded = true;
        }
        for (index, b) in BLOCKS.into_iter().enumerate() {
            if (self.p.x - b.x).abs() < b.w / 2.0 + 0.32
                && (self.p.z - b.z).abs() < b.d / 2.0 + 0.32
            {
                if old_y >= b.h - 0.03 && self.p.y <= b.h && self.v.y <= 0.0 {
                    self.p.y = b.h;
                    self.v.y = 0.0;
                    self.grounded = true;
                    if GROUPS[index] == self.playground as i32 && !WALLS[index] {
                        let progress = BLOCKS
                            .iter()
                            .enumerate()
                            .take(index + 1)
                            .filter(|(i, _)| GROUPS[*i] == self.playground as i32 && !WALLS[*i])
                            .count();
                        let progress = if self.playground == 6
                            && progress == 4
                            && !(self.used_wall && self.used_jump)
                        {
                            3
                        } else {
                            progress
                        };
                        self.course_progress = self.course_progress.max(progress);
                    }
                } else if old_y + 1.7 <= b.base && self.p.y + 1.7 >= b.base && self.v.y > 0.0 {
                    self.p.y = b.base - 1.7;
                    self.v.y = 0.0;
                } else if self.p.y < b.h && self.p.y + 1.7 > b.base {
                    push_box(&mut self.p, &mut self.v, b, 0.32, false);
                }
            }
        }
        for axis in [0, 2] {
            if axis == 0 && self.p.x.abs() > HALF_WIDTH - 0.5 {
                self.p.x = self.p.x.clamp(-HALF_WIDTH + 0.5, HALF_WIDTH - 0.5);
                self.v.x = 0.0;
            }
            if axis == 2 && self.p.z.abs() > HALF_LENGTH - 0.5 {
                self.p.z = self.p.z.clamp(-HALF_LENGTH + 0.5, HALF_LENGTH - 0.5);
                self.v.z = 0.0;
            }
        }
        if self.grounded {
            self.air_jump_available = true;
        }
        let wdt = dt;
        let previous_bodies = self.bodies.map(|b| b.p);
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
                    && body.p.y + body.r > b.base
                    && (body.p.x - b.x).abs() < b.w / 2.0 + body.r
                    && (body.p.z - b.z).abs() < b.d / 2.0 + body.r
                {
                    if old.y - body.r >= b.h - 0.02 && body.v.y < 0.0 {
                        body.p.y = b.h + body.r;
                        body.v.y *= -0.55;
                    } else if old.y + body.r <= b.base && body.v.y > 0.0 {
                        body.p.y = b.base - body.r;
                        body.v.y *= -0.55;
                    } else {
                        push_box(&mut body.p, &mut body.v, b, body.r, true);
                    }
                }
            }
            for axis in [0, 2] {
                let (p, v) = if axis == 0 {
                    (&mut body.p.x, &mut body.v.x)
                } else {
                    (&mut body.p.z, &mut body.v.z)
                };
                let edge = (if axis == 0 { HALF_WIDTH } else { HALF_LENGTH }) - 0.3 - body.r;
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
        self.step_pellets(dt, previous_bodies);
    }
    fn step_pellets(&mut self, dt: f32, previous: [V; COUNT]) {
        for n in 0..PELLETS {
            let pellet = self.pellets[n];
            if !pellet.alive {
                continue;
            }
            let distance = (PELLET_SPEED * dt).min(9.0 - pellet.travel);
            let motion = pellet.dir.scale(distance);
            let mut fraction = 1.0_f32;
            let mut stopped = false;
            let mut hit = None;
            for b in BLOCKS {
                let t = ray_box(pellet.p, pellet.dir, b);
                if t <= distance {
                    fraction = fraction.min(t / distance.max(0.000001));
                    stopped = true;
                }
            }
            if motion.y < 0.0 && pellet.p.y + motion.y <= 0.0 {
                fraction = fraction.min(-pellet.p.y / motion.y);
                stopped = true;
            }
            for (i, b) in self.bodies.iter().enumerate() {
                // Sweep relative to the target's movement to prevent tunnelling.
                let relative = motion.sub(b.p.sub(previous[i]));
                let oc = pellet.p.sub(previous[i]);
                let a = relative.dot(relative);
                let d = oc.dot(relative);
                let c = oc.dot(oc) - b.r.powi(2);
                let disc = d * d - a * c;
                let t = if c <= 0.0 {
                    0.0
                } else if a > 0.000001 && disc >= 0.0 {
                    (-d - disc.sqrt()) / a
                } else {
                    2.0
                };
                if t >= 0.0 && t < fraction {
                    fraction = t;
                    hit = Some(i);
                    stopped = true;
                }
            }
            self.pellets[n].p = pellet.p.add(motion.scale(fraction));
            self.pellets[n].travel += distance * fraction;
            self.pellets[n].alive = !stopped && self.pellets[n].travel < 9.0;
            if let Some(i) = hit {
                let strength = 5.0 * (0.2 + 0.8 * (1.0 - self.pellets[n].travel / 9.0));
                let b = &mut self.bodies[i];
                b.v = b.v.add(pellet.dir.scale(strength));
                b.glow = 1.0;
                let new_hit = b.last_shot != pellet.shot;
                if new_hit {
                    b.v.y += 1.2;
                    b.last_shot = pellet.shot;
                }
                self.last_hit = i as i32;
                self.last_impact_weapon = 1;
                if new_hit {
                    self.reward(1, 16);
                }
            }
        }
    }
    fn attack(&mut self, yaw: f32, pitch: f32, weapon: u32) -> i32 {
        if self.attack_cd > 0.0 || !yaw.is_finite() || !pitch.is_finite() {
            return -2;
        }
        if weapon == 1 && self.pellets.iter().filter(|p| !p.alive).count() < 9 {
            return -2;
        }
        self.attack_cd = if weapon == 0 { 0.32 } else { 0.58 };
        self.shots += 1;
        self.last_hit = -1;
        self.last_distance = 35.0;
        let eye = self.p.add(V::new(0.0, 1.65, 0.0));
        let dir = V::new(
            -yaw.sin() * pitch.cos(),
            pitch.sin(),
            -yaw.cos() * pitch.cos(),
        );
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
                    self.last_impact_weapon = 0;
                    self.last_distance = dist;
                }
            }
            if count > 0 {
                self.reward(count, 12);
            }
            return count as i32;
        }
        // Fire travelling pellets. Hits are resolved against moving targets in step().
        let right = V::new(yaw.cos(), 0.0, -yaw.sin());
        let up = V::new(
            yaw.sin() * pitch.sin(),
            pitch.cos(),
            yaw.cos() * pitch.sin(),
        );
        let mut index = 0;
        for pellet in self.pellets.iter_mut().filter(|p| !p.alive).take(9) {
            let ray = if index == 0 {
                dir
            } else {
                let angle = (index - 1) as f32 * std::f32::consts::TAU / 8.0;
                dir.add(right.scale(angle.cos() * 0.12))
                    .add(up.scale(angle.sin() * 0.12))
                    .unit()
            };
            *pellet = Pellet {
                p: eye,
                dir: ray,
                travel: 0.0,
                shot: self.shots,
                alive: true,
            };
            index += 1;
        }
        0
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
fn push_box(p: &mut V, v: &mut V, b: Block, r: f32, bounce: bool) {
    let dx = b.w / 2.0 + r - (p.x - b.x).abs();
    let dz = b.d / 2.0 + r - (p.z - b.z).abs();
    if dx < dz {
        let sign = if p.x >= b.x { 1.0 } else { -1.0 };
        p.x = b.x + sign * (b.w / 2.0 + r);
        v.x *= if bounce { -0.2 } else { 0.0 };
    } else {
        let sign = if p.z >= b.z { 1.0 } else { -1.0 };
        p.z = b.z + sign * (b.d / 2.0 + r);
        v.z *= if bounce { -0.2 } else { 0.0 };
    }
}
fn ray_box(origin: V, dir: V, b: Block) -> f32 {
    let mut near = 0.0_f32;
    let mut far = 100.0_f32;
    for (o, d, min, max) in [
        (origin.x, dir.x, b.x - b.w / 2.0, b.x + b.w / 2.0),
        (origin.y, dir.y, b.base, b.h),
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
    GAME.with(|g| {
        let preset = g.borrow().movement_preset;
        let mut fresh = Game::new(seed);
        fresh.movement_preset = preset;
        *g.borrow_mut() = fresh;
    });
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
pub extern "C" fn projectile_count() -> usize {
    PELLETS
}
#[no_mangle]
pub extern "C" fn projectile(index: usize, field: usize) -> f32 {
    GAME.with(|g| {
        g.borrow()
            .pellets
            .get(index)
            .map(|p| match field {
                0 => p.alive as u32 as f32,
                1 => p.p.x,
                2 => p.p.y,
                3 => p.p.z,
                4 => p.dir.x,
                5 => p.dir.y,
                6 => p.dir.z,
                7 => p.travel,
                _ => 0.0,
            })
            .unwrap_or(0.0)
    })
}
#[no_mangle]
pub extern "C" fn last_impact_weapon() -> u32 {
    GAME.with(|g| g.borrow().last_impact_weapon)
}
#[no_mangle]
pub extern "C" fn air_jump_available() -> u32 {
    GAME.with(|g| g.borrow().air_jump_available as u32)
}
#[no_mangle]
pub extern "C" fn set_movement_preset(index: usize) {
    GAME.with(|g| g.borrow_mut().movement_preset = index.min(2));
}
#[no_mangle]
pub extern "C" fn movement_preset() -> usize {
    GAME.with(|g| g.borrow().movement_preset)
}
#[no_mangle]
pub extern "C" fn course_progress() -> usize {
    GAME.with(|g| g.borrow().course_progress)
}
#[no_mangle]
pub extern "C" fn arena_extent(axis: u32) -> f32 {
    if axis == 0 {
        HALF_WIDTH * 2.0
    } else {
        HALF_LENGTH * 2.0
    }
}
#[no_mangle]
pub extern "C" fn start_runway() {
    GAME.with(|g| {
        let mut g = g.borrow_mut();
        g.p = V::new(-50.0, 0.0, 140.0);
        g.v = V::default();
        g.grounded = true;
        g.air_jump_available = true;
        g.coyote = 0.14;
        g.jump_buffer = 0.0;
        g.attack_cd = 0.0;
        g.dash_cd = 0.0;
        g.dash_time = 0.0;
        g.course_progress = 0;
        g.wall_running = false;
        g.wall_time = 0.0;
        g.wall_lock = 0.0;
        g.used_wall = false;
        g.used_jump = false;
        g.pellets.fill(Pellet::default());
    });
}
#[no_mangle]
pub extern "C" fn wall_running() -> u32 {
    GAME.with(|g| g.borrow().wall_running as u32)
}
#[no_mangle]
pub extern "C" fn start_course() {
    let index = GAME.with(|g| g.borrow().playground);
    start_playground(index);
}
#[no_mangle]
pub extern "C" fn start_playground(index: usize) {
    GAME.with(|g| {
        let mut g = g.borrow_mut();
        g.playground = index.min(6);
        let b = BLOCKS[STARTS[g.playground]];
        g.p = V::new(b.x, b.h, b.z);
        if (3..6).contains(&g.playground) {
            g.p.x = 65.18;
        }
        g.v = V::default();
        g.grounded = true;
        g.air_jump_available = true;
        g.coyote = 0.14;
        g.jump_buffer = 0.0;
        g.attack_cd = 0.0;
        g.dash_cd = 0.0;
        g.dash_time = 0.0;
        g.course_progress = 1;
        g.wall_running = false;
        g.wall_time = 0.0;
        g.wall_lock = 0.0;
        g.used_wall = false;
        g.used_jump = false;
        g.pellets.fill(Pellet::default());
    });
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
            5 => b.base,
            6 => GROUPS[index] as f32,
            7 => WALLS[index] as u32 as f32,
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
    fn double_jump_relaunches_falling_player_and_preserves_momentum() {
        let mut g = Game::new(1);
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, true, true, false);
        run(&mut g, 0.5, 0.0, 0.0);
        assert!(g.v.y < 0.0 && g.p.y > 0.0);
        let height = g.p.y;
        let forward_speed = g.v.z;
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
        assert!(g.v.y > 9.5 && g.p.y > height && !g.air_jump_available);
        assert_eq!(g.v.z, forward_speed);
        run(&mut g, 0.12, 0.0, 0.0);
        let before = g.v.y;
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
        assert!(g.v.y < before); // No third midair jump.
        run(&mut g, 1.2, 0.0, 0.0);
        assert!(g.grounded && g.air_jump_available);
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
        assert!(g.v.y > 10.0);
    }
    #[test]
    fn air_jump_redirects_fast_momentum_in_bullet_time_without_extra_jumps() {
        for preset in 0..3 {
            let mut g = Game::new(1);
            g.movement_preset = preset;
            g.p = V::new(-50.0, 6.0, 70.0);
            g.grounded = false;
            g.coyote = 0.0;
            g.v = V::new(0.0, -3.0, -26.0);
            g.dash_time = 0.1;
            let height = g.p.y;
            g.step(1.0 / 120.0, 0.0, 1.0, 0.0, true, false, true);
            assert!((g.v.x - 26.0).abs() < 0.01 && g.v.z.abs() < 0.01);
            assert!(g.v.y > 9.0 && (g.p.y - height).abs() < 0.01);
            assert!(!g.air_jump_available && g.dash_time == 0.0);
            g.step(1.0 / 120.0, 0.0, -1.0, 0.0, true, false, true);
            assert!(g.v.x > 25.0); // No repeat redirect without landing.
        }
    }
    #[test]
    fn early_double_jump_does_not_stack_upward_speed_or_repeat_itself() {
        let mut g = Game::new(1);
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
        assert!(g.v.y < 9.8 && !g.air_jump_available);
        run(&mut g, 0.3, 0.0, 0.0);
        assert!(g.v.y < 3.0 && !g.air_jump_available);
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
    fn walking_into_obstacle_stops_without_bouncing() {
        let mut g = Game::new(1);
        g.p = V::new(0.0, 0.0, -11.5);
        run(&mut g, 0.2, 1.0, 0.0);
        let contact_z = g.p.z;
        for _ in 0..120 {
            g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, false);
            assert!((g.p.z - contact_z).abs() < 0.00001);
            assert_eq!(g.v.z, 0.0);
        }
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
    fn shotgun_hits_aimed_target_and_misses_away() {
        let mut g = Game::new(1);
        let delta = g.bodies[0].p.sub(g.p.add(V::new(0.0, 1.65, 0.0)));
        let pitch = (delta.y / delta.length()).asin();
        assert_eq!(g.attack(0.0, pitch, 1), 0);
        assert_eq!(g.hits, 0);
        run(&mut g, 0.12, 0.0, 0.0);
        assert_eq!(g.hits, 1);
        run(&mut g, 0.6, 0.0, 0.0);
        assert_eq!(g.attack(std::f32::consts::PI, 0.0, 1), 0);
    }
    #[test]
    fn shotgun_is_heavy_up_close_and_cannot_hit_beyond_range() {
        let mut close = Game::new(1);
        let delta = close.bodies[0].p.sub(close.p.add(V::new(0.0, 1.65, 0.0)));
        let pitch = (delta.y / delta.length()).asin();
        assert_eq!(close.attack(0.0, pitch, 1), 0);
        run(&mut close, 0.08, 0.0, 0.0);
        assert_eq!(close.hits, 1);
        assert!(close.bodies[0].v.z < -25.0);
        assert_eq!(close.attack(0.0, pitch, 1), -2);
        let mut far = Game::new(1);
        for b in &mut far.bodies {
            b.p = V::new(15.0, 1.65, 15.0);
        }
        far.bodies[0].p = V::new(0.0, 1.65, -0.0); // Ten metres from the player.
        assert_eq!(far.attack(0.0, 0.0, 1), 0);
        run(&mut far, 0.3, 0.0, 0.0);
        assert_eq!(far.hits, 0);
        assert_eq!(far.bodies[0].v.x, 0.0);
        assert_eq!(far.bodies[0].v.z, 0.0);
    }
    #[test]
    fn shotgun_spread_hits_multiple_targets_and_falls_off() {
        let mut g = Game::new(1);
        for b in &mut g.bodies {
            b.p = V::new(15.0, 1.65, 15.0);
        }
        g.bodies[0].p = V::new(-0.7, 1.65, 4.0);
        g.bodies[1].p = V::new(0.7, 1.65, 4.0);
        assert_eq!(g.attack(0.0, 0.0, 1), 0);
        run(&mut g, 0.15, 0.0, 0.0);
        assert_eq!(g.hits, 2);
        assert!(g.bodies[0].v.z < 0.0 && g.bodies[1].v.z < 0.0);
        assert!(g.bodies[0].v.z.abs() < 20.0);
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
        run(&mut g, 0.3, 0.0, 0.0);
        assert_eq!(g.hits, 0);
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
    fn bullet_time_delays_hits_and_resume_releases_pellets() {
        let mut g = Game::new(1);
        let delta = g.bodies[0].p.sub(g.p.add(V::new(0.0, 1.65, 0.0)));
        let pitch = (delta.y / delta.length()).asin();
        g.attack(0.0, pitch, 1);
        for _ in 0..36 {
            g.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, true);
        }
        assert_eq!(g.hits, 0);
        assert!(g.pellets.iter().any(|p| p.alive));
        assert_eq!(g.bodies[0].v.z, 0.0);
        run(&mut g, 0.06, 0.0, 0.0);
        assert_eq!(g.hits, 1);
        assert!(g.bodies[0].v.z < -20.0);
    }
    #[test]
    fn bullet_time_slows_air_motion_and_cooldowns_then_restores_them() {
        let mut normal = Game::new(1);
        let mut slow = Game::new(1);
        normal.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
        slow.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
        normal.attack(0.0, 0.0, 1);
        slow.attack(0.0, 0.0, 1);
        for _ in 0..120 {
            normal.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, false);
            slow.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, true);
        }
        assert!(normal.grounded && !slow.grounded && slow.v.y > 8.0);
        assert!(slow.attack_cd > 0.49 && normal.attack_cd == 0.0);
        let before = slow.v.y;
        slow.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, false);
        assert!((slow.v.y - (before - 25.0 / 120.0)).abs() < 0.00001);
    }
    #[test]
    fn travelling_pellets_can_miss_a_target_that_moves_away() {
        let mut g = Game::new(1);
        g.attack(0.0, -0.35, 1);
        g.bodies[0].p.x = 8.0;
        run(&mut g, 0.25, 0.0, 0.0);
        assert_eq!(g.hits, 0);
    }
    #[test]
    fn expanded_arena_allows_long_runs_and_stops_at_new_bounds() {
        let mut g = Game::new(1);
        g.p = V::new(-50.0, 0.0, 140.0);
        run(&mut g, 12.0, 1.0, 0.0);
        assert!(g.p.z < -38.0 && g.grounded);
        run(&mut g, 20.0, 1.0, 0.0);
        assert!((g.p.z - (-HALF_LENGTH + 0.5)).abs() < 0.001);
        assert_eq!(g.v.z, 0.0);
        g.p = V::new(HALF_WIDTH - 0.51, 0.0, 0.0);
        run(&mut g, 1.0, 0.0, 1.0);
        assert!((g.p.x - (HALF_WIDTH - 0.5)).abs() < 0.001);
    }
    #[test]
    fn wall_only_playgrounds_finish_without_jumping_in_all_presets() {
        for preset in 0..3 {
            for group in 3..6 {
                let b = BLOCKS[STARTS[group]];
                let mut g = Game::new(1);
                g.playground = group;
                g.movement_preset = preset;
                g.p = V::new(65.18, b.h, b.z);
                let mut touched = false;
                for _ in 0..720 {
                    g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, false);
                    touched |= g.wall_running;
                    if g.course_progress == 2 {
                        break;
                    }
                }
                assert!(
                    touched && g.course_progress == 2,
                    "wall tier {group}, preset {preset}: {:?}",
                    (g.p.x, g.p.y, g.p.z)
                );
            }
        }
    }
    #[test]
    fn wall_jump_pushes_away_and_wall_run_respects_bullet_time() {
        let mut g = Game::new(1);
        g.p = V::new(65.18, 6.0, 115.0);
        g.grounded = false;
        g.coyote = 0.0;
        g.v = V::new(0.0, -2.0, -18.0);
        let z = g.p.z;
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, true);
        assert!(g.wall_running && (g.p.z - z).abs() < 0.02);
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, true, false, true);
        assert!(!g.wall_running && g.v.x < -7.0 && g.v.y > 10.0 && g.air_jump_available);
        g.p.y = 0.0;
        g.grounded = true;
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, false);
        assert!(!g.wall_running);
    }
    #[test]
    fn tall_jump_tiers_are_reachable_with_snappy_without_wall_running() {
        for group in 0..3 {
            for preset in 0..1 {
                for index in STARTS[group]..STARTS[group] + 9 {
                    let from = BLOCKS[index];
                    let to = BLOCKS[index + 1];
                    let yaw = -(to.x - from.x).atan2(from.z - to.z);
                    let mut reached = false;
                    for second in [usize::MAX, 18, 30, 42, 54, 66, 78] {
                        let mut g = Game::new(1);
                        g.playground = group;
                        g.movement_preset = preset;
                        g.p = V::new(from.x, from.h, from.z);
                        g.v = V::new(-yaw.sin(), 0.0, -yaw.cos()).scale(MOVEMENT[preset].speed);
                        for frame in 0..480 {
                            g.step(
                                1.0 / 120.0,
                                1.0,
                                0.0,
                                yaw,
                                frame == 0 || frame == second,
                                false,
                                false,
                            );
                            assert!(!g.wall_running);
                            if frame > 1 && g.grounded {
                                reached |= (g.p.y - to.h).abs() < 0.001;
                                break;
                            }
                        }
                        if reached {
                            break;
                        }
                    }
                    assert!(reached, "jump tier {group}, preset {preset}, gap {index}");
                }
            }
        }
    }
    #[test]
    fn combined_gaps_can_chain_wall_runs_and_jumps_with_snappy() {
        for segment in 0..3 {
            let from = BLOCKS[44 + segment];
            let to = BLOCKS[45 + segment];
            let wall = BLOCKS[48 + segment];
            let face = wall.x - wall.w / 2.0 - 0.32;
            let mut reached = false;
            for kick_distance in [6.0, 9.0, 12.0, 15.0, 18.0] {
                for double_delay in [24, 36, 48, 60] {
                    let mut g = Game::new(1);
                    g.playground = 6;
                    g.p = V::new(from.x, from.h, from.z);
                    let mut kicked = false;
                    let mut kick_frame = 0;
                    for frame in 0..900 {
                        let yaw = if !kicked {
                            -(face - g.p.x).atan2(10.0)
                        } else {
                            -(to.x - g.p.x).atan2((g.p.z - to.z).max(1.0))
                        };
                        let kick = g.wall_running && !kicked && g.p.z - to.z < kick_distance;
                        let jump =
                            frame == 24 || kick || (kicked && frame == kick_frame + double_delay);
                        if kick {
                            kicked = true;
                            kick_frame = frame;
                        }
                        g.step(1.0 / 120.0, 1.0, 0.0, yaw, jump, false, false);
                        if kicked && g.grounded {
                            reached = (g.p.y - to.h).abs() < 0.001 && g.used_wall && g.used_jump;
                            break;
                        }
                    }
                    if reached {
                        break;
                    }
                }
                if reached {
                    break;
                }
            }
            assert!(reached, "combined gap {segment}");
        }
    }
    #[test]
    fn combined_finish_requires_both_skills() {
        for (wall, jump, expected) in [
            (false, false, 3),
            (true, false, 3),
            (false, true, 3),
            (true, true, 4),
        ] {
            let mut g = Game::new(1);
            g.playground = 6;
            g.used_wall = wall;
            g.used_jump = jump;
            let finish = BLOCKS[47];
            g.p = V::new(finish.x, finish.h + 0.1, finish.z);
            g.v.y = -5.0;
            g.grounded = false;
            run(&mut g, 0.1, 0.0, 0.0);
            assert_eq!(g.course_progress, expected);
        }
    }
    #[test]
    fn wall_runs_work_on_both_faces_and_have_a_time_limit() {
        let mut g = Game::new(1);
        g.p = V::new(66.82, 6.0, 115.0);
        g.grounded = false;
        g.coyote = 0.0;
        g.v.z = -18.0;
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, false);
        assert!(g.wall_running);
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, true, false, false);
        assert!(g.v.x > 7.0);
        g.wall_lock = 0.0;
        g.wall_time = 3.4;
        g.v.z = -18.0;
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, false);
        assert!(!g.wall_running);
    }
    #[test]
    fn movement_presets_produce_distinct_speed_and_jump_arcs() {
        let mut speeds = [0.0; 3];
        let mut airtimes = [0.0; 3];
        for preset in 0..3 {
            let mut g = Game::new(1);
            g.movement_preset = preset;
            g.p = V::new(-25.0, 0.0, 20.0);
            run(&mut g, 0.5, 1.0, 0.0);
            speeds[preset] = -g.v.z;
            g.v = V::default();
            g.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
            while !g.grounded && airtimes[preset] < 3.0 {
                run(&mut g, 1.0 / 120.0, 0.0, 0.0);
                airtimes[preset] += 1.0 / 120.0;
            }
        }
        assert!(speeds[1] > speeds[0] && speeds[0] > speeds[2]);
        assert!(airtimes[2] > airtimes[0] * 1.6 && airtimes[2] > airtimes[1] * 1.6);
    }
    #[test]
    fn floating_platforms_support_landings_and_allow_walking_underneath() {
        for (index, b) in BLOCKS
            .into_iter()
            .enumerate()
            .skip(5)
            .filter(|(i, _)| !WALLS[*i])
        {
            let group = GROUPS[index] as usize;
            let mut g = Game::new(1);
            g.playground = group;
            g.used_wall = true;
            g.used_jump = true;
            g.p = V::new(b.x, b.h + 1.0, b.z);
            g.v.y = -10.0;
            run(&mut g, 0.2, 0.0, 0.0);
            assert!(g.grounded && (g.p.y - b.h).abs() < 0.001);
            assert_eq!(
                g.course_progress,
                BLOCKS
                    .iter()
                    .enumerate()
                    .take(index + 1)
                    .filter(|(i, _)| GROUPS[*i] == group as i32 && !WALLS[*i])
                    .count()
            );
        }
        let mut g = Game::new(1);
        let b = BLOCKS[8];
        g.p = V::new(b.x, 0.0, b.z);
        run(&mut g, 0.1, 0.0, 0.0);
        assert_eq!(g.p.x, b.x);
        assert_eq!(g.p.z, b.z);
        g.p.y = b.base - 1.75;
        g.v.y = 10.0;
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, false);
        assert!((g.p.y + 1.7 - b.base).abs() < 0.001 && g.v.y == 0.0);
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
        assert!(g.p.x.abs() <= HALF_WIDTH - 0.5 && g.p.z.abs() <= HALF_LENGTH - 0.5);
        for b in g.bodies {
            assert!(b.p.x.is_finite() && b.p.y.is_finite() && b.p.z.is_finite());
        }
    }
}
