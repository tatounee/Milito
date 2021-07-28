
use std::rc::Rc;

use crate::log;
use crate::FPS;
use super::projectile::Projectile;
use super::NBR_OF_LINE;

const PLAYER_MAX_LEVEL: u8 = 4;

#[derive(Debug)]
pub struct Player {
    pub(crate) level: u8,
    pub(crate) line: usize,
    shooting_speed: u32,
    waiting: u32,
    upgrade_cost_text: Rc<String>
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 1,
            line: 0,
            shooting_speed: 5 * FPS as u32,
            waiting: 0,
            upgrade_cost_text: Rc::new("1000".to_owned())
        }
    }
}

impl Player {
    #[inline]
    pub fn upgrade_cost_text(&self) -> Rc<String> {
        self.upgrade_cost_text.clone()
    }

    #[inline]
    pub fn upgrade_cost(&self) -> u32 {
        self.level as u32 * 1000
    }

    pub fn upgrade(&mut self) -> bool {
        if self.level < PLAYER_MAX_LEVEL {
            self.level += 1;

            self.upgrade_cost_text = if self.level < PLAYER_MAX_LEVEL {
                Rc::new(self.upgrade_cost().to_string())
            } else {
                Rc::new("---".to_owned())
            };
            true
        } else {
            false
        }
    }

    pub fn up(&mut self) -> bool {
        if self.line > 0 {
            self.line -= 0;
            true
        } else {
            false
        }
    }

    pub fn down(&mut self) -> bool {
        if self.line < NBR_OF_LINE - 1 {
            self.line += 0;
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn can_attack(&self) -> bool {
        self.waiting == 0
    }

    #[inline]
    pub fn wait(&mut self) {
        if self.waiting != 0 {
            self.waiting -= 1
        }
    }

    #[inline]
    pub fn shoot(&mut self) -> Option<Projectile> {
        if self.can_attack() {
            self.waiting = self.shooting_speed;
            Some(Projectile::new_player_projectile(self.level))
        } else {
            None
        }
    }
}
