use crate::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use crate::force::particle_force_types::ParticleIdx;
use crate::math::common::assert_vector;
use crate::math::vector::Vector;
use crate::math::vector::VectorOps;
use crate::particle::particle::Particle;
use crate::particle::particle::ParticleOps;

pub struct ParticleContact {
    particle: Vec<Option<ParticleIdx>>,
    restitution: f32,
    penetration: f32,
    contact_normal: Vector,

}

impl ParticleContact {
    pub fn new() -> ParticleContact {
        ParticleContact {
            particle: vec![None; 2],
            restitution: 0.0,
            penetration: 0.0,
            contact_normal: Vector::new_vector(0.0, 0.0, 0.0),
        }
    }

    pub fn resolve(&self, duration: f32, registry: &mut ParticleForceRegistry) {
        self.resolve_velocity(duration, registry);
        self.resolve_interpenetration(duration);
    }

    pub fn calculate_separating_velocity(&self, registry: &mut ParticleForceRegistry) -> f32 {
        let p0 = registry.get_particle(self.particle[0].unwrap());
        let mut relative_velocity =
            Vector::new_vector_from(p0.get_velocity());

        if self.particle[1].is_some() {
            let p1 = registry.get_particle(self.particle[1].unwrap());
            relative_velocity = &relative_velocity - p1.get_velocity();
        }
        println!("calculate_separating_velocity = {:?}",  relative_velocity ^ self.contact_normal);

        relative_velocity ^ self.contact_normal
    }

    fn resolve_velocity(&self, duration: f32, registry: &mut ParticleForceRegistry) {
        let separating_velocity = self.calculate_separating_velocity(registry);

        if separating_velocity > 0.0 {
            // the contact is either separating or stationary -> simple return
            return;
        }
        let new_sep_velocity = -separating_velocity * self.restitution;
        let delta_velocity = new_sep_velocity - separating_velocity;

        // apply change in veloctiy to all objects
        let p0 = registry.get_particle(self.particle[0].unwrap());

        let mut total_inverse_mass = p0.get_inverse_mass();
        println!("total_inverse_mass = {:?}",  total_inverse_mass );

        if self.particle[1].is_some() {
            let p1 = registry.get_particle(self.particle[1].unwrap());

            total_inverse_mass = total_inverse_mass + p1.get_inverse_mass();
        }
        if (total_inverse_mass <= 0.0) {
            return;
        }

        // calc impulse to apply
        let impulse = delta_velocity / total_inverse_mass;
        println!("impulse = {:?}",  impulse );

        let impulse_per_mass = &self.contact_normal * impulse;

        // amount of imp per unit of inv. mass
        let impulse_per_inverse_mass = &self.contact_normal * impulse;

        let particle0_new_velocity =
            p0.get_velocity() + &(&impulse_per_inverse_mass * p0.get_inverse_mass());
        println!("particle0: old_velocity = {:?}",   p0.get_velocity()  );
        println!("particle0_new_velocity = {:?}",  particle0_new_velocity );
        registry.set_velocity(self.particle[0].unwrap(), particle0_new_velocity);
        //         p0.set_velocity(particle0_new_velocity);

        if self.particle[1].is_some() {
            // particle 1 goes into the inverse direction -> -
            let p1 = registry.get_particle(self.particle[1].unwrap());
            let particle1_new_velocity =
                p1.get_velocity() + &(&impulse_per_inverse_mass * (-p1.get_inverse_mass()));
            println!("particle1: old_velocity = {:?}",   p1.get_velocity()  );
            println!("particle1_new_velocity = {:?}",  particle1_new_velocity );
            registry.set_velocity(self.particle[1].unwrap(), particle1_new_velocity);
        }
    }

    pub fn resolve_interpenetration(&self, duration: f32) {}

    pub fn set_particle1(&mut self, p1: ParticleIdx) {
        self.particle[0] = Some(p1);
    }

    pub fn set_particle2(&mut self, p2: ParticleIdx) {
        self.particle[1] = Some(p2);
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution;
    }

    pub fn set_penetration(&mut self, penetration: f32) {
        self.penetration = penetration;
    }

    pub fn set_contact_normal(&mut self, contact_normal: Vector) {
        self.contact_normal = contact_normal;
    }
}

// TODO
#[test]
fn test_particle_contact() {}
