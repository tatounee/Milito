use std::rc::Rc;

use crate::FPS;

use super::{
    components::{Collide, RangeBox},
    enemy::Enemy,
    projectile::Projectile,
};

#[derive(Debug, Clone)]
pub struct Turret {
    x: f32,
    price: u32,
    price_text: Rc<String>,
    level: u8,
    shoot: bool,
    life: u32,
    hitbox: RangeBox,
    waiting: f32,
    attack_waiting: f32,
}

impl Turret {
    pub fn prefab_turret(level: u8) -> Option<Self> {
        match level {
            1 => Some(Self {
                x: 0.,
                price: 100,
                price_text: Rc::new("100".to_owned()),
                level,
                shoot: true,
                life: 120,
                hitbox: RangeBox::new(1, 7),
                waiting: 0.,
                attack_waiting: 1.8 * FPS as f32, // 30 dmg
            }),
            2 => Some(Self {
                x: 0.,
                price: 300,
                price_text: Rc::new("300".to_owned()),
                level,
                shoot: true,
                life: 120,
                hitbox: RangeBox::new(1, 7),
                waiting: 0.,
                attack_waiting: 2.5 * FPS as f32, // 90 dmg
            }),
            3 => Some(Self {
                x: 0.,
                price: 200,
                price_text: Rc::new("200".to_owned()),
                level,
                shoot: false,
                life: 10000,
                hitbox: RangeBox::new(2, 8),
                waiting: 0.,
                attack_waiting: 0.,
            }),
            _ => None
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    pub fn set_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    #[inline]
    pub fn price(&self) -> u32 {
        self.price
    }

    #[inline]
    pub fn level(&self) -> u8 {
        self.level
    }

    #[inline]
    pub fn price_text(&self) -> Rc<String> {
        self.price_text.clone()
    }

    #[inline]
    pub fn refund(&self) -> u32 {
        self.price >> 1
    }

    #[inline]
    pub fn wait(&mut self) {
        if self.waiting < self.attack_waiting {
            self.waiting += 1.;
        }
    }

    #[inline]
    pub fn can_attack(&self) -> bool {
        self.waiting >= self.attack_waiting && self.shoot
    }

    #[inline]
    pub fn get_hitbox(&self) -> RangeBox {
        self.hitbox.clone() + self.x as i32
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.life == 0
    }

    #[inline]
    pub fn take_damage(&mut self, damage: u32) {
        self.life = self.life.saturating_sub(damage)
    }

    pub fn shoot(&mut self) -> Option<Projectile> {
        if self.can_attack() {
            self.waiting = 0.;
            Projectile::new_turret_projectile(self.level, self.x)
        } else {
            None
        }
    }
}

impl Collide<&Enemy> for &Turret {
    #[inline]
    fn collide(&self, with: &Enemy) -> bool {
        self.get_hitbox().collide(with.get_hitbox())
    }
}

impl PartialEq for Turret {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}
