
use std::borrow::Cow;

use super::NBR_OF_LINE;
use super::components::{CoordX, Damage, Hitbox, Level, Move, RangeBox, Shoot};
use super::projectile::Projectile;

const SHOOTING_SPEED: u32 = 50;

#[derive(Debug)]
pub struct Player {
    pub level: u8,
    pub line: usize,
    shooting_speed: u32,
    waiting: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 1,
            line: 0,
            shooting_speed: SHOOTING_SPEED,
            waiting: 0,
        }
    }
}

impl Player {
    pub(crate) fn up(&mut self) {
        if self.line > 0 {
            self.line -= 0
        }
    }

    pub(crate) fn down(&mut self) {
        if self.line < NBR_OF_LINE - 1 {
            self.line += 0
        }
    }
}

impl Shoot<Box<PlayerBullet>> for Player {
    fn can_attack(&self) -> bool {
        self.waiting == 0
    }

    fn wait(&mut self) {
        if self.waiting != 0 {
            self.waiting -= 1
        }
    }

    fn shoot(&mut self) -> Option<Box<PlayerBullet>> {
        if self.can_attack() {
            self.waiting = self.shooting_speed;
            Some(Box::new(PlayerBullet::new(
                self.level,
                self.level as i32 * 2,
                self.level as u32 * 2,
            )))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerBullet {
    level: u8,
    x: u32,
    speed: i32,
    damage: u32,
}

impl PlayerBullet {
    pub fn new(level: u8, speed: i32, damage: u32) -> Self {
        Self {
            level,
            x: 0,
            speed,
            damage,
        }
    }
}

impl CoordX for PlayerBullet {
    #[inline]
    fn x(&self) -> u32 {
        self.x
    }
}


impl Move for PlayerBullet {
    fn deplace(&mut self) {
        if self.speed.is_positive() {
            self.x += self.speed as u32
        } else {
            self.x = self.x.saturating_sub(self.speed.abs() as u32);
        }
    }
}

impl Damage for PlayerBullet {
    fn damage(&self) -> u32 {
        self.damage
    }
}

impl Hitbox for PlayerBullet {
    fn hitbox(&self) -> Cow<RangeBox> {
        Cow::Owned(RangeBox::new(self.x - 1, self.x + 2))
    }
}

impl Level for PlayerBullet {
    fn level(&self) -> u8 {
        self.level
    }
}

impl Projectile for PlayerBullet {}
