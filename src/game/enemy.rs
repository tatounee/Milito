
use std::cmp::Ordering;

use crate::log;
use super::BOARD_LENGHT;
use super::projectile::Projectile;
use super::components::{Collide, RangeBox};
use crate::FPS;

#[derive(Debug, Clone)]
pub struct Enemy {
    life: u32,
    max_life: f32,
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
            1 => Some(Self::new(80, 35, 1, 10, -4., RangeBox::new(3, 7), 0.7)),
            2 => Some(Self::new(200, 40, 2, 20, -2.5, RangeBox::new(4, 7), 1.)),
            3 => Some(Self::new(30, 20, 3, 15, -7., RangeBox::new(2, 8), 0.4)),
            4 => Some(Self::new(2000, 600, 4, 50, -1.8, RangeBox::new(1, 9), 1.3)),
            _ => None
        }
    }

    #[inline]
    pub fn new(life: u32, damage: u32, level: u8, reward: u32, speed: f32, hitbox: RangeBox, attack_waiting: f32) -> Self {
        Self {
            life,
            max_life: life as f32,
            x: BOARD_LENGHT as f32,
            damage,
            level,
            reward,
            speed: speed / FPS as f32,
            hitbox,
            waiting: 0.,
            attack_waiting: attack_waiting * FPS as f32
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
    pub fn scale(&self) -> f32 {
        let x = self.life as f32 / self.max_life;
        1. / -10f32.powf(1.7 * (x + 0.1)) + 1.
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
