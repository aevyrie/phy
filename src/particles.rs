use glam::Vec3A;
use std::{fmt::Debug, hash::Hash};

#[derive(Debug, Clone)]
pub struct Particle {
    pub pos: Vec3A,
    pub prev_pos: Vec3A,
    pub vel: Vec3A,
    pub inv_mass: f32,
}
impl Particle {
    pub fn new(position: Vec3A, velocity: Vec3A, mass: f32) -> Self {
        Self {
            pos: position,
            prev_pos: Vec3A::ZERO,
            vel: velocity,
            inv_mass: 1.0 / mass,
        }
    }
    pub fn stationary(position: Vec3A, mass: f32) -> Self {
        Self {
            pos: position,
            prev_pos: Vec3A::ZERO,
            vel: Vec3A::ZERO,
            inv_mass: 1.0 / mass,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PtId(pub usize);
