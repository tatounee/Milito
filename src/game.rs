
pub mod components;
pub mod line;
pub mod enemy;
pub mod player;
pub mod projectile;
pub mod turret;

use components::Shoot;
use line::Line;
use player::Player;
use turret::Turret;

pub const NBR_OF_LINE: usize = 5;
pub const NBR_OF_COLUMN: usize = 7;
pub const BOARD_LENGHT: usize = 120;

pub const GOD_RECHAGE_TIME: u32 = 200;
pub const GOD_LEVEL_MAX: u32 = 7;
pub const GOD_CHARGED: u32 = GOD_RECHAGE_TIME * GOD_LEVEL_MAX;

#[derive(Debug)]
pub struct Game {
    pub lines: Vec<Line>,
    pub money: u32,
    pub player: Player,
    pub action: Option<ActionOnBoard>,
    pub god: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            lines: vec![Line::default(); NBR_OF_LINE],
            money: 0,
            player: Player::default(),
            action: None,
            god: 0,
        }
    }
}

impl Game {
    #[inline]
    pub fn move_player_up(&mut self) {
        self.player.up()
    }

    #[inline]
    pub fn move_player_down(&mut self) {
        self.player.down()
    }

    pub fn execute_action(&mut self, x: usize, y: usize) {
        if let Some(action) = self.action.take() {
            if check_x(x) && check_y(y) {
                match action {
                    ActionOnBoard::PlaceTurret(turret) => {
                        if self.money > turret.price() {
                            self.money -= self.lines[y].add_turret(x, turret);
                        }
                    },
                    ActionOnBoard::Delete => {
                        self.money += self.lines[y].delete_turret(x);
                    }
                }
            }
        }
    }

    pub fn player_shoot(&mut self) {
        if self.player.can_attack() {
            self.lines[self.player.line].spawn_projectile(self.player.shoot().unwrap())
        }
    }

    pub fn process(&mut self) {
        if self.god < GOD_CHARGED - 1 {
            self.god += 1;
        }
        // PLAYER WAIT
        self.player.wait();
        self.lines.iter_mut().for_each(|line| line.process());
    }

    pub fn kill_all(&mut self) -> bool {
        if self.god == GOD_CHARGED - 1 {
            let reward = self.lines.iter_mut().map(|line| {
                line.kill_all()
            }).sum::<u32>();
            self.money += reward;
            self.god = 0;
            reward != 0
        } else {
            false
        }
    }

    #[inline]
    pub fn god_level(&self) -> u32 {
        (self.god / GOD_RECHAGE_TIME) + 1
    }
}

#[derive(Debug)]
pub enum ActionOnBoard {
    PlaceTurret(Box<dyn Turret>),
    Delete
}

#[inline]
fn check_x(x: usize) -> bool {
    x < NBR_OF_COLUMN
}

#[inline]
fn check_y(y: usize) -> bool {
    y < NBR_OF_LINE
}
