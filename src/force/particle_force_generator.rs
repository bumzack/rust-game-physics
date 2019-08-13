use crate::math::common::assert_vector;
use crate::particle::particle::{Particle, ParticleOps};

pub trait ParticleForceGenerator {
    fn update_force(&self, particle: &mut Particle, duration: f32);
}

