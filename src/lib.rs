use glam::Vec3;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::{simulate_iteration, Constraint, Particle, ParticleId, Particles};

    #[test]
    fn triple_pendulum() {
        let mut particles = Particles::new();
        let p0 = particles.insert(Particle::stationary(Vec3::ZERO, 1.0));
        let p1 = particles.insert(Particle::stationary([0.0, -0.5, 0.0].into(), 0.5));
        let p2 = particles.insert(Particle::stationary([0.0, -1.0, 0.0].into(), 0.5));
        let p3 = particles.insert(Particle::stationary([0.0, -1.5, 0.0].into(), 0.5));

        let constraints = vec![
            Constraint::fixed(Vec3::ZERO, p0, 0.0),
            Constraint::distance(0.5, p0, p1, 0.0),
            Constraint::distance(0.5, p1, p2, 0.0),
            Constraint::distance(0.5, p2, p3, 0.0),
        ];

        for _ in 0..2 {
            simulate_iteration(1.0 / 60.0, 40, &mut particles, &constraints);
        }

        dbg!(particles.get(&ParticleId(3)));
    }
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub pos: Vec3,
    pub prev_pos: Vec3,
    pub vel: Vec3,
    pub inv_mass: f32,
}
impl Particle {
    pub fn new(position: Vec3, velocity: Vec3, mass: f32) -> Self {
        Self {
            pos: position,
            prev_pos: Vec3::ZERO,
            vel: velocity,
            inv_mass: 1.0 / mass,
        }
    }
    pub fn stationary(position: Vec3, mass: f32) -> Self {
        Self {
            pos: position,
            prev_pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            inv_mass: 1.0 / mass,
        }
    }
}

#[derive(Debug)]
pub struct Particles {
    max_id: usize,
    data: HashMap<ParticleId, Particle>,
}
impl Particles {
    pub fn new() -> Self {
        Self {
            max_id: 0,
            data: Default::default(),
        }
    }
    pub fn insert(&mut self, particle: Particle) -> ParticleId {
        let key = ParticleId(self.max_id);
        self.data.insert(key, particle);
        self.max_id += 1;
        key
    }
    pub fn iter(&self) -> std::collections::hash_map::Values<ParticleId, Particle> {
        self.data.values()
    }
    pub fn iter_mut(&mut self) -> std::collections::hash_map::ValuesMut<ParticleId, Particle> {
        self.data.values_mut()
    }
    pub fn get(&self, id: &ParticleId) -> &Particle {
        self.data.get(id).unwrap()
    }
    pub fn get_mut(&mut self, id: &ParticleId) -> &mut Particle {
        self.data.get_mut(id).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParticleId(usize);

#[derive(Debug, Clone)]
pub enum Constraint {
    Fixed {
        origin: Vec3,
        p1: ParticleId,
        compliance: f32,
    },
    Distance {
        length: f32,
        p0: ParticleId,
        p1: ParticleId,
        compliance: f32,
    },
}

impl Constraint {
    pub fn fixed(origin: Vec3, p0: ParticleId, compliance: f32) -> Self {
        Constraint::Fixed {
            origin,
            p1: p0,
            compliance,
        }
    }
    pub fn distance(length: f32, p0: ParticleId, p1: ParticleId, compliance: f32) -> Self {
        Constraint::Distance {
            length,
            p0,
            p1,
            compliance,
        }
    }
    pub fn error(&self, particles: &Particles) -> f32 {
        match self {
            Constraint::Fixed {
                origin,
                p1: p0,
                compliance: _,
            } => {
                let p0 = particles.get(p0);
                origin.distance(p0.pos)
            }
            Constraint::Distance {
                length,
                p0,
                p1,
                compliance: _,
            } => {
                let p0 = particles.get(p0);
                let p1 = particles.get(p1);
                p1.pos.distance(p0.pos) - length
            }
        }
    }
    /// Solves the constraint and updates the particles with a position that better satisfies the
    /// constraint.
    pub fn solve(&self, particles: &mut Particles, delta_t_s: f32) {
        match self {
            Constraint::Fixed {
                origin,
                p1: id1,
                compliance,
            } => {
                let p1 = particles.get(id1);
                let grad1 = (p1.pos - *origin).normalize();
                let c = self.error(particles);
                // We can factor out the grad multiplication - we know the len is 1
                let lambda = -c / (p1.inv_mass + compliance / delta_t_s.powi(2));
                let delta_x1 = lambda * p1.inv_mass * grad1;
                let x1_new = p1.pos + delta_x1;

                particles.get_mut(id1).pos = x1_new;
            }
            Constraint::Distance {
                length: _,
                p0: id0,
                p1: id1,
                compliance,
            } => {
                let p0 = particles.get(id0);
                let p1 = particles.get(id1);

                // These vectors tell us the vector traveled as a result of an increase of 1.0 of
                // our error function. These will be used to reposition the particles based on the
                // current error.
                let grad1 = (p1.pos - p0.pos).normalize();
                let grad0 = -grad1;

                // The current error of the constraint.
                let c = self.error(particles);

                // We can factor out the grad multiplication from the general form of the equation
                // because we know the length of the vector is 1.
                let lambda = -c / (p0.inv_mass + p1.inv_mass + compliance / delta_t_s.powi(2));

                // Scale the amount of correction by the inverse of each particle's mass.
                let delta_x0 = lambda * p0.inv_mass * grad0;
                let delta_x1 = lambda * p1.inv_mass * grad1;

                let x0_new = p0.pos + delta_x0;
                let x1_new = p1.pos + delta_x1;

                particles.get_mut(id0).pos = x0_new;
                particles.get_mut(id1).pos = x1_new;
            }
        }
    }
}

pub fn simulate_iteration<'a>(
    delta_t: f32,
    substeps: usize,
    particles: &mut Particles,
    constraints: &(impl Clone + IntoIterator<Item = Constraint>),
) {
    let delta_t_s = delta_t / (substeps as f32);
    let g = Vec3::new(0.0, -9.81, 0.0);
    for substep in 0..substeps {
        for p in particles.iter_mut() {
            p.prev_pos = p.pos;
            p.vel = p.vel + delta_t_s * g;
            p.pos = p.pos + delta_t_s * p.vel;
        }
        for constraint in constraints.clone().into_iter() {
            constraint.solve(particles, delta_t_s);
        }
        for p in particles.iter_mut() {
            p.vel = (p.pos - p.prev_pos) / delta_t_s;
        }

        println!(
            "{substep:2}, y: {}, dy/dt: {}",
            particles.get(&ParticleId(3)).pos.y,
            particles.get(&ParticleId(3)).vel.y
        );
    }
}
