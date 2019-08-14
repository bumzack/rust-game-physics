use crate::force::particle_force_generator::ParticleForceGeneratorOps;
use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::{ParticleContainer, ParticleIdx};
use crate::math::common::assert_vector;
use crate::math::vector::Vector;
use crate::math::vector::VectorOps;
use crate::particle::particle::{Particle, ParticleOps};

#[derive(Clone)]
pub struct ParticleForceFakeSpring {
    anchor: Vector,
    spring_constant: f32,
    damping: f32,
}

impl ParticleForceGeneratorOps for ParticleForceFakeSpring {
    fn update_force(
        &self,
        particle: &mut Particle,
        duration: f32,
        all_particles: &ParticleContainer,
    ) {
        if !particle.has_finite_mass() {
            return;
        }

        let mut position = Vector::new_vector_from(particle.get_position());
        position = &position - &self.anchor;

        let gamma = 0.5 * (4.0 * self.spring_constant - self.damping * self.damping).sqrt();
        if gamma == 0.0 {
            return;
        }

        let c =
            &position * (self.damping / (2.0 * gamma)) + particle.get_velocity() * (1.0 / gamma);

        // target position
        let mut target = &position * (gamma * duration).cos() + c * (gamma * duration).sin();
        target = target * (-0.5 * duration * self.damping).exp();

        // resulting acc. --> force
        let accel = (&target - &position) * (1.0 / (duration * duration))
            - particle.get_velocity() * duration;
        particle.add_force(&(&accel * particle.get_mass()));
    }
}

impl ParticleForceFakeSpring {
    pub fn new() -> ParticleForceFakeSpring {
        ParticleForceFakeSpring {
            anchor: Vector::new_vector(0.0, 0.0, 0.0),
            spring_constant: 0.0,
            damping: 0.0,
        }
    }

    pub fn set_anchor(&mut self, anchor: Vector) {
        self.anchor = anchor;
    }

    pub fn get_anchor(&self) -> &Vector {
        &self.anchor
    }

    pub fn set_spring_constant(&mut self, spring_constant: f32) {
        self.spring_constant = spring_constant;
    }
    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping;
    }
}

// TODO
#[test]
fn test_particle_force_enerator() {}
