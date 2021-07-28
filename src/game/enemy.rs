
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::log;
use super::BOARD_LENGHT;
use super::projectile::Projectile;
use super::components::{Collide, RangeBox};
use crate::FPS;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Enemy {
    life: u32,
    x: f32,
    damage: u32,
    level: u8,
    reward: u32,
    speed: f32,
    hitbox: RangeBox,
    waiting: f32,
    attack_waiting: f32,
}

impl Enemy {
    pub fn prefab(level: u8) -> Option<Self> {
        match level {
            1 => Some(Self {
                life: 80,
                x: BOARD_LENGHT as f32,
                damage: 35,
                level,
                reward: 10,
                speed: -2. / FPS as f32,
                hitbox: RangeBox::new(3, 8),
                waiting: 0.,
                attack_waiting: 1.2 * FPS as f32
            }),
            2 => Some(Self{
                life: 200,
                x: BOARD_LENGHT as f32,
                damage: 40,
                level,
                reward: 20,
                speed: -1.2 / FPS as f32,
                hitbox: RangeBox::new(3, 7),
                waiting: 0.,
                attack_waiting: 1.8 * FPS as f32
            }),
            3 => Some(Self {
                life: 30,
                x: BOARD_LENGHT as f32,
                damage: 20,
                level,
                reward: 15,
                speed: -3.5 / FPS as f32,
                hitbox: RangeBox::new(-1, 10),
                waiting: 0.,
                attack_waiting: 0.8 * FPS as f32,
            }),
            4 => Some(Self {
                life: 500,
                x: BOARD_LENGHT as f32,
                damage: 100,
                level,
                reward: 50,
                speed: -0.8 / FPS as f32,
                hitbox: RangeBox::new(-3, 15),
                waiting: 0.,
                attack_waiting: 2. * FPS as f32,
            }),
            _ => None
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    pub fn level(&self) -> u8 {
        self.level
    }

    #[inline]
    pub fn reward(&self) -> u32 {
        self.reward
    }

    #[inline]
    pub fn wait(&mut self) {
        if self.waiting < self.attack_waiting {
            self.waiting += 1.;
        }
    }

    #[inline]
    pub fn can_attack(&self) -> bool {
        self.waiting == self.attack_waiting
    }

    #[inline]
    pub fn attack(&mut self) -> u32 {
        self.waiting = 0.;
        self.damage
    }

    #[inline]
    pub fn get_hitbox(&self) -> RangeBox {
        self.hitbox.clone() + self.x as i32
    }

    #[inline]
    pub fn get_life(&self) -> u32 {
        self.life
    }

    #[inline]
    pub fn take_damage(&mut self, damage: u32) {
        self.life = self.life.saturating_sub(damage)
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.life == 0
    }

    #[inline]
    pub fn deplace(&mut self) {
        self.x += self.speed;
    }
}

impl PartialEq for Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Eq for Enemy {}

impl PartialOrd for Enemy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for Enemy {
    fn cmp(&self, other: &Self) -> Ordering {
        // We can have a NaN number so it's safe to unwrap
        self.x.partial_cmp(&other.x).unwrap()
    }
}

impl Collide<&Projectile> for &Enemy {
    #[inline]
    fn collide(&self, with: &Projectile) -> bool {
        self.get_hitbox().collide(with.get_hitbox())
    }
}
