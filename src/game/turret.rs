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
    pub fn new(
        x: f32,
        price: u32,
        level: u8,
        shoot: bool,
        life: u32,
        hitbox: RangeBox,
        attack_waiting: f32,
    ) -> Self {
        Self {
            x,
            price,
            price_text: Rc::new(format!("{}", price)),
            level,
            shoot,
            life,
            hitbox,
            waiting: 0.,
            attack_waiting,
        }
    }

    pub fn prefab_turret(level: u8) -> Option<Self> {
        match level {
            1 => Some(Self::new(
                0.,
                100,
                level,
                true,
                120,
                RangeBox::new(-2., 2.),
                1.8 * FPS as f32, // 20 dmg, DSP: 11.1
            )),
            2 => Some(Self::new(
                0.,
                300,
                level,
                true,
                120,
                RangeBox::new(-2., 2.),
                2.5 * FPS as f32, // 90 dmg, DPS: 36
            )),
            3 => Some(Self::new(
                0.,
                200,
                level,
                false,
                5000,
                RangeBox::new(-2., 2.),
                0.,
            )),
            _ => None,
        }
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
    pub fn hitbox(&self) -> RangeBox {
        self.hitbox + self.x
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
            Projectile::new_turret_projectile(self.level, self.hitbox().start())
        } else {
            None
        }
    }
}

impl Collide<&Enemy> for &Turret {
    #[inline]
    fn collide(&self, with: &Enemy) -> bool {
        self.hitbox().collide(&with.hitbox())
    }
}

impl PartialEq for Turret {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}
