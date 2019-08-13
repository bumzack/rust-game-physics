use rust_game_physics::force::particle_force_generator::ParticleForceGeneratorOps;
use rust_game_physics::force::particle_force_gravity::ParticleForceGravity;
use rust_game_physics::force::particle_force_registry::{
    ParticleForceRegistry, ParticleForceRegistryOps,
};
use rust_game_physics::math::vector::Vector;
use rust_game_physics::math::vector::VectorOps;
use rust_game_physics::particle::particle::Particle;
use rust_game_physics::particle::particle::ParticleOps;
use rust_game_physics::force::particle_force_drag::ParticleForceDrag;

fn main() {
    let pfg1 = ParticleForceGravity::new();
    let pfg2 = ParticleForceGravity::new();

    let mut pfg3 = ParticleForceDrag::new();
    pfg3.set_drag_k1(0.95);
    pfg3.set_drag_k2(0.5);

    let v1 = Vector::new_vector(1.0, 2.0, 3.0);
    let mut p1 = Particle::new();
    p1.set_inverse_mass(0.1);
    p1.set_velocity(v1);

    let v2 = Vector::new_vector(1.0, 2.0, 3.0);
    let mut p2 = Particle::new();
    p2.set_inverse_mass(0.1);
    p2.set_velocity(v2);

    let mut registry = ParticleForceRegistry::new();

    let p1_idx = registry.add_particle(p1);
    let p2_idx = registry.add_particle(p2);

    let pfg1_idx = registry.add_particle_force_generator(pfg1);
    let pfg2_idx = registry.add_particle_force_generator(pfg2);
    let pfg3_idx = registry.add_particle_force_generator(pfg3);

    registry.add_force_for_particle(p1_idx, pfg1_idx);
    registry.add_force_for_particle(p2_idx, pfg2_idx);
    registry.add_force_for_particle(p2_idx, pfg3_idx);


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
