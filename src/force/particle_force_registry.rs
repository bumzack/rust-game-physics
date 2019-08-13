use crate::math::common::assert_vector;
use crate::force::particle_force_generator::ParticleForceGenerator;
use crate::particle::particle::Particle;
use crate::force::particle_force_registry_entry::ParticleForceRegistryEntry;

#[derive(Clone, Debug)]
pub struct ParticleForceRegistry<'a, P: ParticleForceGenerator> {
    registrations: Vec<ParticleForceRegistryEntry<'a, P>>,
}

pub trait ParticleForceRegistryOps<'a> {
    fn add(&mut self, p: &'a Particle, gen: &'a ParticleForceGenerator);
    fn remove(&mut self, p: &Particle, gen: &ParticleForceGenerator);
    fn clear(&mut self);
    fn update_forces(&mut self, duration: f32);
}

impl<'a, P: ParticleForceGenerator> ParticleForceRegistryOps<'a> for ParticleForceRegistry<'a, P> {
    fn add(&mut self, p: &'a Particle, gen: &'a ParticleForceGenerator) {
        let entry = ParticleForceRegistryEntry::new(p, gen);
        self.registrations.push(entry);
    }

    fn remove(&mut self, p: &Particle, gen: &ParticleForceGenerator) {
        //TODO: find entry with the two parameters and remove ...
    }

    fn clear(&mut self) {
        self.registrations.clear();
    }

    fn update_forces(&mut self, duration: f32) {
        for entry in self.registrations.iter_mut() {
            entry.get_particle_force_generator().update_force(entry.get_particle_mut(), duration);
        }
    }
}

// TODO
#[test]
fn test_force_registry() {}
