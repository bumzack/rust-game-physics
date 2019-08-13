use std::collections::HashMap;

use crate::force::{ParticleForceGeneratorOpsIdx, ParticleIdx};
use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::math::common::assert_vector;
use crate::particle::particle::{Particle, ParticleOps};

#[derive(Clone, Debug)]
pub struct ParticleForceRegistry<PFG: ParticleForceGeneratorOps> {
    particle_force_generators: Vec<PFG>,
    particles: Vec<Particle>,
    registry: HashMap<ParticleForceGeneratorOpsIdx, Vec<ParticleIdx>>,
}

pub trait ParticleForceRegistryOps<PFG: ParticleForceGeneratorOps> {
    fn add_particle(&mut self, p: Particle) -> ParticleIdx;
    fn add_particle_force_generator(&mut self, g: PFG) -> ParticleForceGeneratorOpsIdx;

    fn add_force_for_particle(&mut self, p_idx: ParticleIdx, g_idx: ParticleForceGeneratorOpsIdx);


    // fn remove(&mut self,  gen:vPFG);
    fn clear(&mut self);
    fn update_forces(&mut self, duration: f32);

    fn get_particle(&self, idx: ParticleIdx) -> &Particle;
    fn get_particle_mut(&mut self, idx: ParticleIdx) -> &mut Particle;

    fn get_particle_force_generators(&self, idx: ParticleForceGeneratorOpsIdx) -> &PFG;
    fn get_particle_force_generators_mut(&mut self, idx: ParticleForceGeneratorOpsIdx) -> &mut PFG;
}

impl<PFG: ParticleForceGeneratorOps> ParticleForceRegistryOps<PFG> for ParticleForceRegistry<PFG> {
    fn add_particle(&mut self, p: Particle) -> ParticleIdx {
        self.particles.push(p);
        self.particles.len() - 1
    }

    fn add_particle_force_generator(&mut self, g: PFG) -> ParticleForceGeneratorOpsIdx {
        self.particle_force_generators.push(g);
        self.particle_force_generators.len() - 1
    }

    fn add_force_for_particle(&mut self, p_idx: ParticleIdx, g_idx: ParticleForceGeneratorOpsIdx) {
        if !self.registry.contains_key(&g_idx) {
            self.registry.insert(g_idx, Vec::new());
        }
        let v = &mut self.registry.get_mut(&g_idx).unwrap();
        v.push(p_idx);
    }

    fn clear(&mut self) {
        // TODO clear the particles and generators too?
        // self.particle_force_generators.clear();
        // self.particles.clear();
        self.registry.clear();
    }

    fn update_forces(&mut self, duration: f32) {
        for (gen_idx, particles_indices) in self.registry.iter() {
            for p_idx in particles_indices.iter() {
                let mut p = self.particles[*p_idx];
                let pfg = &self.particle_force_generators[*gen_idx];
                pfg.update_force(&mut p, duration);
            }
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
            registry: Default::default(),
        }
    }
}

// TODO
#[test]
fn test_force_registry() {}
