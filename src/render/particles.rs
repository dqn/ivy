use macroquad::prelude::*;

/// Particle effect type.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ParticleType {
    #[default]
    None,
    Snow,
    Rain,
    Sakura,
    Sparkle,
    Leaves,
}

impl ParticleType {
    /// Parse from string.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "snow" => ParticleType::Snow,
            "rain" => ParticleType::Rain,
            "sakura" | "cherry" | "petals" => ParticleType::Sakura,
            "sparkle" | "sparkles" | "stars" => ParticleType::Sparkle,
            "leaves" | "leaf" => ParticleType::Leaves,
            "" | "none" => ParticleType::None,
            _ => ParticleType::None,
        }
    }
}

/// A single particle.
#[derive(Debug, Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    size: f32,
    rotation: f32,
    rotation_speed: f32,
    alpha: f32,
    color: Color,
    life: f32,
    max_life: f32,
}

impl Particle {
    fn new(particle_type: ParticleType, screen_w: f32, screen_h: f32) -> Self {
        let mut rng = ::macroquad::rand::rand();
        let mut rand_f = || {
            let val = rng as f32 / u32::MAX as f32;
            rng = ::macroquad::rand::rand();
            val
        };

        let r1 = rand_f();
        let r2 = rand_f();
        let r3 = rand_f();
        let r4 = rand_f();
        let r5 = rand_f();

        match particle_type {
            ParticleType::Snow => Self {
                x: r1 * screen_w,
                y: -20.0 - r2 * 50.0,
                vx: (r3 - 0.5) * 20.0,
                vy: 30.0 + r4 * 40.0,
                size: 3.0 + r5 * 5.0,
                rotation: 0.0,
                rotation_speed: 0.0,
                alpha: 0.7 + r1 * 0.3,
                color: WHITE,
                life: 0.0,
                max_life: 15.0 + r2 * 5.0,
            },
            ParticleType::Rain => Self {
                x: r1 * screen_w,
                y: -20.0 - r2 * 100.0,
                vx: -20.0,
                vy: 300.0 + r3 * 200.0,
                size: 2.0,
                rotation: 0.0,
                rotation_speed: 0.0,
                alpha: 0.3 + r4 * 0.4,
                color: Color::new(0.7, 0.8, 1.0, 1.0),
                life: 0.0,
                max_life: 5.0,
            },
            ParticleType::Sakura => Self {
                x: r1 * screen_w,
                y: -20.0 - r2 * 50.0,
                vx: 20.0 + r3 * 30.0,
                vy: 40.0 + r4 * 30.0,
                size: 8.0 + r5 * 6.0,
                rotation: r1 * std::f32::consts::TAU,
                rotation_speed: (r2 - 0.5) * 2.0,
                alpha: 0.8 + r3 * 0.2,
                color: Color::new(1.0, 0.85, 0.9, 1.0),
                life: 0.0,
                max_life: 12.0 + r4 * 4.0,
            },
            ParticleType::Sparkle => Self {
                x: r1 * screen_w,
                y: r2 * screen_h,
                vx: (r3 - 0.5) * 10.0,
                vy: (r4 - 0.5) * 10.0,
                size: 2.0 + r5 * 4.0,
                rotation: 0.0,
                rotation_speed: 0.0,
                alpha: 0.0,
                color: Color::new(1.0, 1.0, 0.8, 1.0),
                life: 0.0,
                max_life: 2.0 + r1 * 2.0,
            },
            ParticleType::Leaves => Self {
                x: r1 * screen_w,
                y: -20.0 - r2 * 50.0,
                vx: 30.0 + r3 * 40.0,
                vy: 50.0 + r4 * 30.0,
                size: 10.0 + r5 * 8.0,
                rotation: r1 * std::f32::consts::TAU,
                rotation_speed: (r2 - 0.5) * 3.0,
                alpha: 0.7 + r3 * 0.3,
                color: Color::new(0.6, 0.5, 0.2, 1.0),
                life: 0.0,
                max_life: 10.0 + r4 * 5.0,
            },
            ParticleType::None => Self {
                x: 0.0,
                y: 0.0,
                vx: 0.0,
                vy: 0.0,
                size: 0.0,
                rotation: 0.0,
                rotation_speed: 0.0,
                alpha: 0.0,
                color: WHITE,
                life: 0.0,
                max_life: 1.0,
            },
        }
    }

    fn update(&mut self, dt: f32, particle_type: ParticleType) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.rotation += self.rotation_speed * dt;
        self.life += dt;

        // Sparkle particles fade in and out
        if particle_type == ParticleType::Sparkle {
            let progress = self.life / self.max_life;
            self.alpha = if progress < 0.3 {
                progress / 0.3
            } else if progress > 0.7 {
                (1.0 - progress) / 0.3
            } else {
                1.0
            };
        }
    }

    fn is_alive(&self, screen_h: f32) -> bool {
        self.life < self.max_life && self.y < screen_h + 50.0
    }

    fn draw(&self, particle_type: ParticleType) {
        let color = Color::new(self.color.r, self.color.g, self.color.b, self.alpha);

        match particle_type {
            ParticleType::Snow => {
                draw_circle(self.x, self.y, self.size, color);
            }
            ParticleType::Rain => {
                draw_line(
                    self.x,
                    self.y,
                    self.x + self.vx * 0.02,
                    self.y + self.vy * 0.02,
                    self.size,
                    color,
                );
            }
            ParticleType::Sakura | ParticleType::Leaves => {
                // Draw a simple petal/leaf shape using triangles
                let s = self.size;
                let cos_r = self.rotation.cos();
                let sin_r = self.rotation.sin();

                // Rotate points around center
                let rotate = |dx: f32, dy: f32| -> (f32, f32) {
                    (
                        self.x + dx * cos_r - dy * sin_r,
                        self.y + dx * sin_r + dy * cos_r,
                    )
                };

                let p1 = rotate(0.0, -s);
                let p2 = rotate(s * 0.5, s * 0.3);
                let p3 = rotate(-s * 0.5, s * 0.3);

                draw_triangle(
                    Vec2::new(p1.0, p1.1),
                    Vec2::new(p2.0, p2.1),
                    Vec2::new(p3.0, p3.1),
                    color,
                );
            }
            ParticleType::Sparkle => {
                // Draw a star shape
                let s = self.size;
                for i in 0..4 {
                    let angle = (i as f32) * std::f32::consts::PI / 4.0 + get_time() as f32;
                    let x1 = self.x + angle.cos() * s;
                    let y1 = self.y + angle.sin() * s;
                    let x2 = self.x - angle.cos() * s;
                    let y2 = self.y - angle.sin() * s;
                    draw_line(x1, y1, x2, y2, 1.0, color);
                }
            }
            ParticleType::None => {}
        }
    }
}

/// Particle system state.
pub struct ParticleState {
    particles: Vec<Particle>,
    particle_type: ParticleType,
    intensity: f32,
    spawn_timer: f32,
}

impl Default for ParticleState {
    fn default() -> Self {
        Self {
            particles: Vec::new(),
            particle_type: ParticleType::None,
            intensity: 0.5,
            spawn_timer: 0.0,
        }
    }
}

impl ParticleState {
    /// Set the particle type and intensity.
    pub fn set(&mut self, particle_type: ParticleType, intensity: f32) {
        if self.particle_type != particle_type {
            self.particles.clear();
        }
        self.particle_type = particle_type;
        self.intensity = intensity.clamp(0.0, 1.0);
    }

    /// Stop all particles.
    pub fn stop(&mut self) {
        self.particle_type = ParticleType::None;
        // Let existing particles fade out naturally
    }

    /// Update and draw particles.
    pub fn update_and_draw(&mut self) {
        let dt = get_frame_time();
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Update existing particles
        for particle in &mut self.particles {
            particle.update(dt, self.particle_type);
        }

        // Remove dead particles
        self.particles.retain(|p| p.is_alive(screen_h));

        // Spawn new particles based on intensity and type
        if self.particle_type != ParticleType::None {
            self.spawn_timer += dt;

            let spawn_rate = match self.particle_type {
                ParticleType::Snow => 0.1 / self.intensity,
                ParticleType::Rain => 0.02 / self.intensity,
                ParticleType::Sakura => 0.2 / self.intensity,
                ParticleType::Sparkle => 0.15 / self.intensity,
                ParticleType::Leaves => 0.25 / self.intensity,
                ParticleType::None => 1000.0,
            };

            while self.spawn_timer >= spawn_rate {
                self.spawn_timer -= spawn_rate;
                self.particles.push(Particle::new(self.particle_type, screen_w, screen_h));
            }
        }

        // Draw all particles
        for particle in &self.particles {
            particle.draw(self.particle_type);
        }
    }

    /// Check if particles are active.
    pub fn is_active(&self) -> bool {
        self.particle_type != ParticleType::None || !self.particles.is_empty()
    }
}
