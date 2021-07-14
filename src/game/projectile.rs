
use std::cmp::Ordering;
use std::fmt::Debug;

use super::components::{CoordX, Damage, Hitbox, Move};

pub trait Projectile: Move + CoordX + Damage + Debug + ProjectileClone + Send + Sync + Hitbox {}

impl PartialEq for dyn Projectile {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()
    }
}

impl Eq for dyn Projectile {}

impl PartialOrd for dyn Projectile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x().partial_cmp(&other.x())
    }    
}

impl Ord for dyn Projectile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x().cmp(&other.x())
    }
}

pub trait ProjectileClone {
    fn clone_box(&self) -> Box<dyn Projectile>;
}

impl<T> ProjectileClone for T
where T: 'static + Projectile + Clone {
    fn clone_box(&self) -> Box<dyn Projectile> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Projectile> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
