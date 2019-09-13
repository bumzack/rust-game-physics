use rust_game_physics::force::particle_force_anchored_spring::ParticleForceAnchoredSpring;
use rust_game_physics::force::particle_force_buoyancy_spring::ParticleForceBuoyancySpring;
use rust_game_physics::force::particle_force_drag::ParticleForceDrag;
use rust_game_physics::force::particle_force_elastic_bungee_spring::ParticleForceElasticBungeeSpring;
use rust_game_physics::force::particle_force_generator::ParticleForceGeneratorOps;
use rust_game_physics::force::particle_force_gravity::ParticleForceGravity;
use rust_game_physics::force::particle_force_registry::{
    ParticleForceRegistry, ParticleForceRegistryOps,
};
use rust_game_physics::force::particle_force_spring::ParticleForceSpring;
use rust_game_physics::math::Tuple4D::Tuple4D;
use rust_game_physics::math::Tuple4D::Tuple4DOps;
use rust_game_physics::particle::particle::Particle;
use rust_game_physics::particle::particle::ParticleOps;

fn main() {
    let mut registry = ParticleForceRegistry::new();

    let anchor = Tuple4D::new_point(10.0, 10.0, 10.0);

    let mut pfg1 = ParticleForceBuoyancySpring::new();
    pfg1.set_liquid_density(10.0);
    pfg1.set_max_depth(4.0);
    pfg1.set_volume(100.0);
    pfg1.set_water_height(3.0);

    let v1 = Tuple4D::new_Tuple4D(1.0, 2.0, 3.0);
    let mut p1 = Particle::new();
    p1.set_inverse_mass(0.1);
    p1.set_velocity(v1);
    p1.set_id(1);

    let p1_idx = registry.add_particle(p1);

    let pfg1_idx = registry.add_particle_force_generator(Box::new(pfg1));

    registry.add_force_for_particle(p1_idx, pfg1_idx);

    println!("initial position and velocity");
    println!(
        "p1 position = {:?}",
        registry.get_particle(p1_idx).get_position()
    );
    println!(
        "p1 velocity = {:?}",
        registry.get_particle(p1_idx).get_velocity()
    );

    registry.update_forces(2.0);
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    println!("");

    println!("after p1 has been integrated1 ");
    println!(
        "p1 position = {:?}",
        registry.get_particle(p1_idx).get_position()
    );
    println!(
        "p1 velocity = {:?}",
        registry.get_particle(p1_idx).get_velocity()
    );

    registry.update_forces(2.0);
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    println!("");

    println!("after p1 has been 2x integrated1 ");
    println!(
        "p1 position = {:?}",
        registry.get_particle(p1_idx).get_position()
    );
    println!(
        "p1 velocity = {:?}",
        registry.get_particle(p1_idx).get_velocity()
    );
}
