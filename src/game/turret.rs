
use std::fmt::Debug;

use super::projectile::Projectile;
use super::components::{CoordX, Damage, Hitbox, Life, Move, Price, Shoot};

pub trait Turret: Move + CoordX + Damage + Life + Price + Debug + TurretClone + Send + Sync + Shoot<Box<dyn Projectile>> + Hitbox {}

pub trait TurretClone {
    fn clone_box(&self) -> Box<dyn Turret>;
}

impl<T> TurretClone for T
where T: 'static + Turret + Clone {
    fn clone_box(&self) -> Box<dyn Turret> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Turret> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}