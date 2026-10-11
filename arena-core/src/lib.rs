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
        z: 130.00,
        w: 8.50,
        d: 10.00,
        h: 0.80,
        base: 0.35,
    },
    Block {
        x: -20.00,
        z: 112.00,
        w: 8.50,
        d: 10.00,
        h: 2.10,
        base: 1.65,
    },
    Block {
        x: -18.00,
        z: 94.00,
        w: 8.50,
        d: 10.00,
        h: 3.40,
        base: 2.95,
    },
    Block {
        x: -20.00,
        z: 76.00,
        w: 8.50,
        d: 10.00,
        h: 4.70,
        base: 4.25,
    },
    Block {
        x: -22.00,
        z: 58.00,
        w: 8.50,
        d: 10.00,
        h: 6.00,
        base: 5.55,
    },
    Block {
        x: -24.00,
        z: 40.00,
        w: 8.50,
        d: 10.00,
        h: 7.30,
        base: 6.85,
    },
    Block {
        x: -26.00,
        z: 22.00,
        w: 8.50,
        d: 10.00,
        h: 8.60,
        base: 8.15,
    },
    Block {
        x: -24.00,
        z: 4.00,
        w: 8.50,
        d: 10.00,
        h: 9.90,
        base: 9.45,
    },
    Block {
        x: -22.00,
        z: -14.00,
        w: 8.50,
        d: 10.00,
        h: 11.20,
        base: 10.75,
    },
    Block {
        x: -20.00,
        z: -32.00,
        w: 8.50,
        d: 10.00,
        h: 12.50,
        base: 12.05,
    },
    Block {
        x: 3.00,
        z: 130.00,
        w: 7.50,
        d: 9.00,
        h: 0.80,
        base: 0.35,
    },
    Block {
        x: 5.00,
        z: 109.00,
        w: 7.50,
        d: 9.00,
        h: 2.70,
        base: 2.25,
    },
    Block {
        x: 7.00,
        z: 88.00,
        w: 7.50,
        d: 9.00,
        h: 4.60,
        base: 4.15,
    },
    Block {
        x: 5.00,
        z: 67.00,
        w: 7.50,
        d: 9.00,
        h: 6.50,
        base: 6.05,
    },
    Block {
        x: 3.00,
        z: 46.00,
        w: 7.50,
        d: 9.00,
        h: 8.40,
        base: 7.95,
    },
    Block {
        x: 1.00,
        z: 25.00,
        w: 7.50,
        d: 9.00,
        h: 10.30,
        base: 9.85,
    },
    Block {
        x: -1.00,
        z: 4.00,
        w: 7.50,
        d: 9.00,
        h: 12.20,
        base: 11.75,
    },
    Block {
        x: 1.00,
        z: -17.00,
        w: 7.50,
        d: 9.00,
        h: 14.10,
        base: 13.65,
    },
    Block {
        x: 3.00,
        z: -38.00,
        w: 7.50,
        d: 9.00,
        h: 16.00,
        base: 15.55,
    },
    Block {
        x: 5.00,
        z: -59.00,
        w: 7.50,
        d: 9.00,
        h: 17.90,
        base: 17.45,
    },
    Block {
        x: 30.00,
        z: 130.00,
        w: 6.50,
        d: 8.00,
        h: 0.80,
        base: 0.35,
    },
    Block {
        x: 32.00,
        z: 106.00,
        w: 6.50,
        d: 8.00,
        h: 3.30,
        base: 2.85,
    },
    Block {
        x: 34.00,
        z: 82.00,
        w: 6.50,
        d: 8.00,
        h: 5.80,
        base: 5.35,
    },
    Block {
        x: 32.00,
        z: 58.00,
        w: 6.50,
        d: 8.00,
        h: 8.30,
        base: 7.85,
    },
    Block {
        x: 30.00,
        z: 34.00,
        w: 6.50,
        d: 8.00,
        h: 10.80,
        base: 10.35,
    },
    Block {
        x: 28.00,
        z: 10.00,
        w: 6.50,
        d: 8.00,
        h: 13.30,
        base: 12.85,
    },
    Block {
        x: 26.00,
        z: -14.00,
        w: 6.50,
        d: 8.00,
        h: 15.80,
        base: 15.35,
    },
    Block {
        x: 28.00,
        z: -38.00,
        w: 6.50,
        d: 8.00,
        h: 18.30,
        base: 17.85,
    },
    Block {
        x: 30.00,
        z: -62.00,
        w: 6.50,
        d: 8.00,
        h: 20.80,
        base: 20.35,
    },
    Block {
        x: 32.00,
        z: -86.00,
        w: 6.50,
        d: 8.00,
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
        z: 107.50,
        w: 1.00,
        d: 37.00,
        h: 14.00,
        base: 0.00,
    },
    Block {
        x: 63.00,
        z: 85.00,
        w: 8.00,
        d: 12.00,
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
        z: 9.50,
        w: 1.00,
        d: 53.00,
        h: 14.00,
        base: 0.00,
    },
    Block {
        x: 63.00,
        z: -21.00,
        w: 6.00,
        d: 12.00,
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
        z: -88.50,
        w: 1.00,
        d: 69.00,
        h: 14.00,
        base: 0.00,
    },
    Block {
        x: 63.00,
        z: -127.00,
        w: 4.00,
        d: 12.00,
        h: 5.20,
        base: 4.75,
    },
    Block {
        x: -70.00,
        z: 70.00,
        w: 9.00,
        d: 10.00,
        h: 5.00,
        base: 4.55,
    },
    Block {
        x: -70.00,
        z: 20.00,
        w: 7.00,
        d: 9.00,
        h: 8.00,
        base: 7.55,
    },
    Block {
        x: -70.00,
        z: -30.00,
        w: 6.00,
        d: 8.00,
        h: 11.00,
        base: 10.55,
    },
    Block {
        x: -70.00,
        z: -80.00,
        w: 8.00,
        d: 10.00,
        h: 14.00,
        base: 13.55,
    },
    Block {
        x: -66.00,
        z: 45.00,
        w: 1.00,
        d: 52.00,
        h: 18.00,
        base: 0.00,
    },
    Block {
        x: -66.00,
        z: -5.00,
        w: 1.00,
        d: 52.00,
        h: 21.00,
        base: 0.00,
    },
    Block {
        x: -66.00,
        z: -55.00,
        w: 1.00,
        d: 52.00,
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
        speed: 24.0,
        ground_accel: 18.0,
        air_accel: 32.0,
        gravity: 25.0,
        jump: 10.5,
        double_jump: 9.8,
        dash: 30.0,
        max_air_speed: 30.0,
    },
    Movement {
        speed: 28.0,
        ground_accel: 7.0,
        air_accel: 16.0,
        gravity: 30.0,
        jump: 11.7,
        double_jump: 11.0,
        dash: 34.0,
        max_air_speed: 38.0,
    },
    Movement {
        speed: 20.0,
        ground_accel: 12.0,
        air_accel: 65.0,
        gravity: 15.0,
        jump: 10.8,
        double_jump: 10.0,
        dash: 30.0,
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
    health: f32,
    respawn: f32,
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
    landing_grip: f32,
    movement_preset: usize,
    physics_mode: usize,
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
    bat_origin: Option<V>,
    bodies: [Body; COUNT],
    score: u32,
    hits: u32,
    shots: u32,
    combo: u32,
    best: u32,
    combo_time: f32,
    time: f32,
    breaks: u32,
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
            V::new(-2.5, 1.65, 5.0),
            V::new(0.0, 1.65, 2.0),
            V::new(2.5, 1.65, 5.0),
            V::new(4.0, 1.65, 0.0),
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
                health: if i < 6 { 50.0 } else { 20.0 },
                ..Body::default()
            };
        }
        Self {
            p: V::new(0.0, 0.0, 10.0),
            v: V::default(),
            grounded: true,
            landing_grip: 0.0,
            movement_preset: 0,
            physics_mode: 0,
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
            bat_origin: None,
            bodies,
            score: 0,
            hits: 0,
            shots: 0,
            combo: 0,
            best: 0,
            combo_time: 0.0,
            time: 0.0,
            breaks: 0,
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
        self.landing_grip = (self.landing_grip - dt).max(0.0);
        let was_grounded = self.grounded;
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
                // Grip builds over the first 160ms after touchdown. Keep a
                // short landing skid, then return to normal firm traction.
                let grip = 1.0 - 0.55 * (self.landing_grip / 0.16);
                let blend = 1.0 - (-tuning.ground_accel * grip * pdt).exp();
                let horizontal = (self.v.x * self.v.x + self.v.z * self.v.z).sqrt();
                let aligned = wish.length() > 0.1
                    && horizontal > 0.1
                    && self.v.dot(wish.unit()) / horizontal > 0.75;
                // Carry earned speed while holding a direction; releasing or
                // reversing input still brakes with the preset's normal response.
                let speed = if aligned {
                    horizontal.min(tuning.dash).max(tuning.speed)
                } else {
                    tuning.speed
                };
                self.v.x += (wish.x * speed - self.v.x) * blend;
                self.v.z += (wish.z * speed - self.v.z) * blend;
                if wish.length() < 0.01 && (self.v.x * self.v.x + self.v.z * self.v.z).sqrt() < 0.08
                {
                    self.v.x = 0.0;
                    self.v.z = 0.0;
                }
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
        if self.wall_running && wish.x * self.wall_normal > 0.15 {
            self.v.x = wish.x * 6.0;
            self.wall_lock = 0.12;
        }
        self.wall_running = false;
        if !self.grounded && self.wall_lock == 0.0 && self.wall_time < 3.4 && wish.z.abs() > 0.25 {
            for (index, wall) in BLOCKS.into_iter().enumerate() {
                if !WALLS[index] {
                    continue;
                }
                let normal = if self.p.x >= wall.x { 1.0 } else { -1.0 };
                let face = wall.x + normal * (wall.w / 2.0 + 0.32);
                if (self.p.x - face).abs() < 0.20
                    && wish.x * normal <= 0.15
                    && self.v.x * normal < 1.5
                    && (self.p.z - wall.z).abs() < wall.d / 2.0
                    && self.p.y > wall.base + 0.3
                    && self.p.y + 1.7 < wall.h
                    && (self.v.x * self.v.x + self.v.z * self.v.z).sqrt() > 5.0
                {
                    self.wall_running = true;
                    self.used_wall = true;
                    self.wall_normal = normal;
                    self.wall_time += dt;
                    // Contact supports the run without pulling the player onto the wall.
                    if self.v.x * normal < 0.0 {
                        self.v.x = 0.0;
                    }
                    self.v.z = wish.z.signum()
                        * self
                            .v
                            .z
                            .abs()
                            .max(tuning.speed.max(24.0))
                            .min(tuning.max_air_speed);
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
            // A well-timed jump out of a fast landing releases a little extra
            // compression. It depends on momentum and timing, never a dice roll.
            let pop = if self.physics_mode > 0 && self.landing_grip > 0.0 {
                ((self.v.x.hypot(self.v.z) - 20.0).max(0.0) * 0.22)
                    .min(2.4 * self.physics_mode as f32)
            } else {
                0.0
            };
            self.v.y = tuning.jump + pop;
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
            if !was_grounded {
                self.landing_grip = 0.16;
            }
        }
        let wdt = dt;
        let mut previous_bodies = self.bodies.map(|b| b.p);
        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.glow = (body.glow - dt * 2.0).max(0.0);
            body.scored = (body.scored - dt).max(0.0);
            let rebuilding = body.health <= 0.0;
            if body.health <= 0.0 {
                body.respawn = (body.respawn - dt).max(0.0);
                if body.respawn > 0.0 {
                    continue;
                }
                body.p = body.home;
                body.v = V::default();
                body.health = body_max_health(body.kind, self.physics_mode);
                body.last_shot = 0;
                previous_bodies[i] = body.p;
            }
            let old = body.p;
            if body.kind == 1 {
                body.p = body.home;
                body.v = V::default();
                if i == 9 {
                    body.p.x += (self.time * 0.8).sin() * 1.5;
                    body.v.x = (self.time * 0.8).cos() * 1.2;
                }
                if rebuilding {
                    previous_bodies[i] = body.p;
                }
                continue;
            }
            body.v.y -= 18.0 * wdt;
            body.v = body
                .v
                .scale((-(if self.physics_mode == 0 { 0.32 } else { 0.12 }) * wdt).exp());
            body.p = body.p.add(body.v.scale(wdt));
            if body.p.y < body.r {
                body.p.y = body.r;
                if body.v.y < 0.0 {
                    body.v.y *= -body_bounce(self.physics_mode);
                }
                if body.v.y.abs() < 0.8 {
                    body.v.y = 0.0;
                }
                let friction = if self.physics_mode == 0 { 6.0 } else { 2.2 };
                body.v.x *= (-friction * wdt).exp();
                body.v.z *= (-friction * wdt).exp();
            }
            for b in BLOCKS {
                if body.p.y - body.r < b.h
                    && body.p.y + body.r > b.base
                    && (body.p.x - b.x).abs() < b.w / 2.0 + body.r
                    && (body.p.z - b.z).abs() < b.d / 2.0 + body.r
                {
                    if old.y - body.r >= b.h - 0.02 && body.v.y < 0.0 {
                        body.p.y = b.h + body.r;
                        body.v.y *= -body_bounce(self.physics_mode);
                    } else if old.y + body.r <= b.base && body.v.y > 0.0 {
                        body.p.y = b.base - body.r;
                        body.v.y *= -body_bounce(self.physics_mode);
                    } else {
                        let incoming = body.v;
                        push_box(&mut body.p, &mut body.v, b, body.r, true);
                        if self.physics_mode > 0 {
                            let hit_x = (incoming.x - body.v.x).abs() > 0.001;
                            let normal = if hit_x {
                                incoming.x.abs()
                            } else {
                                incoming.z.abs()
                            };
                            let tangent = if hit_x {
                                incoming.z.abs()
                            } else {
                                incoming.x.abs()
                            };
                            if hit_x {
                                body.v.x = -incoming.x * body_bounce(self.physics_mode);
                            } else {
                                body.v.z = -incoming.z * body_bounce(self.physics_mode);
                            }
                            // A glancing edge contact redirects some horizontal
                            // energy upward. Head-on bumps remain ordinary rebounds.
                            if normal > 6.0 && tangent > 4.0 {
                                let lift = (normal * 0.35 + tangent * 0.22)
                                    .min(16.0 * self.physics_mode as f32);
                                body.v.y = body.v.y.max(lift);
                            }
                        }
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
                    *v *= -if self.physics_mode == 0 { 0.7 } else { 0.9 };
                }
            }
            if body.p.y > 0.0
                && body.p.y < 2.0
                && (body.p.x - self.p.x).hypot(body.p.z - self.p.z) < body.r + 0.35
            {
                let delta = V::new(body.p.x - self.p.x, 0.0, body.p.z - self.p.z).unit();
                body.v = body.v.add(delta.scale(8.0 * wdt));
                if self.physics_mode > 0 {
                    let closing = self.v.sub(body.v).dot(delta).max(0.0);
                    body.v = body.v.add(delta.scale(closing * 0.6));
                    if closing > 10.0 {
                        body.v.y = body.v.y.max(closing * 0.25 * self.physics_mode as f32);
                    }
                }
            }
        }
        // Axis-aligned block contacts match the simple visible crate geometry.
        for a in 0..COUNT {
            for b in a + 1..COUNT {
                if self.bodies[a].kind != 0
                    || self.bodies[b].kind != 0
                    || self.bodies[a].health <= 0.0
                    || self.bodies[b].health <= 0.0
                {
                    continue;
                }
                let delta = self.bodies[b].p.sub(self.bodies[a].p);
                let radius = self.bodies[a].r + self.bodies[b].r;
                let overlap = V::new(
                    radius - delta.x.abs(),
                    radius - delta.y.abs(),
                    radius - delta.z.abs(),
                );
                if overlap.x > 0.0 && overlap.y > 0.0 && overlap.z > 0.0 {
                    let (n, depth) = if overlap.x < overlap.y && overlap.x < overlap.z {
                        (
                            V::new(if delta.x >= 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0),
                            overlap.x,
                        )
                    } else if overlap.y < overlap.z {
                        (
                            V::new(0.0, if delta.y >= 0.0 { 1.0 } else { -1.0 }, 0.0),
                            overlap.y,
                        )
                    } else {
                        (
                            V::new(0.0, 0.0, if delta.z >= 0.0 { 1.0 } else { -1.0 }),
                            overlap.z,
                        )
                    };
                    let push = n.scale(depth * 0.5);
                    self.bodies[a].p = self.bodies[a].p.sub(push);
                    self.bodies[b].p = self.bodies[b].p.add(push);
                    let closing = self.bodies[b].v.sub(self.bodies[a].v).dot(n);
                    if closing < 0.0 {
                        let restitution = if self.physics_mode == 0 { 0.65 } else { 0.95 };
                        let impulse = n.scale(-closing * restitution);
                        self.bodies[a].v = self.bodies[a].v.sub(impulse);
                        self.bodies[b].v = self.bodies[b].v.add(impulse);
                        if self.physics_mode > 0 && n.y == 0.0 && closing < -8.0 {
                            // Uneven side contacts can pop the higher block up;
                            // an exactly level impact exchanges sideways momentum.
                            let height_offset = delta.y.abs() / radius;
                            if height_offset > 0.12 {
                                let lift = (-closing * height_offset * 0.8)
                                    .min(22.0 * self.physics_mode as f32);
                                let higher = if delta.y >= 0.0 { b } else { a };
                                self.bodies[higher].v.y = self.bodies[higher].v.y.max(lift);
                            }
                        }
                    }
                }
            }
        }
        if self.physics_mode > 0 {
            // Bound stacked impacts while keeping strong, reproducible launches.
            for b in &mut self.bodies {
                let limit = 70.0 + 20.0 * self.physics_mode as f32;
                if b.v.length() > limit {
                    b.v = b.v.unit().scale(limit);
                }
                b.v.y = b.v.y.clamp(-50.0, 35.0 + self.physics_mode as f32 * 8.0);
                b.p.y = b.p.y.max(b.r);
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
                if b.health <= 0.0 {
                    continue;
                }
                let relative = motion.sub(b.p.sub(previous[i]));
                let length = relative.length();
                let bounds = body_bounds(*b, previous[i]);
                let t = if length > 0.000001 {
                    ray_box(pellet.p, relative.scale(1.0 / length), bounds) / length
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
                if self.physics_mode > 0 && b.kind == 0 {
                    b.v =
                        b.v.add(pellet.dir.scale(strength * 0.65 * self.physics_mode as f32));
                }
                b.glow = 1.0;
                b.health =
                    (b.health - 8.0 * (0.2 + 0.8 * (1.0 - self.pellets[n].travel / 9.0))).max(0.0);
                if b.health == 0.0 {
                    b.respawn = 3.0;
                    self.breaks += 1;
                }
                let new_hit = b.last_shot != pellet.shot;
                if new_hit {
                    b.v.y += 1.2;
                    if self.physics_mode > 0 && b.kind == 0 {
                        b.v.y += 2.5 * self.physics_mode as f32;
                    }
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
            let start = self.bat_origin.take().unwrap_or(eye);
            let path = eye.sub(start);
            let length_squared = path.dot(path);
            let mut count = 0;
            for (i, b) in self.bodies.iter_mut().enumerate() {
                if b.health <= 0.0 {
                    continue;
                }
                // Sweep the melee reach through the player's movement since
                // wind-up. A fast run must not skip a block between frames.
                let fraction = if length_squared > 0.0001 {
                    b.p.sub(dir.scale(1.8)).sub(start).dot(path) / length_squared
                } else {
                    1.0
                };
                let origin = start.add(path.scale(fraction.clamp(0.0, 1.0)));
                let delta = b.p.sub(origin);
                let dist = delta.length();
                if dist < 3.7 + b.r
                    && delta.unit().dot(dir) > 0.62
                    && BLOCKS
                        .iter()
                        .all(|block| ray_box(origin, delta.unit(), *block) > dist - b.r)
                {
                    // A driving sideways strike: forward power, modest lift, carried momentum.
                    let carry = b.v.scale(0.2).add(self.v.scale(0.45));
                    b.v = V::new(-yaw.sin(), 0.0, -yaw.cos()).scale(32.0).add(carry);
                    b.v.y = 3.2 + pitch.max(0.0) * 16.0 + self.v.y.max(0.0) * 0.2;
                    if self.physics_mode > 0 && b.kind == 0 {
                        let factor = self.physics_mode as f32;
                        let speed = self.v.x.hypot(self.v.z);
                        let sideways = delta.dot(V::new(yaw.cos(), 0.0, -yaw.sin())) / b.r;
                        b.v = b.v.add(
                            V::new(yaw.cos(), 0.0, -yaw.sin())
                                .scale(sideways.clamp(-1.0, 1.0) * 7.0 * factor),
                        );
                        b.v.y += (2.5 + speed * 0.32 + carry.y.max(0.0) * 0.7) * factor;
                    }
                    b.glow = 1.0;
                    b.health = (b.health
                        - 50.0
                        - (self.v.x * self.v.x + self.v.z * self.v.z).sqrt() * 0.9)
                        .max(0.0);
                    if b.health == 0.0 {
                        b.respawn = 3.0;
                        self.breaks += 1;
                    }
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
            self.breaks as f32,
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
fn body_max_health(kind: u32, physics_mode: usize) -> f32 {
    if kind == 1 {
        20.0
    } else if physics_mode == 0 {
        50.0
    } else {
        220.0
    }
}
fn body_bounce(physics_mode: usize) -> f32 {
    [0.25, 0.62, 0.82][physics_mode]
}
fn body_bounds(body: Body, p: V) -> Block {
    Block {
        x: p.x,
        z: p.z,
        w: body.r * 2.0,
        d: if body.kind == 0 { body.r * 2.0 } else { 0.18 },
        h: p.y + body.r,
        base: p.y - body.r,
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
        let physics = g.borrow().physics_mode;
        let mut fresh = Game::new(seed);
        fresh.movement_preset = preset;
        fresh.physics_mode = physics;
        for body in &mut fresh.bodies {
            body.health = body_max_health(body.kind, physics);
        }
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
pub extern "C" fn begin_bat_swing() {
    GAME.with(|g| {
        let mut g = g.borrow_mut();
        g.bat_origin = Some(g.p.add(V::new(0.0, 1.65, 0.0)));
    });
}
#[no_mangle]
pub extern "C" fn cancel_bat_swing() {
    GAME.with(|g| g.borrow_mut().bat_origin = None);
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
pub extern "C" fn set_physics_mode(index: usize) {
    GAME.with(|g| {
        let mut g = g.borrow_mut();
        g.physics_mode = index.min(2);
        let mode = g.physics_mode;
        for b in &mut g.bodies {
            b.p = b.home;
            b.v = V::default();
            b.health = body_max_health(b.kind, mode);
            b.respawn = 0.0;
            b.last_shot = 0;
        }
        g.pellets.fill(Pellet::default());
    });
}
#[no_mangle]
pub extern "C" fn physics_mode() -> usize {
    GAME.with(|g| g.borrow().physics_mode)
}
#[no_mangle]
pub extern "C" fn max_body_health(index: usize) -> f32 {
    GAME.with(|g| {
        let g = g.borrow();
        g.bodies
            .get(index)
            .map(|b| body_max_health(b.kind, g.physics_mode))
            .unwrap_or(0.0)
    })
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
        g.landing_grip = 0.0;
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
pub extern "C" fn body_health(index: usize) -> f32 {
    GAME.with(|g| {
        g.borrow()
            .bodies
            .get(index)
            .map(|b| b.health)
            .unwrap_or(0.0)
    })
}
#[no_mangle]
pub extern "C" fn body_respawn(index: usize) -> f32 {
    GAME.with(|g| {
        g.borrow()
            .bodies
            .get(index)
            .map(|b| b.respawn)
            .unwrap_or(0.0)
    })
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
        g.landing_grip = 0.0;
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
            assert!(
                (g.v.x - 26.0_f32.max(MOVEMENT[preset].speed)).abs() < 0.01 && g.v.z.abs() < 0.01
            );
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
    fn bat_smashes_front_block_with_forward_impulse() {
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
            b.home = b.p;
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
            b.home = b.p;
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
    fn wall_running_does_not_pull_from_a_distance_and_releases_when_steering_away() {
        let mut g = Game::new(1);
        g.p = V::new(64.75, 6.0, 115.0);
        g.grounded = false;
        g.coyote = 0.0;
        g.v.z = -30.0;
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, false);
        assert!(!g.wall_running && g.p.x == 64.75);
        g.p.x = 65.08;
        g.v.x = 0.0;
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, false);
        assert!(g.wall_running && g.p.x == 65.08 && g.v.z < -29.9);
        let x = g.p.x;
        g.step(1.0 / 120.0, 1.0, -1.0, 0.0, false, false, false);
        assert!(!g.wall_running && g.p.x < x && g.v.x < 0.0);
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
        assert!(g.wall_running && (g.p.z - z).abs() < 0.025);
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, true, false, true);
        assert!(!g.wall_running && g.v.x < -7.0 && g.v.y > 10.0 && g.air_jump_available);
        g.p.y = 0.0;
        g.grounded = true;
        g.landing_grip = 0.0;
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, false, false);
        assert!(!g.wall_running);
    }
    #[test]
    fn tall_jump_tiers_are_reachable_with_snappy_without_wall_running() {
        for group in 0..3 {
            for speed in [24.0, 30.0] {
                let preset = 0;
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
                        g.v = V::new(-yaw.sin(), 0.0, -yaw.cos()).scale(speed);
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
            for kick_distance in [3.0, 6.0, 9.0, 12.0, 15.0, 18.0, 21.0, 24.0, 27.0] {
                for double_delay in [12, 24, 36, 48, 60, 72] {
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
    fn landing_has_a_short_skid_then_stops_with_firm_grip() {
        let mut g = Game::new(1);
        g.p = V::new(-50.0, 0.02, 140.0);
        g.grounded = false;
        g.coyote = 0.0;
        g.v = V::new(30.0, -5.0, 0.0);
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, false);
        assert!(g.grounded && g.v.x == 30.0 && g.landing_grip == 0.16);
        let x = g.p.x;
        run(&mut g, 0.08, 0.0, 0.0);
        assert!(g.v.x > 12.0 && g.v.x < 30.0 && g.p.x > x + 1.0);
        run(&mut g, 0.5, 0.0, 0.0);
        assert_eq!(g.v.x, 0.0);
        assert!(g.p.x - x < 4.0);
        let stopped = g.p.x;
        run(&mut g, 1.0, 0.0, 0.0);
        assert_eq!(g.p.x, stopped);
    }
    #[test]
    fn snappy_runs_at_24_and_carries_30_without_repeated_dashes_or_jumps() {
        let mut g = Game::new(1);
        g.p = V::new(-50.0, 0.0, 140.0);
        run(&mut g, 1.0, 1.0, 0.0);
        assert!((-g.v.z - 24.0).abs() < 0.01);
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, false, true, false);
        run(&mut g, 1.0, 1.0, 0.0);
        assert!((-g.v.z - 30.0).abs() < 0.01);
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, true, false, false);
        run(&mut g, 1.2, 1.0, 0.0);
        assert!(g.grounded && (-g.v.z - 30.0).abs() < 0.01);
        run(&mut g, 0.5, 0.0, 0.0);
        assert!(g.v.z.abs() < 0.1);
        run(&mut g, 1.0, 1.0, 0.0);
        assert!((-g.v.z - 24.0).abs() < 0.01);
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
    fn blocks_break_after_damage_and_respawn_cleanly() {
        let mut g = Game::new(1);
        assert_eq!(g.attack(0.0, -0.15, 0), 1);
        assert_eq!(g.bodies[0].health, 0.0);
        assert_eq!(g.breaks, 1);
        g.attack_cd = 0.0;
        assert_eq!(g.attack(0.0, -0.15, 0), 0);
        run(&mut g, 3.1, 0.0, 0.0);
        assert_eq!(g.bodies[0].health, 50.0);
        assert!(g.bodies[0].v.length() < 1.0);
    }
    #[test]
    fn square_target_hits_match_panel_edges_and_break_on_contact() {
        let mut g = Game::new(1);
        g.bodies[6].p = V::new(0.0, 1.65, 5.0);
        g.bodies[6].home = g.bodies[6].p;
        let b = g.bodies[6];
        let bounds = body_bounds(b, b.p);
        assert!(
            ray_box(
                V::new(b.r - 0.01, 1.65, 10.0),
                V::new(0.0, 0.0, -1.0),
                bounds
            ) < 6.0
        );
        assert!(
            ray_box(
                V::new(b.r + 0.01, 1.65, 10.0),
                V::new(0.0, 0.0, -1.0),
                bounds
            ) > 9.0
        );
        g.bodies[0].p.x = 8.0;
        g.attack(0.0, 0.0, 1);
        run(&mut g, 0.2, 0.0, 0.0);
        assert_eq!(g.bodies[6].health, 0.0);
        assert!(g.breaks > 0);
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
    #[test]
    fn playful_running_hits_launch_durable_blocks_without_randomness() {
        let mut standing = Game::new(1);
        standing.physics_mode = 1;
        standing.bodies[0].health = 220.0;
        let mut running = Game::new(1);
        running.physics_mode = 1;
        running.bodies[0].health = 220.0;
        running.v.z = -30.0;
        assert_eq!(standing.attack(0.0, -0.14, 0), 1);
        assert_eq!(running.attack(0.0, -0.14, 0), 1);
        assert!(standing.bodies[0].health > 0.0 && running.bodies[0].health > 0.0);
        assert!(running.bodies[0].v.y > standing.bodies[0].v.y + 9.0);
        assert!(running.bodies[0].v.z < standing.bodies[0].v.z - 12.0);
        run(&mut running, 0.4, 0.0, 0.0);
        assert!(running.bodies[0].p.y > 4.0);
    }
    #[test]
    fn bat_sweep_hits_a_block_passed_at_dash_speed_but_respects_walls() {
        let mut g = Game::new(1);
        g.bat_origin = Some(g.p.add(V::new(0.0, 1.65, 0.0)));
        g.p.z = 6.4;
        g.v.z = -30.0;
        assert_eq!(g.attack(0.0, -0.14, 0), 1);
        assert!(g.bat_origin.is_none());
        let mut blocked = Game::new(1);
        blocked.bat_origin = Some(V::new(0.0, 1.65, -11.0));
        blocked.p = V::new(0.0, 0.0, -11.65);
        // The block is in range, but the solid slab at z=-13 blocks the arc.
        blocked.bodies[0].p = V::new(0.0, 0.6, -14.6);
        assert_eq!(blocked.attack(0.0, -0.14, 0), 0);
    }
    #[test]
    fn off_center_bat_hits_curve_in_the_direction_of_the_contact() {
        for offset in [-0.4, 0.4] {
            let mut g = Game::new(1);
            g.physics_mode = 1;
            g.bodies[0].p.x = offset;
            g.bodies[0].health = 220.0;
            assert_eq!(g.attack(0.0, -0.14, 0), 1);
            assert!(g.bodies[0].v.x * offset > 1.0);
        }
    }
    #[test]
    fn glancing_static_collisions_pop_but_head_on_collisions_do_not() {
        for tangent in [0.0, 15.0] {
            let mut g = Game::new(1);
            g.physics_mode = 1;
            g.bodies[0].health = 220.0;
            g.bodies[0].p = V::new(-6.0, 0.6, 2.65);
            g.bodies[0].v = V::new(tangent, 0.0, -25.0);
            g.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, false);
            assert!(g.bodies[0].v.z > 10.0);
            if tangent > 0.0 {
                assert!(g.bodies[0].v.y > 10.0);
            } else {
                assert!(g.bodies[0].v.y < 1.0);
            }
        }
    }
    #[test]
    fn uneven_block_contacts_launch_the_higher_block() {
        let mut g = Game::new(1);
        g.physics_mode = 1;
        g.bodies[0].p = V::new(15.0, 0.6, 20.0);
        g.bodies[1].p = V::new(16.18, 1.1, 20.0);
        g.bodies[0].v = V::new(30.0, 0.0, 0.0);
        g.step(1.0 / 120.0, 0.0, 0.0, 0.0, false, false, false);
        assert!(g.bodies[1].v.x > 20.0 && g.bodies[1].v.y > 8.0);
        assert!(g.bodies[0].v.y < 1.0);
    }
    #[test]
    fn playful_rebuild_and_mode_switch_preserve_the_selected_physics() {
        set_physics_mode(2);
        reset(99);
        assert_eq!(physics_mode(), 2);
        assert_eq!(max_body_health(0), 220.0);
        assert_eq!(max_body_health(6), 20.0);
        set_physics_mode(0);
        assert_eq!(max_body_health(0), 50.0);
        let mut g = Game::new(1);
        g.physics_mode = 1;
        g.bodies[0].health = 0.0;
        g.bodies[0].respawn = 0.1;
        run(&mut g, 0.15, 0.0, 0.0);
        assert_eq!(g.bodies[0].health, 220.0);
    }
    #[test]
    fn fast_timed_landing_jump_adds_pop_but_idle_jump_does_not() {
        let mut g = Game::new(1);
        g.physics_mode = 1;
        g.p = V::new(-50.0, 0.0, 70.0);
        g.landing_grip = 0.1;
        g.v.z = -30.0;
        g.step(1.0 / 120.0, 1.0, 0.0, 0.0, true, false, false);
        assert!(g.v.y > 12.0);
        let mut idle = Game::new(1);
        idle.physics_mode = 1;
        idle.step(1.0 / 120.0, 0.0, 0.0, 0.0, true, false, false);
        assert!(idle.v.y < 10.5);
    }
    #[test]
    fn wild_stacked_impulses_stay_bounded_and_reproducible() {
        fn simulate() -> [f32; SNAPSHOT] {
            let mut g = Game::new(1);
            g.physics_mode = 2;
            for b in &mut g.bodies {
                b.health = body_max_health(b.kind, 2);
            }
            for i in 0..6000 {
                if i % 180 == 0 {
                    g.bodies[0].v = V::new(800.0, 300.0, 400.0);
                }
                g.step(
                    1.0 / 120.0,
                    1.0,
                    0.5,
                    i as f32 * 0.02,
                    i % 90 == 0,
                    i % 80 == 0,
                    i % 400 < 120,
                );
                if i % 60 == 0 {
                    g.attack(i as f32 * 0.02, 0.1, 0);
                }
                for b in &g.bodies {
                    assert!(b.v.length() < 110.01 && b.p.y >= b.r);
                }
            }
            let mut out = [0.0; SNAPSHOT];
            g.snapshot(&mut out);
            out
        }
        let a = simulate();
        assert_eq!(a, simulate());
        assert!(a.into_iter().all(f32::is_finite));
    }
}
