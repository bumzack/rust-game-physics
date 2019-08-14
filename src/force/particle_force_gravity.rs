use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::math::common::assert_vector;
use crate::math::vector::Vector;
use crate::math::vector::VectorOps;
use crate::particle::particle::{Particle, ParticleOps};
use crate::force::particle_force_registry::ParticleForceRegistry;

#[derive(Clone)]
pub struct ParticleForceGravity {
    gravity: Vector,
}

impl<'a> ParticleForceGeneratorOps for ParticleForceGravity {
    fn update_force(&self, particle: &mut Particle, duration: f32,  all_particles: &Vec<Particle>) {
        if !particle.has_finite_mass() {
            return;
        }
        let f = &self.gravity * particle.get_mass();
        println!(
            "GRAVITY           add force from gravity: {:?}         particle.id = {}",
            f,
            particle.get_id()
        );
        particle.add_force(&f);
    }
}

impl<'a> ParticleForceGravity {
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
