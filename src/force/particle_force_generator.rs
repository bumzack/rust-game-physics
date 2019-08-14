use crate::force::ParticleIdx;
use crate::math::common::assert_vector;
use crate::particle::particle::{Particle, ParticleOps};

pub trait ParticleForceGeneratorOps: ParticleForceGeneratorOpsClone {
    fn update_force(&self, particle: &mut Particle, duration: f32);
    // fn update_force_two_particles(&self, particle: &mut Particle, other_particle: &Particle, duration: f32) { println!("DEFAULT update_force_two_particles ") }
    fn needs_other_particle(&self) -> bool { false }
    fn get_other_particle_idx(&self) -> ParticleIdx { 0 }
}

pub trait ParticleForceGeneratorOpsClone {
    fn clone_box(&self) -> Box<ParticleForceGeneratorOps>;
}

impl<T> ParticleForceGeneratorOpsClone for T
    where
        T: 'static + ParticleForceGeneratorOps + Clone,
{
    fn clone_box(&self) -> Box<ParticleForceGeneratorOps> {
        Box::new(self.clone())
    }
}


// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<ParticleForceGeneratorOps> {
    fn clone(&self) -> Box<ParticleForceGeneratorOps> {
        self.clone_box()
    }
}

