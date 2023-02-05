use super::Constraint;
use crate::particles::Particle;
use glam::Vec3A;

#[derive(Clone)]
pub struct Distance {
    pub length: f32,
    pub pt_0: usize,
    pub pt_1: usize,
    pub compliance: f32,
}

impl Distance {
    pub fn new(length: f32, p0: usize, p1: usize, compliance: f32) -> Self {
        Self {
            length,
            pt_0: p0,
            pt_1: p1,
            compliance,
        }
    }
}

impl Constraint for Distance {
    fn solve(&self, particles: &mut [Particle], delta_t_s: f32) {
        // These vectors tell us the vector traveled as a result of an increase of 1.0 of
        // our error function. These will be used to reposition the particles based on the
        // current error.
        let (x_0, x_1) = (&particles[self.pt_0], &particles[self.pt_1]);
        let (inv_mass_0, inv_mass_1) = (x_0.inv_mass, x_1.inv_mass);
        let grad1 = (x_1.pos - x_0.pos).normalize();
        let grad0 = -grad1;

        // The current error of the constraint.
        let c = x_1.pos.distance(x_0.pos) - self.length;

        // We can factor out the grad multiplication from the general form of the equation
        // because we know the length of the vector is 1.
        let lambda = -c / (inv_mass_0 + inv_mass_1 + self.compliance / delta_t_s.powi(2));

        // Scale the amount of correction by the inverse of each particle's mass.
        particles[self.pt_0].pos += lambda * inv_mass_0 * grad0;
        particles[self.pt_1].pos += lambda * inv_mass_1 * grad1;
    }
}

#[derive(Clone)]
pub struct Fixed {
    pub origin: Vec3A,
    pub pt_0: usize,
    pub compliance: f32,
}

impl Fixed {
    pub fn new(origin: Vec3A, p1: usize, compliance: f32) -> Self {
        Self {
            origin,
            pt_0: p1,
            compliance,
        }
    }
}
impl Constraint for Fixed {
    fn solve(&self, particles: &mut [Particle], delta_t_s: f32) {
        let x_0 = &particles[self.pt_0];
        let inv_mass_0 = x_0.inv_mass;
        let grad_0 = (x_0.pos - self.origin).normalize();
        let c = self.origin.distance(x_0.pos);
        // We can factor out the grad multiplication here - we know the len is 1
        let lambda = -c / (inv_mass_0 + self.compliance / delta_t_s.powi(2));
        particles[self.pt_0].pos += lambda * inv_mass_0 * grad_0;
    }
}
