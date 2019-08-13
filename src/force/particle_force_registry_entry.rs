use crate::math::common::assert_vector;

use crate::force::particle_force_generator::ParticleForceGenerator;
use crate::particle::particle::Particle;


#[derive(Clone, Debug)]
pub struct ParticleForceRegistryEntry<'a, P: ParticleForceGenerator> {
    particle: &'a Particle,
    particle_force_generator: &'a P,
}

impl<'a, P: ParticleForceGenerator> ParticleForceRegistryEntry<'a, P> {
    pub fn new(p: &'a Particle, gen: &'a P) -> ParticleForceRegistryEntry<'a, P> {
        ParticleForceRegistryEntry {
            particle: p,
            particle_force_generator: &gen,
        }
    }

    pub fn get_particle_mut(&mut self) -> &mut Particle {
        &mut self.particle
    }

    pub fn get_particle_force_generator(&self) -> &P {
        &self.particle_force_generator
    }
}

// TODO
#[test]
fn test_force_registry() {}
