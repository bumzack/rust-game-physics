use std::collections::HashMap;
use std::rc::Rc;

use crate::force::{ParticleForceGeneratorOpsIdx, ParticleIdx};
use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::math::common::assert_vector;
use crate::particle::particle::{Particle, ParticleOps};

#[derive(Clone)]
pub struct ParticleForceRegistry {
    particle_force_generators: Vec<Box<ParticleForceGeneratorOps>>,
    particles: Vec<Particle>,
    registry: HashMap<ParticleForceGeneratorOpsIdx, Vec<ParticleIdx>>,
}

pub trait ParticleForceRegistryOps {
    fn add_particle(&mut self, p: Particle) -> ParticleIdx;
    fn add_particle_force_generator(&mut self, g: Box<ParticleForceGeneratorOps>) -> ParticleForceGeneratorOpsIdx;

    fn add_force_for_particle(&mut self, p_idx: ParticleIdx, g_idx: ParticleForceGeneratorOpsIdx);


    // fn remove(&mut self,  gen:vPFG);
    fn clear(&mut self);
    fn update_forces(&mut self, duration: f32);

    fn get_particle(&self, idx: ParticleIdx) -> &Particle;
    fn get_particle_mut(&mut self, idx: ParticleIdx) -> &mut Particle;

    fn get_particle_force_generators(&self, idx: ParticleForceGeneratorOpsIdx) -> &Box<ParticleForceGeneratorOps>;
    fn get_particle_force_generators_mut(&mut self, idx: ParticleForceGeneratorOpsIdx) -> &mut Box<ParticleForceGeneratorOps>;
}

impl ParticleForceRegistryOps for ParticleForceRegistry {
    fn add_particle(&mut self, p: Particle) -> ParticleIdx {
        self.particles.push(p);
        self.particles.len() - 1
    }

    fn add_particle_force_generator(&mut self, g: Box<ParticleForceGeneratorOps>) -> ParticleForceGeneratorOpsIdx {
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
                let pfg = &self.particle_force_generators.get(*gen_idx).unwrap();
                let mut p = &mut self.particles.get_mut(*p_idx).unwrap();
                println!("update_forces            gen_idx = {}, p_idx = {}", gen_idx, p_idx);
                pfg.update_force(p, duration);
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

    fn get_particle_force_generators(&self, idx: ParticleForceGeneratorOpsIdx) -> &Box<ParticleForceGeneratorOps> {
        // TODO: index check?
        &self.particle_force_generators[idx]
    }

    fn get_particle_force_generators_mut(&mut self, idx: usize) -> &mut Box<ParticleForceGeneratorOps> {
        &mut self.particle_force_generators[idx]
    }
}

impl ParticleForceRegistry {
    pub fn new() -> ParticleForceRegistry {
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
