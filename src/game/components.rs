
use std::borrow::Cow;

pub trait Move {
    fn deplace(&mut self);
}

pub trait CoordX {
    fn x(&self) -> u32;
}

pub trait Damage {
    fn damage(&self) -> u32;
}

pub trait Life {
    fn get_life(&self) -> &u32;
    fn get_mut_life(&mut self) -> &mut u32;
    fn take_damage(&mut self, sub: u32);
    fn is_dead(&self) -> bool;
}

pub trait Price {
    fn refund(&self) -> u32;
    fn price(&self) -> u32;
}

pub trait Shoot<P> {
    fn can_attack(&self) -> bool;
    fn wait(&mut self);
    fn shoot(&mut self) -> Option<P>;
}

pub trait Reward {
    fn reward(&self) -> u32;
}

pub trait Level {
    fn level(&self) -> u8;
}

pub trait Hitbox {
    fn hitbox(&self) -> Cow<RangeBox>;
}

#[derive(Debug, Clone)]
pub struct RangeBox {
    start: u32,
    end: u32,
}

impl RangeBox {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, number: u32) -> bool {
        number >= self.start || number < self.end
    }

    pub fn collide(&self, other: Cow<RangeBox>) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }
}
