mod components;
mod game;
mod utils;

use std::time::Duration;

use yew::{
    prelude::*,
    services::{
        keyboard::{KeyListenerHandle, KeyboardService},
        DialogService, IntervalService, Task,
    },
    utils::window,
};

use components::{Footer, FooterProps, Header, HeaderProps};
use game::{turret::Turret, waves::WAVES, ActionOnBoard, Game};

use crate::components::{Board, GameRow, GameRowProps};

const FPS: u64 = 10;
const FRAME_TIME: u64 = 1000 / FPS;

enum Msg {
    KeyDown(KeyboardEvent),
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
    no_more_wave: bool,
    ticker: Box<dyn Task>,
    input_handler: KeyListenerHandle,
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

        let input_handler = KeyboardService::register_key_down(
            &window(),
            link.callback(|key_event: KeyboardEvent| Msg::KeyDown(key_event)),
        );

        let mut game = Game::default();
        game.add_waves(WAVES.clone());
        game.enemy_wave_assign_line();

        Self {
            link,
            game,
            show_grid: false,
            victory: false,
            no_more_wave: false,
            ticker: Box::new(ticker),
            input_handler,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                self.game.process();
                let no_more_enemies = !self.game.is_remaining_enemies();
                if self.no_more_wave && no_more_enemies {
                    self.victory = true;
                    DialogService::alert("GG !")
                } else if self.game.defeat {
                    DialogService::alert("DEFEAT :(")
                }
                true
            }
            Msg::KeyDown(key) => {
                match key.key_code() {
                    38 => self.game.move_player_up(),
                    39 => self.game.player_shoot(),
                    40 => self.game.move_player_down(),
                    _ => (),
                }
                false
            }
            Msg::KillAll => {
                self.game.kill_all();
                false
            }
            Msg::ExectuteAction(x, y) => {
                if self.game.execute_action(x, y) {
                    self.show_grid = false;
                    true
                } else {
                    false
                }
            }
            Msg::NewAction(action) => {
                if self.game.action.as_ref() == Some(&action) {
                    self.game.action = None;
                    self.show_grid = false;
                } else {
                    if self.game.can_execut_action(&action) {
                        self.game.action = Some(action);
                        self.show_grid = true;
                    } else {
                        self.game.action = None;
                        self.show_grid = false;
                    }
                };
                false
            }
            Msg::UpgradePlayer => {
                self.game.upgrade_player();
                false
            }
            Msg::NextWave => {
                self.no_more_wave = self.game.next_wave();
                false
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
            turret_selected: self
                .game
                .action
                .as_ref()
                .as_ref()
                .map(|act| act.get_turret_level())
                .flatten(),
            player_level: self.game.player.level,
            upgrade_cost_text: self.game.player.upgrade_cost_text(),
            on_turret_selected: self
                .link
                .callback(|turret: Turret| Msg::NewAction(ActionOnBoard::PlaceTurret(turret))),
            upgrade_player: self.link.callback(|_| Msg::UpgradePlayer),
        };

        let footer_props = FooterProps {
            god_level: self.game.god_level(),
            wave: self.game.wave(),
            delete_mode: self.game.is_delete_mode(),
            active_god: self.link.callback(|_| Msg::KillAll),
            toggle_delete_mode: self
                .link
                .callback(|_| Msg::NewAction(ActionOnBoard::Delete)),
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
