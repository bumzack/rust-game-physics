use crate::force::particle_force_generator::ParticleForceGenerator;
use crate::math::common::assert_vector;
use crate::particle::particle::{Particle, ParticleOps};

#[derive(Clone, Debug)]
pub struct ParticleForceRegistry<'a, PFG: ParticleForceGenerator> {
    particles: Vec<&'a Particle>,
    particle_force_generators: Vec<&'a PFG>,

}

pub trait ParticleForceRegistryOps<'a, PFG: ParticleForceGenerator> {
    fn add(&mut self, p: &'a Particle, gen: &'a PFG);
    fn remove(&mut self, p: &'a Particle, gen: &'a PFG);
    fn clear(&mut self);
    fn update_forces(&mut self, duration: f32);
}

impl<'a, PFG: ParticleForceGenerator> ParticleForceRegistryOps<'a, PFG> for ParticleForceRegistry<'a, PFG> {
    fn add(&mut self, p: &'a Particle, gen: &'a PFG) {
        self.particles.push(p);
        self.particle_force_generators.push(gen);
    }

    fn remove(&mut self, p: &'a Particle, gen: &'a PFG) {
        //TODO: find entry with the two parameters and remove ...
    }

    fn clear(&mut self) {
        self.particles.clear();
        self.particle_force_generators.clear();
    }

    fn update_forces(&mut self, duration: f32) {
        for i in 0..self.particles.len() {
            let pfg = self.particle_force_generators[i];
            let mut p = self.particles[i];
            pfg.update_force(  &mut p, duration);
        }
    }
}

impl<'a, PFG: ParticleForceGenerator> ParticleForceRegistry<'a, PFG> {
  pub  fn new() -> ParticleForceRegistry<'a, PFG> {
        ParticleForceRegistry {
            particles: vec![],
            particle_force_generators: vec![],
        }
    }
}


// TODO
#[test]
fn test_force_registry() {}
