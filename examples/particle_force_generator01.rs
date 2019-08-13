use rust_game_physics::math::vector::Vector;
use rust_game_physics::math::vector::VectorOps;
use rust_game_physics::particle::particle::Particle;
use rust_game_physics::particle::particle::ParticleOps;

fn main() {
    let pfg  = ParticleForceGenerator::new();

    let v1 = Vector::new_vector(1.0, 2.0, 3.0);
    let mut p1 = Particle::new();
    p1.set_inverse_mass(0.1);
    p1.set_velocity(v1);

    let v2 = Vector::new_vector(10.0, 11.0, 12.0);
    let mut p2 = Particle::new();
    p2.set_inverse_mass(10.0);
    p2.set_velocity(v2);





}
