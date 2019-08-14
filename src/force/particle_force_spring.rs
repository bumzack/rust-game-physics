use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::ParticleIdx;
use crate::math::common::assert_vector;
use crate::math::vector::Vector;
use crate::math::vector::VectorOps;
use crate::particle::particle::{Particle, ParticleOps};

#[derive(Clone)]
pub struct ParticleForceSpring {
    other: ParticleIdx,
    spring_constant: f32,
    rest_length: f32,
    registry: Option<ParticleForceRegistry>,
}

impl ParticleForceGeneratorOps for ParticleForceSpring {
    fn update_force(&self, particle: &mut Particle, duration: f32) {
        let other_particle = self.registry.as_ref().unwrap().get_particle(self.other);
        let mut f = Vector::new_vector_from(particle.get_position());
        f = &f - other_particle.get_position();

        let mut magnitude = f.magnitude();
        magnitude = (magnitude - self.rest_length).abs();
        magnitude = magnitude * self.spring_constant;

        // calc. final force and apply
        f.normalize();
        f = f * (-magnitude);
        println!("add force from spring: {:?},    particle.id = {}", f, particle.get_id());
        particle.add_force(&f);
    }

    fn needs_other_particle(&self) -> bool { true }
    fn get_other_particle_idx(&self) -> ParticleIdx { self.other }
}

impl ParticleForceSpring {
    pub fn new() -> ParticleForceSpring {
        ParticleForceSpring {
            other: 99999,
            spring_constant: 0.0,
            rest_length: 0.0,
            registry: None,
        }
    }

    pub fn set_other(&mut self, other_idx: ParticleIdx) {
        self.other = other_idx;
    }

    pub fn get_other(&self) -> ParticleIdx {
        self.other
    }

    pub fn set_registry(&mut self, r: ParticleForceRegistry) {
        self.registry = Some(r);
    }
}

// TODO
#[test]
fn test_particle_force_enerator() {}
