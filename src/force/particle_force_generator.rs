use crate::math::common::assert_vector;
use crate::particle::particle::{Particle, ParticleOps};

pub trait ParticleForceGenerator {
    fn update_force(&self, particle: &mut Particle, duration: f32);
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
