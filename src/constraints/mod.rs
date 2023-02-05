use crate::particles::Particle;
mod implementations;

pub use implementations::*;

pub trait Constraint: ConstraintClone + Send + Sync {
    fn solve(&self, particles: &mut [Particle], delta_t_s: f32);
}

/// This wrapper trait allows us to clone boxed constraints, without preventing the creation of
/// Constraint trait objects. Simply making Constraint a super trait of Clone, would prevent us from
/// using constraints as trait objects, because Clone requires it to be Sized.
pub trait ConstraintClone {
    fn clone_box(&self) -> Box<dyn Constraint>;
}

impl<T> ConstraintClone for T
where
    T: 'static + Constraint + Clone,
{
    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Constraint> {
    fn clone(&self) -> Box<dyn Constraint> {
        self.clone_box()
    }
}

impl<T: 'static> From<T> for Box<dyn Constraint>
where
    T: Constraint,
{
    fn from(constraint: T) -> Self {
        Box::new(constraint)
    }
}

#[test]
fn hanging_mass() {
    use crate::{constraints::Fixed, particles::Particle};
    use glam::Vec3A;
    use std::time::Duration;
    let mut particles = vec![Particle::stationary(Vec3A::ZERO, 1.0)];
    let mut constraints = vec![Fixed::new(Vec3A::ZERO, 0, 0.001).into()];

    crate::simulate_iteration(
        Duration::from_secs_f32(2.0),
        1000,
        &mut particles,
        &mut constraints,
    );

    let p_final = &particles[0];
    let error = (p_final.pos.y).abs();
    dbg!(p_final);
    assert!(error < 0.0001)
}
