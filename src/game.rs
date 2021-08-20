#![allow(unused_imports)]

pub mod components;
pub mod enemy;
pub mod line;
pub mod player;
pub mod projectile;
pub mod turret;
pub mod wave;

use std::{cell::RefCell, collections::VecDeque, rc::Rc, vec};

use js_sys::Math::{log, random as js_random};

use line::Line;
use player::Player;
use turret::Turret;

use self::wave::{Wave, WaveLine};
use crate::{log, utils::rng, FPS};

pub type Reward = u32;
pub type Defeat = bool;

pub const NBR_OF_LINE: usize = 5;
pub const NBR_OF_COLUMN: usize = 7;
pub const BOARD_LENGHT: f32 = 110.;
const CELL_SIZE: f32 = 12.5;

pub const GOD_RECHAGE_TIME: u32 = 20 * FPS as u32;
pub const GOD_LEVEL_MAX: u32 = 7;
pub const GOD_CHARGED: u32 = GOD_RECHAGE_TIME * (GOD_LEVEL_MAX - 1);

fn get_rng_lines(lenght: usize, amount: usize) -> Vec<usize> {
    if lenght == 0 || amount == 0 {
        return Vec::new();
    }

    let mut vec = (0..lenght).collect::<Vec<usize>>();

    for i in vec.clone() {
        let goto = (rng() * (lenght - 1) as f64) as usize;
        vec.swap(i, goto);
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
    wave_counter: usize,
    pub god: u32,
    pub defeat: bool,
    turret_list: Rc<Vec<Rc<Turret>>>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            lines: vec![Line::default(); NBR_OF_LINE],
            money: if cfg!(debug_assertions) { 99999 } else { 0 },
            player: Player::default(),
            action: None,
            god: 1,
            waves: VecDeque::new(),
            wave_counter: 0,
            defeat: false,
            turret_list: Rc::new(vec![
                Rc::new(Turret::prefab_turret(1).unwrap()),
                Rc::new(Turret::prefab_turret(2).unwrap()),
                Rc::new(Turret::prefab_turret(3).unwrap()),
            ]),
        }
    }
}

impl Game {
    pub(crate) fn skip_one_wave(&mut self) {
        if self.is_wave_ended() {
            self.wave_counter += 1;
            if self.wave_counter == 10 {
                self.unlock_new_turret()
            }

            self.money += self
                .lines
                .iter_mut()
                .map(|line| line.skip_one_wave())
                .sum::<u32>();
        }
    }

    #[inline]
    pub fn turret_list(&self) -> Rc<Vec<Rc<Turret>>> {
        self.turret_list.clone()
    }

    #[inline]
    pub fn move_player_up(&mut self) {
        self.player.up()
    }

    #[inline]
    pub fn move_player_down(&mut self) {
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
    pub fn generate_wave(&mut self) {
        self.add_wave(Wave::generate(self.waves.len() as u32 + 1))
    }

    #[inline]
    pub fn generate_waves(&mut self, amount: u32) {
        for _ in 0..amount {
            self.generate_wave()
        }
    }

    #[inline]
    pub fn is_delete_mode(&self) -> bool {
        matches!(self.action, Some(ActionOnBoard::Delete))
    }

    #[inline]
    pub fn wave(&self) -> usize {
        self.lines[0].wave()
    }

    // Return true if there is not more wave
    #[inline]
    pub fn start_next_wave(&mut self) -> bool {
        if self.is_wave_ended() {
            self.wave_counter += 1;
            log!("counter:", self.wave_counter);
            if self.wave_counter == 10 {
                self.unlock_new_turret()
            }
        }

        self.lines
            .iter_mut()
            .map(|line| line.start_next_wave())
            .collect::<Vec<_>>()
            .iter()
            .all(|opt| opt.is_none())
    }

    #[inline]
    fn unlock_new_turret(&mut self) {
        self.turret_list = Rc::new(vec![
            Rc::new(Turret::prefab_turret(1).unwrap()),
            Rc::new(Turret::prefab_turret(2).unwrap()),
            Rc::new(Turret::prefab_turret(3).unwrap()),
            Rc::new(Turret::prefab_turret(4).unwrap()),
            Rc::new(Turret::prefab_turret(5).unwrap()),
            Rc::new(Turret::prefab_turret(6).unwrap()),
        ])
    }

    #[inline]
    fn is_wave_running(&self) -> bool {
        self.lines.iter().any(|l| !l.is_wave_ended())
    }

    #[inline]
    pub fn is_wave_ended(&self) -> bool {
        self.lines.iter().all(|line| line.is_wave_ended())
    }

    #[inline]
    pub fn is_remaining_enemies(&self) -> bool {
        self.lines.iter().any(|line| line.is_remaining_enemies())
    }

    pub fn assign_line_for_enemies(&mut self) {
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
            .for_each(|(line, wave)| line.set_waves(RefCell::new(wave)));
    }

    pub fn can_execut_action(&self, action: &ActionOnBoard) -> bool {
        match action {
            ActionOnBoard::PlaceTurret(ref turret) => self.money >= turret.price(),
            ActionOnBoard::Delete => true,
        }
    }

    pub fn execute_action(&mut self, x: usize, y: usize) -> bool {
        if let Some(action) = self.action.take() {
            if check_x(x) && check_y(y) {
                match action {
                    ActionOnBoard::PlaceTurret(turret) => {
                        if self.money >= turret.price() {
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
        false
    }

    pub fn upgrade_player(&mut self) {
        let upgrade_cost = self.player.upgrade_cost();
        if self.money >= upgrade_cost && self.player.upgrade() {
            self.money -= upgrade_cost;
        }
    }

    pub fn player_shoot(&mut self) {
        if self.player.can_attack() {
            self.lines[self.player.line].spawn_projectile(self.player.shoot().unwrap());
        }
    }

    pub fn process(&mut self) {
        if self.god < GOD_CHARGED && self.is_wave_running() {
            self.god += 1;
        }
        // PLAYER WAIT
        self.player.wait();
        let result = self
            .lines
            .iter_mut()
            .map(|line| line.process())
            .collect::<Vec<(u32, bool)>>();

        let reward = result.iter().map(|r| r.0).sum::<u32>();
        self.money += reward;

        self.defeat = result.iter().any(|r| r.1);
    }

    pub fn use_god(&mut self) -> bool {
        if self.god == GOD_CHARGED {
            let reward = self
                .lines
                .iter_mut()
                .map(|line| line.use_god())
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
            Self::Delete => None,
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
