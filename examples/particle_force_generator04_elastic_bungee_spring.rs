use game_physics::prelude::*;

use math::prelude::*;

fn main() {
    let mut registry = ParticleForceRegistry::new();

    let anchor = Tuple4D::new_point(10.0, 10.0, 10.0);

    let mut pfg1 = ParticleForceElasticBungeeSpring::new();
    // TODO???  pfg1.set(9.0);
    pfg1.set_spring_constant(10.0);

    let v1 = Tuple4D::new_vector(1.0, 2.0, 3.0);
    let mut p1 = Particle::new();
    p1.set_inverse_mass(0.1);
    p1.set_velocity(v1);
    p1.set_id(1);

    let v2 = Tuple4D::new_vector(-1.0, -2.0, -3.0);
    let mut p2 = Particle::new();
    p2.set_inverse_mass(0.1);
    p2.set_velocity(v2);
    p2.set_id(2);

    let p1_idx = registry.add_particle(p1);
    let p2_idx = registry.add_particle(p2);

    pfg1.set_other(p2_idx);

    let pfg1_idx = registry.add_particle_force_generator(Box::new(pfg1));

    registry.add_force_for_particle(p1_idx, pfg1_idx);

    println!("initial position and velocity");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());

    registry.update_forces(2.0);
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    println!("");

    println!("after p1 has been integrated1 ");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());

    registry.update_forces(2.0);
    println!("");
    registry.get_particle_mut(p1_idx).integrate(2.0);
    println!("");

    println!("after p1 has been 2x integrated1 ");
    println!("p1 position = {:?}", registry.get_particle(p1_idx).get_position());
    println!("p1 velocity = {:?}", registry.get_particle(p1_idx).get_velocity());
}
