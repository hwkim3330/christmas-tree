use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// Snowflake particle
#[derive(Clone, Copy)]
struct Snowflake {
    x: f64,
    y: f64,
    radius: f64,
    velocity_x: f64,
    velocity_y: f64,
    opacity: f64,
    wobble_offset: f64,
}

impl Snowflake {
    fn new(width: f64, height: f64, seed: u32) -> Self {
        let rand = pseudo_random(seed);
        Self {
            x: rand * width,
            y: pseudo_random(seed + 1) * height,
            radius: pseudo_random(seed + 2) * 2.5 + 0.5,
            velocity_x: pseudo_random(seed + 3) * 0.4 - 0.2,
            velocity_y: pseudo_random(seed + 4) * 1.2 + 0.4,
            opacity: pseudo_random(seed + 5) * 0.5 + 0.3,
            wobble_offset: pseudo_random(seed + 6) * 1000.0,
        }
    }

    fn update(&mut self, width: f64, height: f64, time: f64) {
        self.y += self.velocity_y;
        self.x += self.velocity_x + ((self.y + self.wobble_offset) / 80.0).sin() * 0.3;

        // Wrap around
        if self.y > height {
            self.y = -5.0;
            self.x = pseudo_random((time * 1000.0) as u32 + (self.x * 100.0) as u32) * width;
        }
        if self.x > width {
            self.x = 0.0;
        }
        if self.x < 0.0 {
            self.x = width;
        }
    }
}

// Sparkle particle
#[derive(Clone, Copy)]
struct Sparkle {
    x: f64,
    y: f64,
    dest_x: f64,
    dest_y: f64,
    life: f64,
    max_life: f64,
    active: bool,
}

impl Sparkle {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            dest_x: 0.0,
            dest_y: 0.0,
            life: 0.0,
            max_life: 1.0,
            active: false,
        }
    }

    fn spawn(&mut self, x: f64, y: f64, seed: u32) {
        let angle = pseudo_random(seed) * std::f64::consts::PI * 2.0;
        let distance = 25.0 + pseudo_random(seed + 1) * 40.0;
        self.x = x;
        self.y = y;
        self.dest_x = x + angle.cos() * distance;
        self.dest_y = y + angle.sin() * distance;
        self.life = 0.0;
        self.max_life = 0.4 + pseudo_random(seed + 2) * 0.4;
        self.active = true;
    }

    fn update(&mut self, dt: f64) {
        if !self.active {
            return;
        }
        self.life += dt;
        if self.life >= self.max_life {
            self.active = false;
        }
    }

    fn get_position(&self) -> (f64, f64, f64) {
        let progress = (self.life / self.max_life).min(1.0);
        let eased = 1.0 - (1.0 - progress).powi(2); // ease-out
        let x = self.x + (self.dest_x - self.x) * eased;
        let y = self.y + (self.dest_y - self.y) * eased;
        let opacity = 1.0 - progress;
        (x, y, opacity)
    }
}

// Light effect
#[derive(Clone, Copy)]
struct Light {
    x: f64,
    y: f64,
    color_r: u8,
    color_g: u8,
    color_b: u8,
    phase: f64,
}

impl Light {
    fn new(x: f64, y: f64, r: u8, g: u8, b: u8, phase: f64) -> Self {
        Self {
            x,
            y,
            color_r: r,
            color_g: g,
            color_b: b,
            phase,
        }
    }

    fn get_brightness(&self, time: f64) -> f64 {
        let t = (time * 4.0 + self.phase).sin();
        0.4 + (t + 1.0) * 0.3 // 0.4 to 1.0
    }
}

// Pseudo-random number generator (simple LCG)
fn pseudo_random(seed: u32) -> f64 {
    let a: u64 = 1103515245;
    let c: u64 = 12345;
    let m: u64 = 2147483648;
    let result = ((a.wrapping_mul(seed as u64).wrapping_add(c)) % m) as f64 / m as f64;
    result
}

#[wasm_bindgen]
pub struct ParticleSystem {
    snowflakes: Vec<Snowflake>,
    sparkles: Vec<Sparkle>,
    lights: Vec<Light>,
    width: f64,
    height: f64,
    time: f64,
    sparkle_index: usize,
}

#[wasm_bindgen]
impl ParticleSystem {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, snow_count: u32) -> ParticleSystem {
        #[cfg(feature = "console_error_panic_hook")]
        set_panic_hook();

        let mut snowflakes = Vec::with_capacity(snow_count as usize);
        for i in 0..snow_count {
            snowflakes.push(Snowflake::new(width, height, i * 7));
        }

        // Pre-allocate sparkles pool
        let sparkles = vec![Sparkle::new(); 100];

        // Initialize lights
        let light_positions = [
            (25.0, 85.0), (40.0, 88.0), (55.0, 86.0), (70.0, 84.0),
            (28.0, 70.0), (42.0, 73.0), (58.0, 71.0), (72.0, 68.0),
            (32.0, 55.0), (45.0, 58.0), (55.0, 56.0), (68.0, 53.0),
            (36.0, 42.0), (50.0, 45.0), (64.0, 40.0),
            (42.0, 28.0), (50.0, 32.0), (58.0, 27.0),
        ];
        let light_colors: [(u8, u8, u8); 6] = [
            (239, 68, 68),   // red
            (251, 191, 36),  // yellow
            (34, 197, 94),   // green
            (59, 130, 246),  // blue
            (244, 114, 182), // pink
            (168, 85, 247),  // purple
        ];

        let mut lights = Vec::new();
        for (i, &(x, y)) in light_positions.iter().enumerate() {
            let (r, g, b) = light_colors[i % light_colors.len()];
            lights.push(Light::new(x, y, r, g, b, pseudo_random(i as u32 * 13) * 6.28));
        }

        ParticleSystem {
            snowflakes,
            sparkles,
            lights,
            width,
            height,
            time: 0.0,
            sparkle_index: 0,
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;

        // Update snowflakes
        for flake in &mut self.snowflakes {
            flake.update(self.width, self.height, self.time);
        }

        // Update sparkles
        for sparkle in &mut self.sparkles {
            sparkle.update(dt);
        }
    }

    pub fn spawn_sparkles(&mut self, x: f64, y: f64, count: u32) {
        let base_seed = (self.time * 10000.0) as u32;
        for i in 0..count {
            let sparkle = &mut self.sparkles[self.sparkle_index];
            sparkle.spawn(x, y, base_seed + i * 3);
            self.sparkle_index = (self.sparkle_index + 1) % self.sparkles.len();
        }
    }

    pub fn render_snow(&self, ctx: &CanvasRenderingContext2d) {
        for flake in &self.snowflakes {
            ctx.begin_path();
            let color = format!("rgba(255, 255, 255, {})", flake.opacity);
            ctx.set_fill_style_str(&color);
            ctx.arc(flake.x, flake.y, flake.radius, 0.0, std::f64::consts::PI * 2.0).ok();
            ctx.fill();
        }
    }

    pub fn render_sparkles(&self, ctx: &CanvasRenderingContext2d) {
        for sparkle in &self.sparkles {
            if !sparkle.active {
                continue;
            }
            let (x, y, opacity) = sparkle.get_position();
            let size = 8.0 * (1.0 - sparkle.life / sparkle.max_life);

            // Create radial gradient effect
            ctx.begin_path();
            let color = format!("rgba(255, 255, 255, {})", opacity);
            ctx.set_fill_style_str(&color);
            ctx.arc(x, y, size / 2.0, 0.0, std::f64::consts::PI * 2.0).ok();
            ctx.fill();
        }
    }

    pub fn render_lights(&self, ctx: &CanvasRenderingContext2d, tree_width: f64, tree_height: f64, offset_x: f64, offset_y: f64) {
        for light in &self.lights {
            let brightness = light.get_brightness(self.time);
            let x = offset_x + (light.x / 100.0) * tree_width;
            let y = offset_y + (light.y / 100.0) * tree_height;
            let size = 4.0 + brightness * 4.0;

            // Glow effect
            let glow_color = format!(
                "rgba({}, {}, {}, {})",
                light.color_r, light.color_g, light.color_b,
                brightness * 0.6
            );
            ctx.begin_path();
            ctx.set_fill_style_str(&glow_color);
            ctx.arc(x, y, size * 2.0, 0.0, std::f64::consts::PI * 2.0).ok();
            ctx.fill();

            // Core
            let core_color = format!(
                "rgba({}, {}, {}, {})",
                light.color_r, light.color_g, light.color_b,
                brightness
            );
            ctx.begin_path();
            ctx.set_fill_style_str(&core_color);
            ctx.arc(x, y, size, 0.0, std::f64::consts::PI * 2.0).ok();
            ctx.fill();
        }
    }

    pub fn get_snow_count(&self) -> u32 {
        self.snowflakes.len() as u32
    }

    pub fn get_active_sparkle_count(&self) -> u32 {
        self.sparkles.iter().filter(|s| s.active).count() as u32
    }
}

// Performance stats
#[wasm_bindgen]
pub struct PerfStats {
    frame_times: Vec<f64>,
    index: usize,
}

#[wasm_bindgen]
impl PerfStats {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PerfStats {
        PerfStats {
            frame_times: vec![0.0; 60],
            index: 0,
        }
    }

    pub fn record_frame(&mut self, dt: f64) {
        self.frame_times[self.index] = dt;
        self.index = (self.index + 1) % self.frame_times.len();
    }

    pub fn get_avg_fps(&self) -> f64 {
        let avg_dt: f64 = self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64;
        if avg_dt > 0.0 {
            1.0 / avg_dt
        } else {
            60.0
        }
    }
}
