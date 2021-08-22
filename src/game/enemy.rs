use std::cmp::Ordering;

use super::components::{Collide, RangeBox};
use super::projectile::Projectile;
use super::BOARD_LENGHT;
use crate::log;
use crate::utils::{rng, GetRandom};
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
            1 => Some(Self::new(80, 35, 1, 10, -4., RangeBox::new(4., 6.), 0.7)), // DPS: 50
            2 => Some(Self::new(250, 40, 2, 20, -3., RangeBox::new(4., 7.), 1.)), // DPS: 40
            3 => Some(Self::new(30, 25, 3, 15, -10., RangeBox::new(2., 7.), 0.4)), // DPS: 62.5
            4 => Some(Self::new(
                2000,
                600,
                4,
                50,
                -1.8,
                RangeBox::new(2., 7.),
                1.3,
            )), // DPS: 461.5

            5 => Some(Self::new(200, 40, 5, 10, -4.5, RangeBox::new(4., 6.), 0.7)), // DPS: 57.1
            6 => Some(Self::new(600, 45, 6, 20, -3.8, RangeBox::new(4., 7.), 1.)),  // DPS: 45
            7 => Some(Self::new(70, 30, 7, 15, -14., RangeBox::new(2., 7.), 0.4)),  // DPS: 75
            8 => Some(Self::new(
                3500,
                650,
                8,
                60,
                -2.3,
                RangeBox::new(2., 7.),
                1.3,
            )), // DPS: 500
            _ => None,
        }
    }

    #[inline]
    pub fn new(
        life: u32,
        damage: u32,
        level: u8,
        reward: u32,
        speed: f32,
        hitbox: RangeBox,
        attack_waiting: f32,
    ) -> Self {
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
            attack_waiting: attack_waiting * FPS as f32,
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    pub fn speed(&self) -> f32 {
        self.speed
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
        1. / -(10f32.powf(1.7 * (x + 0.1))) + 1.
    }

    #[inline]
    pub fn wait(&mut self) {
        if self.waiting < self.attack_waiting {
            self.waiting += 1.;
        }
    }

    #[inline]
    pub fn can_attack(&self) -> bool {
        (self.waiting - self.attack_waiting).abs() < f32::EPSILON
    }

    #[inline]
    pub fn attack(&mut self) -> u32 {
        self.waiting = 0.;
        self.damage
    }

    #[inline]
    pub fn hitbox(&self) -> RangeBox {
        self.hitbox + self.x
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

    #[inline]
    pub fn max_life(&self) -> f32 {
        self.max_life
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
        self.x.partial_cmp(&other.x).unwrap()
    }
}

impl Collide<&Projectile> for &Enemy {
    #[inline]
    fn collide(&self, with: &Projectile) -> bool {
        self.hitbox().collide(&with.hitbox())
    }
}

#[derive(Debug, Clone)]
pub struct EnemyProceced {
    pub level: u8,
    pub weight: u32,
}

impl EnemyProceced {
    pub fn new_random(proba_rank2_enemy: f64) -> Self {
        if rng() > proba_rank2_enemy {
            RANK1.get_random().unwrap().clone()
        } else {
            RANK2.get_random().unwrap().clone()
        }
    }
}

const E1: EnemyProceced = EnemyProceced {
    level: 1,
    weight: 1,
};
const E2: EnemyProceced = EnemyProceced {
    level: 2,
    weight: 5,
};
const E3: EnemyProceced = EnemyProceced {
    level: 3,
    weight: 4,
};
const E4: EnemyProceced = EnemyProceced {
    level: 4,
    weight: 20,
};
const E5: EnemyProceced = EnemyProceced {
    level: 5,
    weight: 4,
};
const E6: EnemyProceced = EnemyProceced {
    level: 6,
    weight: 9,
};
const E7: EnemyProceced = EnemyProceced {
    level: 7,
    weight: 5,
};
const E8: EnemyProceced = EnemyProceced {
    level: 8,
    weight: 55,
};

pub const RANK1: [EnemyProceced; 4] = [E1, E2, E3, E4];
pub const RANK2: [EnemyProceced; 4] = [E5, E6, E7, E8];
