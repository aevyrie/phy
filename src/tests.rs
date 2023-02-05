use std::time::Duration;

use crate::{
    constraints::{Distance, Fixed},
    particles::Particle,
    simulate_iteration,
};
use glam::Vec3A;

//impl IntoIterator for Particles>

#[test]
fn triple_pendulum() {
    let mut particles = vec![
        Particle::stationary(Vec3A::ZERO, 1.0),
        Particle::stationary([0.0, -0.5, 0.0].into(), 1.0),
        Particle::stationary([0.0, -1.0, 0.0].into(), 1.0),
        Particle::stationary([0.0, -1.5, 0.0].into(), 1.0),
    ];

    let mut constraints = vec![
        Fixed::new(Vec3A::ZERO, 0, 0.0).into(),
        Distance::new(0.5, 0, 1, 0.0).into(),
        Distance::new(0.5, 1, 2, 0.0).into(),
        Distance::new(0.5, 2, 3, 0.0).into(),
    ];

    let d_t = Duration::from_secs_f32(1.0 / 60.0);

    simulate_iteration(d_t, 1000, &mut particles, &mut constraints);

    let final_pos = particles[3].pos.y;
    let error = (final_pos + 1.5).abs();
    dbg!(error);
    assert!(error < 0.0001)
}

/// This is an n-pendulum with 100 links, stacked vertically - not hanging!
#[test]
fn stacked_pendulum() {
    let mut particles = vec![Particle::stationary(Vec3A::ZERO, 1.0)];
    let mut constraints = vec![Fixed::new(Vec3A::ZERO, 0, 0.0).into()];

    for i in 1..=100 {
        particles.push(Particle::stationary([0.0, 1.0 * i as f32, 0.0].into(), 1.0));
        constraints.push(Distance::new(1.0, i - 1, i, 0.0).into());
    }

    let d_t = Duration::from_secs_f32(1.0);

    simulate_iteration(d_t, 2000, &mut particles, &mut constraints);
    let final_pos = particles[100].pos.y;
    let error = (final_pos - 100.0).abs();
    dbg!(error);
    assert!(error < 0.1)
}
