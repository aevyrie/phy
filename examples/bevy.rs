use std::time::Duration;

use bevy::prelude::*;
use glam::Vec3A;
use phy::{
    bevy::{Constraints, Particles},
    constraints::{Distance, Fixed},
    particles::Particle,
    simulate_iteration,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Particles>()
        .init_resource::<Constraints>()
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PostUpdate, physics_update)
        .add_system(velocity_colors)
        .run();
}

#[derive(Component)]
struct Id(usize);

fn setup(
    mut commands: Commands,
    mut particles: ResMut<Particles>,
    mut constraints: ResMut<Constraints>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Build physical model
    let chain_count = 50;
    let diameter = 0.005;
    let mass = 0.005;
    let chain_length = chain_count.max(15) as f32 * diameter;
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: diameter / 2.0,
        subdivisions: 5,
    }));
    particles.inner = vec![Particle::stationary(Vec3A::ZERO, 1.0)];
    constraints.inner = vec![Fixed::new(Vec3A::ZERO, 0, 0.0).into()];
    for i in 1..chain_count {
        particles.inner.push(Particle::stationary(
            [diameter * i as f32, 0.0, 0.0].into(),
            mass,
        ));
        constraints
            .inner
            .push(Distance::new(diameter, i - 1, i, 0.0).into());
    }

    // Add meshes and ID marker components for all particles
    for i in 0..particles.inner.len() {
        commands
            .spawn(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
                ..Default::default()
            })
            .insert(Id(i));
    }

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, -chain_length / 2.0, 1.5 * chain_length),
        ..default()
    });
}

const MAX_FRAMETIME: Duration = Duration::from_millis(250);
const PHY_DT: Duration = Duration::from_millis(10);
const SUBSTEPS: usize = 20;

fn physics_update(
    mut particles: ResMut<Particles>,
    constraints: Res<Constraints>,
    mut spheres: Query<(&Id, &mut Transform)>,
    time: Res<Time>,
    mut accumulator: Local<Duration>,
) {
    let frametime = time.delta().min(MAX_FRAMETIME);
    *accumulator = *accumulator + frametime;

    while *accumulator >= PHY_DT {
        simulate_iteration(PHY_DT, SUBSTEPS, &mut particles.inner, &constraints.inner);
        *accumulator = *accumulator - PHY_DT;
    }

    // Update bevy object positions
    for (particle, mut transform) in spheres.iter_mut() {
        transform.translation = particles.inner[particle.0].pos.into()
    }
}

fn velocity_colors(
    physics_particles: Res<Particles>,
    particles: Query<(&Handle<StandardMaterial>, &Id)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (handle, id) in particles.iter() {
        let velocity = physics_particles.inner[id.0].vel.length();
        materials
            .get_mut(handle)
            .unwrap()
            .base_color
            .set_g(velocity);
    }
}
