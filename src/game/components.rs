use std::ops::Add;

#[derive(Debug, Clone)]
pub struct RangeBox {
    start: f32,
    end: f32,
}

impl Add<f32> for RangeBox {
    type Output = Self;

    #[inline]
    fn add(self, shift: f32) -> Self::Output {
        Self::new(self.start + shift, self.end + shift)
    }
}

impl RangeBox {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn contains(&self, number: f32) -> bool {
        self.start <= number && number < self.end
    }
}

impl Collide for RangeBox {
    #[inline]
    fn collide(&self, with: Self) -> bool {
        self.contains(with.start) || self.contains(with.end)
    }
}

pub trait Collide<With = Self> {
    fn collide(&self, with: With) -> bool;
}
