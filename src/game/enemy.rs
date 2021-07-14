
use std::cmp::Ordering;
use std::fmt::Debug;

use super::projectile::Projectile;
use super::components::{CoordX, Damage, Hitbox, Life, Move, Shoot};

pub trait Enemy: Move + CoordX + Damage + Life + Debug + EnemyClone + Send + Sync + Shoot<Box<dyn Projectile>> + Hitbox {}

impl PartialEq for dyn Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()
    }
}

impl Eq for dyn Enemy {}

impl PartialOrd for dyn Enemy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x().partial_cmp(&other.x())
    }    
}

impl Ord for dyn Enemy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x().cmp(&other.x())
    }
}
pub trait EnemyClone {
    fn clone_box(&self) -> Box<dyn Enemy>;
}

impl<T> EnemyClone for T
where T: 'static + Enemy + Clone {
    fn clone_box(&self) -> Box<dyn Enemy> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Enemy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}