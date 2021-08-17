use std::cmp::Ordering;

use crate::FPS;

use super::components::RangeBox;

#[derive(Debug, Clone)]
pub struct Projectile {
    x: f32,
    damage: u32,
    level: u8,
    speed: f32,
    hitbox: RangeBox,
    from_player: bool,
    next_impact: Option<usize>,
}

impl Projectile {
    pub fn new(
        x: f32,
        damage: u32,
        level: u8,
        speed: f32,
        hitbox: RangeBox,
        from_player: bool,
    ) -> Self {
        Self {
            x,
            damage,
            level,
            speed,
            hitbox,
            from_player,
            next_impact: None,
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
    pub fn damage(&self) -> u32 {
        self.damage
    }

    #[inline]
    pub fn level(&self) -> u8 {
        self.level
    }

    #[inline]
    pub fn from_player(&self) -> bool {
        self.from_player
    }

    #[inline]
    pub fn take_next_impact(&mut self) -> Option<usize> {
        self.next_impact.take()
    }

    #[inline]
    pub fn add_next_impact(&mut self, id: usize) {
        self.next_impact = Some(id)
    }

    pub fn new_player_projectile(level: u8) -> Self {
        Self::new(
            4.,
            (level as u32).pow(2) * 10 + 10,
            level,
            (level as f32 / 2. + 0.8) * 30. / FPS as f32,
            RangeBox::new(-1., 2.),
            true,
        )
    }

    pub fn new_turret_projectile(level: u8, x: f32) -> Option<Self> {
        match level {
            1 => Some(Self::new(
                x - 1.,
                20,
                level,
                50. / FPS as f32,
                RangeBox::new(1., 2.),
                false,
            )),
            2 => Some(Self::new(
                x - 1.,
                90,
                level,
                35. / FPS as f32,
                RangeBox::new(1., 2.),
                false,
            )),
            _ => None,
        }
    }

    #[inline]
    pub fn deplace(&mut self) {
        self.x += self.speed;
    }

    #[inline]
    pub fn hitbox(&self) -> RangeBox {
        self.hitbox + self.x
    }
}

impl PartialEq for Projectile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Eq for Projectile {}

impl PartialOrd for Projectile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for Projectile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.partial_cmp(&other.x).unwrap()
    }
}
