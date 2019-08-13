use crate::force::particle_force_generator::ParticleForceGenerator;
use crate::math::common::assert_vector;
use crate::math::vector::Vector;
use crate::math::vector::VectorOps;
use crate::particle::particle::{Particle, ParticleOps};

pub struct ParticleForceGravity {
    gravity: Vector,
}

impl ParticleForceGenerator for ParticleForceGravity {
    fn update_force(&self, particle: &mut Particle, duration: f32) {
        let m = particle.mass;
        let f = &self.gravity * m;
        particle.add_force(&f);
    }
}

impl ParticleForceGravity {
    pub fn new() -> ParticleForceGravity {
        ParticleForceGravity {
            gravity: Vector::new_vector(0.0, 10.0, 0.0),
        }
    }

    pub fn set_gravity(&mut self, g: Vector) {
        self.gravity = g;
    }

    pub fn get_gravity(&self) -> &Vector {
        &self.gravity
    }

    pub fn get_gravity_mut(&mut self) -> &mut Vector {
        &mut self.gravity
    }
}

// TODO
#[test]
fn test_particle_force_enerator() {}
