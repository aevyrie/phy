use bevy::prelude::*;

use crate::{constraints::Constraint, particles::Particle};

#[derive(Resource, Default)]
pub struct Particles {
    pub inner: Vec<Particle>,
}

#[derive(Resource, Default)]
pub struct Constraints {
    pub inner: Vec<Box<dyn Constraint>>,
}
