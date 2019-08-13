use crate::math::common::assert_vector;
use crate::particle::particle::{Particle, ParticleOps};
use crate::force::particle_force_generator::ParticleForceGenerator;
use crate::math::vector::Vector;
use crate::math::vector::VectorOps;

pub struct ParticleForceGravity {
    gravity: Vector,
}

impl ParticleForceGenerator for ParticleForceGravity {
    fn update_force(&self, mut particle: &mut Particle, duration: f32) {
        particle.add_force(&(&self.gravity * particle.get_mass()));
    }
}

impl ParticleForceGravity {
    pub fn new() -> ParticleForceGravity {
        ParticleForceGravity {
            gravity: Vector::new_vector(0.0, 10.0, 0.0),
        }
    }
}
//
//impl Force {
//    // seems good default values?
//    pub fn new() -> Particle {
//        Particle {
//            position: Vector::new_point(0.0, 0.0, 0.0),
//            velocity: Vector::new_vector(0.0, 0.0, 0.0),
//            acceleration: Vector::new_vector(0.0, 0.0, 0.0),
//            force_accum: Vector::new_vector(0.0, 0.0, 0.0),
//            damping: 0.999,
//            inverse_mass: 0.0,
//        }
//    }
//}
//


// TODO
#[test]
fn test_particle_force_enerator() {}
