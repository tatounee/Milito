
#![allow(unused_imports)]

pub mod components;
pub mod enemy;
pub mod line;
pub mod player;
pub mod projectile;
pub mod turret;
pub mod wave;

use std::{cell::RefCell, collections::VecDeque, rc::Rc, vec};

use js_sys::Math::random as js_random;

use line::Line;
use player::Player;
use turret::Turret;

use self::wave::{Wave, WaveLine};
use crate::{FPS, log};

pub const NBR_OF_LINE: usize = 5;
pub const NBR_OF_COLUMN: usize = 7;
pub const BOARD_LENGHT: f32 = 110.;
 
pub const GOD_RECHAGE_TIME: u32 = 20 * FPS as u32;
pub const GOD_LEVEL_MAX: u32 = 7;
pub const GOD_CHARGED: u32 = GOD_RECHAGE_TIME * (GOD_LEVEL_MAX - 1);

#[allow(unused_unsafe)]
fn get_rng_lines(lenght: usize, amount: usize) -> Vec<usize> {
    if lenght == 0 || amount == 0 {
        return Vec::new();
    }

    let mut vec = (0..lenght).collect::<Vec<usize>>();

    for i in vec.clone().iter() {
        let goto = (unsafe { js_random() } * (lenght - 1) as f64) as usize;
        vec.swap(*i, goto);
    }

    let drained = vec.drain(0..amount).collect::<Vec<usize>>();
    drained
}

#[derive(Debug)]
pub struct Game {
    pub lines: Vec<Line>,
    pub money: u32,
    pub player: Player,
    pub action: Option<ActionOnBoard>,
    pub waves: VecDeque<Wave>,
    pub god: u32,
    turret_list: Rc<Vec<Rc<Turret>>>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            lines: vec![Line::default(); NBR_OF_LINE],
            money: 9999999,
            player: Player::default(),
            action: None,
            god: 1,
            waves: VecDeque::new(),
            turret_list: Rc::new(vec![
                Rc::new(Turret::prefab_turret(1).unwrap()),
                Rc::new(Turret::prefab_turret(2).unwrap()),
                Rc::new(Turret::prefab_turret(3).unwrap()),
            ]),
        }
    }
}

impl Game {
    #[inline]
    pub fn turret_list(&self) -> Rc<Vec<Rc<Turret>>> {
        self.turret_list.clone()
    }

    #[inline]
    pub fn move_player_up(&mut self) -> bool {
        self.player.up()
    }

    #[inline]
    pub fn move_player_down(&mut self) -> bool {
        self.player.down()
    }

    #[inline]
    pub fn add_wave(&mut self, wave: Wave) {
        self.waves.push_back(wave)
    }

    #[inline]
    pub fn add_waves(&mut self, waves: Vec<Wave>) {
        for wave in waves {
            self.add_wave(wave)
        }
    }

    #[inline]
    pub fn is_delete_mode(&self) -> bool {
        matches!(self.action, Some(ActionOnBoard::Delete))
    }

    // Return true if there is not more wave
    #[inline]
    pub fn next_wave(&mut self) -> bool {
        self.lines
            .iter_mut()
            .map(|line| line.next_wave())
            .collect::<Vec<_>>()
            .iter()
            .all(|opt| opt.is_none())
    }

    #[inline]
    pub fn is_wave_ended(&self) -> bool {
        self.lines.iter().all(|line| line.is_wave_ended())
    }

    #[inline]
    pub fn remaining_enemies(&self) -> bool {
        self.lines.iter().any(|line| line.remaining_enemies())
    }

    pub fn enemy_wave_assign_line(&mut self) {
        let mut wave_packs = vec![VecDeque::new(); NBR_OF_LINE];

        for wave in self.waves.iter_mut() {
            let mut wave_lines = (0..NBR_OF_LINE)
                .map(|_| WaveLine::default())
                .collect::<Vec<WaveLine>>();
            let mut keys = wave.troops.keys().cloned().collect::<Vec<u64>>();
            keys.sort_unstable();

            for frame in keys {
                let levels = wave.troops.get_mut(&frame).unwrap();

                for line in get_rng_lines(self.lines.len(), levels.len()) {
                    let wave_line = wave_lines.get_mut(line).unwrap();
                    wave_line.add_enemy(frame, levels.pop().unwrap());
                }
            }

            for (i, wave_line) in wave_lines.into_iter().enumerate() {
                wave_packs[i].push_back(wave_line)
            }
        }

        self.lines
            .iter_mut()
            .zip(wave_packs.into_iter())
            .for_each(|(line, wave)| line.waves = RefCell::new(wave));
    }

    pub fn execute_action(&mut self, x: usize, y: usize) -> bool {
        if let Some(action) = self.action.take() {
            if check_x(x) && check_y(y) {
                match action {
                    ActionOnBoard::PlaceTurret(turret) => {
                        if self.money > turret.price() {
                            self.money -= self.lines[y].add_turret(x, turret);
                            return true;
                        }
                    }
                    ActionOnBoard::Delete => {
                        self.money += self.lines[y].delete_turret(x);
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn upgrade_player(&mut self) -> bool {
        let upgrade_cost = self.player.upgrade_cost();
        if self.money >= upgrade_cost {
            if self.player.upgrade() {
                
                self.money -= upgrade_cost;
                log!("player level", self.player.level);
                return true;
            }
        }
        false
    }

    pub fn player_shoot(&mut self) -> bool {
        if self.player.can_attack() {
            self.lines[self.player.line].spawn_projectile(self.player.shoot().unwrap());
            true
        } else {
            false
        }
    }

    pub fn process(&mut self) {
        if self.god < GOD_CHARGED {
            self.god += 1;
        }
        // PLAYER WAIT
        self.player.wait();
        self.lines.iter_mut().for_each(|line| line.process());
    }

    pub fn kill_all(&mut self) -> bool {
        if self.god == GOD_CHARGED {
            let reward = self
                .lines
                .iter_mut()
                .map(|line| line.kill_all())
                .sum::<u32>();
            self.money += reward;
            self.god = 0;
            reward != 0
        } else {
            false
        }
    }

    #[inline]
    pub fn god_level(&self) -> u32 {
        self.god / GOD_RECHAGE_TIME + 1
    }
}

#[derive(Debug, PartialEq)]
pub enum ActionOnBoard {
    PlaceTurret(Turret),
    Delete,
}

impl ActionOnBoard {
    #[inline]
    pub fn get_turret_level(&self) -> Option<u8> {
        match self {
            Self::PlaceTurret(t) => Some(t.level()),
            Self::Delete => None
        }
    }
}

#[inline]
fn check_x(x: usize) -> bool {
    x < NBR_OF_COLUMN
}

#[inline]
fn check_y(y: usize) -> bool {
    y < NBR_OF_LINE
}
