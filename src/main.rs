#![allow(dead_code)]

mod components;
mod game;
mod utils;
mod input_handler;

use std::time::Duration;

use yew::{
    prelude::*,
    services::{IntervalService, Task},
};
use lazy_static::lazy_static;

use components::{Footer, FooterProps, Header, HeaderProps};
use game::{ActionOnBoard, Game, turret::Turret, wave::Wave};

use crate::components::{Board, GameRow, GameRowProps};

lazy_static! {
    static ref WAVE_1: Wave = wave![
        1 => [1, 1],
        2 => [1, 1],
        3 => [1, 1, 1],
        4 => [1],
        5 => [1, 1]
    ];

    static ref WAVE_2: Wave = wave![
        0 => [1, 1, 2],
        2 => [2],
        4 => [2, 1],
        6 => [3, 1, 1]
    ];

    static ref WAVE_3: Wave = wave![
        1 => [2, 2],
        2 => [3],
        3 => [3],
        4 => [1, 1, 1, 1],
        5 => [1, 1, 1, 1],
        6 => [4],
    ];

    static ref WAVE_TEST: Wave = wave![
        0 => [1],
    ];

    // static ref WAVES: Vec<Wave> = vec![WAVE_1.clone(), WAVE_2.clone(), WAVE_3.clone()];
    static ref WAVES: Vec<Wave> = vec![WAVE_TEST.clone()];
       
}

const FPS: u64 = 20;
const FRAME_TIME: u64 = 1000 / FPS;

enum Msg {
    Shoot,
    MoveUp,
    MoveDown,
    KillAll,
    ExectuteAction(usize, usize),
    NewAction(ActionOnBoard),
    NextWave,
    UpgradePlayer,
    Tick,
}

struct Model {
    link: ComponentLink<Self>,
    game: Game,
    show_grid: bool,
    victory: bool,
    ticker: Box<dyn Task>,
}

#[derive(Debug, Properties, Clone)]
struct ListProps {
    list_turrets: Vec<Turret>,
    add_turret: Callback<Turret>,
    on_delete_mode: Callback<()>,
    money: u32,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ticker = IntervalService::spawn(
            Duration::from_millis(FRAME_TIME),
            link.callback(|_| Msg::Tick),
        );

        let mut game = Game::default();
        game.add_waves(WAVES.clone());
        game.enemy_wave_assign_line();

        Self {
            link,
            game,
            show_grid: false,
            victory: false,
            ticker: Box::new(ticker),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                self.game.process();
                true
            }
            Msg::Shoot => self.game.player_shoot(),
            Msg::MoveUp => self.game.move_player_up(),
            Msg::MoveDown => self.game.move_player_down(),
            Msg::KillAll => {
                log!("Kill All");
                self.game.kill_all()
            }
            Msg::ExectuteAction(x, y) => {
                log!(x, y);
                if self.game.execute_action(x, y) {
                    self.show_grid = false;
                    true
                } else {
                    false
                }
            }
            Msg::NewAction(action) => {
                if self.game.action.as_ref() == Some(&action) {
                    self.show_grid = !self.show_grid;
                    self.game.action = None;
                } else {
                    self.game.action = Some(action);
                    self.show_grid = true;
                }
                true
            }
            Msg::UpgradePlayer => {
                let succes = self.game.upgrade_player();
                succes
            }
            Msg::NextWave => {
                log!("Next wave !");
                let no_more_wave = self.game.next_wave();
                let no_more_enemies = !self.game.remaining_enemies();
                log!(no_more_wave, no_more_enemies);
                if no_more_wave && no_more_enemies {
                    self.victory = true;
                }
                self.victory
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let header_props = HeaderProps {
            money: self.game.money,
            turrets: self.game.turret_list(),
            turret_selected: self.game.action.as_ref().as_ref().map(|act| act.get_turret_level()).flatten(),
            player_level: self.game.player.level,
            upgrade_cost_text: self.game.player.upgrade_cost_text(),
            on_turret_selected: self.link.callback(|turret: Turret| Msg::NewAction(ActionOnBoard::PlaceTurret(turret))),
            upgrade_player: self.link.callback(|_| Msg::UpgradePlayer)
        };

        let footer_props = FooterProps {
            god_level: self.game.god_level(),
            wave: 42,
            delete_mode: self.game.is_delete_mode(),
            active_god: self.link.callback(|_| Msg::KillAll),
            toggle_delete_mode: self.link.callback(|_| Msg::NewAction(ActionOnBoard::Delete)),
            start_next_wave: self.link.callback(|_| Msg::NextWave),
            wave_ended: self.game.is_wave_ended(),
        };

        html! {
            <>
                <Header with header_props/>
                <Board show_grid=self.show_grid>
                    { for self.game.lines.iter().enumerate().map(|(y, line)| {
                        let cells = line.cells.iter().map(|opt| if let Some(turret) = opt {
                            Some(turret.level())
                        } else {
                            None
                        }).collect::<Vec<_>>();

                        let player_level = if self.game.player.line == y {
                            Some(self.game.player.level)
                        } else {
                            None
                        };

                        let execute_action = self.link.callback(|(x, y)| Msg::ExectuteAction(x, y));

                        let game_row_props = GameRowProps {
                            player_level,
                            cells,
                            y,
                            execute_action,
                            show_grid: self.show_grid,
                            delete_mode: self.game.is_delete_mode(),
                            enemies: line.enemies.clone(),
                            projectiles: line.projectiles.clone(),
                        };

                        html_nested!( <GameRow with game_row_props/> )
                    })}
                </Board>
                <Footer with footer_props>
                </Footer>
            </>
        }
    }
}

fn main() {

    yew::start_app::<Model>();
}
