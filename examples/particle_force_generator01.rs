use rust_game_physics::math::vector::Vector;
use rust_game_physics::math::vector::VectorOps;
use rust_game_physics::particle::particle::Particle;
use rust_game_physics::particle::particle::ParticleOps;
use rust_game_physics::force::particle_force_registry::{ParticleForceRegistry, ParticleForceRegistryOps};
use rust_game_physics::force::particle_force_gravity::ParticleForceGravity;

fn main() {
    let pfg = ParticleForceGravity::new();

    let v1 = Vector::new_vector(1.0, 2.0, 3.0);
    let mut p1 = Particle::new();
    p1.set_inverse_mass(0.1);
    p1.set_velocity(v1);

    let v2 = Vector::new_vector(10.0, 11.0, 12.0);
    let mut p2 = Particle::new();
    p2.set_inverse_mass(10.0);
    p2.set_velocity(v2);


    let mut registry = ParticleForceRegistry::new();

    registry.add(&p1, &pfg);
    registry.add(&p2, &pfg);
}
