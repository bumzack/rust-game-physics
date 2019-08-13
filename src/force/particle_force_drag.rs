use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::math::common::assert_vector;
use crate::math::vector::Vector;
use crate::math::vector::VectorOps;
use crate::particle::particle::{Particle, ParticleOps};

pub struct ParticleForceDrag {
    k1: f32,
    k2: f32,
}

impl<'a> ParticleForceGeneratorOps for ParticleForceDrag {
    fn update_force(&self, particle: &mut Particle, duration: f32) {
        let mut f = particle.get_velocity();
//        let mut drag_coeff = f.magnitude();
//        drag_coeff = self.k1 * drag_coeff + self.k2 * drag_coeff * drag_coeff;
//        f.normalize();
//        f = &(-drag_coeff * f);
//        particle.add_force(&f);
    }
}

impl<'a> ParticleForceDrag {
    pub fn new() -> ParticleForceDrag {
        // defualt: no drag ?!
        ParticleForceDrag {
            k1: 1.0,
            k2: 1.0,
        }
    }

    pub fn set_drag_k1(&mut self, k1: f32) {
        self.k1 = k1;
    }
    pub fn set_drag_k2(&mut self, k2: f32) {
        self.k2 = k2;
    }

    pub fn get_drag_k1(&self) -> f32 {
        self.k1
    }

    pub fn get_drag_k2(&self) -> f32 {
        self.k2
    }
}

// TODO
#[test]
fn test_particle_force_enerator() {}
