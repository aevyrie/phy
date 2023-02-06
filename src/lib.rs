use std::time::Duration;

use constraints::Constraint;
use glam::Vec3A;
use particles::Particle;
use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub mod constraints;
pub mod particles;

#[cfg(feature = "bevy_integration")]
pub mod bevy;

pub fn simulate_iteration(
    delta_t: Duration,
    substeps: usize,
    particle_list: &mut Vec<Particle>,
    constraint_list: &Vec<Box<dyn Constraint>>,
) {
    let delta_t_s = (delta_t.as_secs_f32()) / (substeps as f32);

    for _substep in 0..substeps {
        // Simple implicit Euler integration to estimate the next position based on velocity
        implicit_euler_integration(particle_list, delta_t_s);

        // Solve all constraints, finding new positions for all particles
        for constraint in constraint_list.iter() {
            constraint.solve(particle_list, delta_t_s);
        }

        // Update velocities using updated positions
        particle_list.par_iter_mut().for_each(|particle| {
            particle.vel = (particle.pos - particle.prev_pos) / delta_t_s;
        });
    }
}

const G: [f32; 3] = [0.0, -9.81, 0.0];

fn implicit_euler_integration(particle_list: &mut Vec<Particle>, delta_t_s: f32) {
    particle_list.par_iter_mut().for_each(|particle| {
        particle.prev_pos = particle.pos;
        particle.vel += delta_t_s * Vec3A::from(G); // External forces
        particle.pos += delta_t_s * particle.vel;
    });
}
