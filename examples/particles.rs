use std::ops::AddAssign;
use tin::view::TView;
use tin::*;
use tin::{
    color::{TColor, TinColor},
    draw::*,
    random::random,
    vector2::TinVector2,
};

#[derive(Clone, Debug)]
struct Particle {
    center: TinVector2,
    size: Double,
    rotation: Double,
    velocity: TinVector2,
    lifetime: UShort,
    should_die: bool,
}

const PARTICLE_LIFETIME: UShort = 240;

impl Default for Particle {
    fn default() -> Self {
        let mut v = TinVector2::from_angle(random(0.0, 360.0).to_radians());
        v.set_magnitude(0.0013);
        let r = random(0.0, std::f64::consts::FRAC_2_PI);
        Self::new(TinVector2::from_xy(0.0, 0.0), 0.2, r, v)
    }
}

impl Particle {
    fn new(center: TinVector2, size: Double, rotation: Double, velocity: TinVector2) -> Self {
        Particle {
            center,
            size,
            rotation,
            velocity,
            lifetime: PARTICLE_LIFETIME,
            should_die: false,
        }
    }

    fn update(&mut self) {
        if self.lifetime <= 0 {
            self.should_die = true
        } else {
            self.lifetime -= 1;
        }
        self.center += self.velocity;
    }

    fn draw(&self) {
        let mut vecs: [TinVector2; 3] = [
            TinVector2::from_angle(240_f64.to_radians()),
            TinVector2::from_angle(0_f64.to_radians()),
            TinVector2::from_angle(120_f64.to_radians()),
        ];
        for vec in &mut vecs {
            vec.rotate(self.rotation);
            vec.set_magnitude(self.size);
            vec.add_assign(self.center);
        }

        let a = tin::calculation::remap(
            self.lifetime as Double,
            PARTICLE_LIFETIME as Double,
            0.0,
            1.0,
            0.0,
        );
        println!("{}", a);
        fill_color_from_rgba(0.7, 0.7, 0.7, a);
        triangle(
            vecs[0].x, vecs[0].y, vecs[1].x, vecs[1].y, vecs[2].x, vecs[2].y,
        )
    }
}

trait ParticleSystem {
    fn get_particles(&self) -> Vec<Particle>;
    fn update(&mut self);
    fn generate_particle(&mut self);
    fn clear_particles(&mut self);
    fn draw(&self);
}

impl ParticleSystem for Vec<Particle> {
    fn get_particles(&self) -> Vec<Particle> {
        self.to_vec()
    }

    fn update(&mut self) {
        self.retain(|p| !p.should_die);
        for p in self {
            p.update();
        }
    }

    fn generate_particle(&mut self) {
        let p = Particle::default();
        self.push(p);
    }

    fn clear_particles(&mut self) {
        self.clear()
    }

    fn draw(&self) {
        for p in self {
            p.draw()
        }
    }
}

use tin::scene::TScene;
struct Scene {
    time_elapsed: Double,
    particles: Vec<Particle>,
}

impl TScene for Scene {
    fn setup() -> Self {
        let mut scene = Self {
            time_elapsed: 0.0,
            particles: Vec::new(),
        };
        scene.particles = Vec::<Particle>::new();

        for _ in 0..5 {
            scene.particles.generate_particle();
        }
        return scene;
    }

    fn update(&mut self) {
        const RATE_OF_CHANGE: Double = 0.0000001;
        background(0.5, 0.5, 0.5);

        let color: TinColor = TinColor::from_rgba(1.0, 0.1, 0.1, 1.0);
        color.fill_color();

        self.particles.update();

        for p in &mut self.particles {
            p.velocity
                .set_magnitude(p.velocity.get_magnitude() + RATE_OF_CHANGE);
        }
        self.particles.draw();

        self.time_elapsed -= RATE_OF_CHANGE;
    }

    fn on_event(&mut self, event: tin::TinEvent) {
        match event {
            TinEvent::KeyDown(TinKey::D) => self.time_elapsed += 0.2,
            _ => {}
        }
    }
}

fn main() {
    Tin::<Scene>::app()
        .view(tin::view::TinView::from_dimensions("title", 750, 600))
        .run()
        .unwrap();
}
