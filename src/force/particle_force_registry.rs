use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::force::{ParticleForceGeneratorOpsIdx, ParticleIdx};
use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::math::common::assert_vector;
use crate::particle::particle::{Particle, ParticleOps};

#[derive(Clone, Debug)]
pub struct ParticleForceRegistry<PFG: ParticleForceGeneratorOps> {
    particle_force_generators: Vec<PFG>,
    particles: Vec<Particle>,
}

pub trait ParticleForceRegistryOps<PFG: ParticleForceGeneratorOps> {
    fn add(&mut self, p: Particle, g: PFG) -> (ParticleIdx, ParticleForceGeneratorOpsIdx);
    // fn remove(&mut self,  gen:vPFG);
    fn clear(&mut self);
    fn update_forces(&mut self, duration: f32);

    fn get_particle(&self, idx: ParticleIdx) -> &Particle;
    fn get_particle_mut(&mut self, idx: ParticleIdx) -> &mut Particle;

    fn get_particle_force_generators(&self, idx: ParticleForceGeneratorOpsIdx) -> &PFG;
    fn get_particle_force_generators_mut(&mut self, idx: ParticleForceGeneratorOpsIdx) -> &mut PFG;
}

impl<PFG: ParticleForceGeneratorOps> ParticleForceRegistryOps<PFG> for ParticleForceRegistry<PFG> {
    fn add(&mut self, p: Particle, g: PFG) -> (ParticleIdx, ParticleForceGeneratorOpsIdx) {
        self.particles.push(p);
        self.particle_force_generators.push(g);
        (self.particles.len() - 1, self.particle_force_generators.len() - 1)
    }


    //    fn remove(&mut self, p: &'a Particle, gen: &'a PFG) {
    //        //TODO: find entry with the two parameters and remove ...
    //    }

    fn clear(&mut self) {
        self.particle_force_generators.clear();
        self.particles.clear();
    }

    fn update_forces(&mut self, duration: f32) {
        for i in 0..self.particles.len() {
            let mut p = self.particles[i];
            let pfg = &self.particle_force_generators[i];
            pfg.update_force(&mut p, duration);
        }
    }

    fn get_particle(&self, idx: ParticleIdx) -> &Particle {
        // TODO: index check?
        &self.particles[idx]
    }

    fn get_particle_mut(&mut self, idx: usize) -> &mut Particle {
        &mut self.particles[idx]
    }

    fn get_particle_force_generators(&self, idx: ParticleForceGeneratorOpsIdx) -> &PFG {
        // TODO: index check?
        &self.particle_force_generators[idx]
    }

    fn get_particle_force_generators_mut(&mut self, idx: usize) -> &mut PFG {
        &mut self.particle_force_generators[idx]
    }
}

impl<PFG: ParticleForceGeneratorOps> ParticleForceRegistry<PFG> {
    pub fn new() -> ParticleForceRegistry<PFG> {
        ParticleForceRegistry {
            particle_force_generators: Vec::new(),
            particles: Vec::new(),
        }
    }
}

// TODO
#[test]
fn test_force_registry() {}
