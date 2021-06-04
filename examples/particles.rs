use std::{ops::AddAssign};

use tin::{Double, UShort, color::{TColor, TinColor}, draw::*, event::TinEvent, frame::TinFrame, key::TinKey, random::random, run, scene::TScene, vector2::TVector2, view::TinView};

#[derive(Clone, Debug)]
struct Particle {
    center: TVector2,
    size: Double,
    rotation: Double,
    velocity: TVector2,
    lifetime: UShort,
    should_die: bool
}

const PARTICLE_LIFETIME: UShort = 240;

impl Default for Particle {
    fn default() -> Self {
        let mut v = TVector2::new_from_angle(random(0.0,360.0 ).to_radians());
        v.set_magnitude(0.0013);
        let r = tin::random::random(0.0, std::f64::consts::FRAC_2_PI);
        Self::new(
            TVector2::new_from_xy(0.0, 0.0),
            0.2,
            r,
            v
        )
    }
}

impl Particle {
    fn new(center: TVector2, size: Double, rotation: Double, velocity: TVector2) -> Self {
        Particle {
            center: center,
            size: size,
            rotation: rotation,
            velocity: velocity,
            lifetime: PARTICLE_LIFETIME,
            should_die: false
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
        let mut vecs: [TVector2; 3] = [
            TVector2::new_from_angle(240_f64.to_radians()),
            TVector2::new_from_angle(0_f64.to_radians()),
            TVector2::new_from_angle(120_f64.to_radians())
        ];
        for vec in &mut vecs {
            vec.rotate(self.rotation);
            vec.set_magnitude(self.size);
            vec.add_assign(self.center);
        }

        let a = tin::calculation::remap(self.lifetime as Double, PARTICLE_LIFETIME as Double, 0.0, 1.0, 0.0);
        fill_color_from_rgba(&0.7, &0.7, &0.7, &a);
        triangle(vecs[0].x, vecs[0].y, vecs[1].x, vecs[1].y, vecs[2].x, vecs[2].y)
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


struct Scene {
    time_elapsed: Double,
    particles: Vec<Particle>
}

impl TScene for Scene {

    fn setup(&mut self) {
        self.particles = Vec::<Particle>::new();

        for _ in 0..5 {
            self.particles.generate_particle();
        }
        
    }

    fn update(&mut self) {
        const RATE_OF_CHANGE: Double = 0.0000001;
        background(&0.5, &0.5, &0.5);

        let color: TinColor = TinColor::new_from_rgba(1.0, 0.1, 0.1, 1.0);
        fill_color_from_color(&color);

        self.particles.update();

        for p in &mut self.particles {
            p.velocity.set_magnitude(p.velocity.get_magnitude() + RATE_OF_CHANGE);
        }
        self.particles.draw();

        

        self.time_elapsed = self.time_elapsed - RATE_OF_CHANGE;
    }
}

fn handler_test(event: TinEvent, scene: &mut Scene, _view: &mut TinView) {
    match event {
        TinEvent::KeyDown(TinKey::D) => {scene.time_elapsed += 0.2}
        _ => {}
    } 
}

fn main() {
    let frame = TinFrame::new(750, 600);
    let scene = Scene{time_elapsed: 0.0, particles: Vec::new()};
    let test_view = TinView::new("title", frame);

    std::process::exit(
        match run(test_view, scene, handler_test) { 
            Ok(_) => 0,
            Err(err) => {
                eprintln!("error: {:?}", err);
                1
            }
        }
    );
}

