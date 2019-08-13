use rust_game_physics::force::particle_force_generator::ParticleForceGeneratorOps;
use rust_game_physics::force::particle_force_gravity::ParticleForceGravity;
use rust_game_physics::force::particle_force_registry::{
    ParticleForceRegistry, ParticleForceRegistryOps,
};
use rust_game_physics::math::vector::Vector;
use rust_game_physics::math::vector::VectorOps;
use rust_game_physics::particle::particle::Particle;
use rust_game_physics::particle::particle::ParticleOps;

fn main() {
    let   pfg1 = ParticleForceGravity::new();
    let   pfg2 = ParticleForceGravity::new();

    let v1 = Vector::new_vector(1.0, 2.0, 3.0);
    let mut p1 = Particle::new();
    p1.set_inverse_mass(0.1);
    p1.set_velocity(v1);

    let v2 = Vector::new_vector(10.0, 11.0, 12.0);
    let mut p2 = Particle::new();
    p2.set_inverse_mass(10.0);
    p2.set_velocity(v2);

    let mut registry = ParticleForceRegistry::new();

    let (p1_idx, pfg1_idx ) = registry.add(p1, pfg1);
    let (p2_idx, pfg2_idx ) = registry.add(p2, pfg2);

    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p2 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());

    registry.update_forces(2.0);

    registry.get_particle_mut(p1_idx).integrate(2.0);
    registry.get_particle_mut(p2_idx).integrate(2.0);

    println!("now we expected changes ");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p2 velocity = {:?}", registry.get_particle(p2_idx).get_velocity());

}
