mod cheat;
mod components;
mod game;
mod utils;

use std::time::Duration;

use cheat::Cheat;
use yew::{
    prelude::*,
    services::{
        keyboard::{KeyListenerHandle, KeyboardService},
        IntervalService, Task,
    },
    utils::window,
};

use components::{Footer, FooterProps, Header, HeaderProps};
use game::{turret::Turret, wave::WAVES, ActionOnBoard, Game};

use crate::{
    components::{Board, GameRow, GameRowProps, Hover, HoverProps},
    game::GameStats,
};

const FPS: u64 = 30;
const FRAME_TIME: u64 = 1000 / FPS;

enum Msg {
    KeyDown(KeyboardEvent),
    KillAll,
    ExectuteAction(usize, usize),
    NewAction(ActionOnBoard),
    AbortAction(Option<MouseEvent>),
    NextWave,
    Pause(bool),
    UpgradePlayer,
    Tick,
}

#[allow(dead_code)]
struct Model {
    link: ComponentLink<Self>,
    game: Game,
    show_grid: bool,
    ticker: Box<dyn Task>,
    input_handler: KeyListenerHandle,
    cheat: Cheat,
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

        let input_handler =
            KeyboardService::register_key_down(&window(), link.callback(Msg::KeyDown));

        let mut game = Game::default();
        game.add_waves(WAVES.clone());
        game.generate_waves(10);
        game.assign_line_for_enemies();

        let cheat = Cheat::new("ilovetatoune");

        Self {
            link,
            game,
            show_grid: false,
            ticker: Box::new(ticker),
            input_handler,
            cheat,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                self.game.process();
                true
            }
            Msg::KeyDown(event) => {
                let code = event.code();
                let key = event.key();
                if code.len() == 6 && &code[0..5] == "Digit" {
                    event.prevent_default();
                    if let Ok(nbr) = code[5..6].parse::<usize>() {
                        if let Some(turret) = self.game.turret_list().get(nbr.saturating_sub(1)) {
                            self.link
                                .send_message(Msg::NewAction(ActionOnBoard::PlaceTurret(
                                    turret.as_ref().clone(),
                                )))
                        }
                    }
                };

                match key.as_str() {
                    "ArrowUp" => self.game.move_player_up(),
                    "ArrowRight" => self.game.player_shoot(),
                    "ArrowDown" => self.game.move_player_down(),
                    "s" if self.cheat.is_active() => self.game.skip_one_wave(),
                    "g" => {
                        self.game.use_god();
                    }
                    "d" => self
                        .link
                        .send_message(Msg::NewAction(ActionOnBoard::Delete)),
                    " " => self.link.send_message(Msg::NextWave),
                    "u" => self.link.send_message(Msg::UpgradePlayer),
                    "Escape" => {
                        if self.game.have_action() {
                            self.link.send_message(Msg::AbortAction(None))
                        } else {
                            self.link.send_message(Msg::Pause(true))
                        }
                    }
                    _ => (),
                }

                if key.len() == 1 {
                    self.cheat.type_key(key.chars().next().unwrap())
                }

                false
            }
            Msg::KillAll => {
                self.game.use_god();
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
            Msg::AbortAction(event) => {
                if self.game.action.is_some() {
                    if let Some(event) = event {
                        event.prevent_default()
                    }
                    self.game.action = None;
                    self.show_grid = false;
                }
                false
            }
            Msg::NewAction(action) => {
                if self.game.action.as_ref() == Some(&action) {
                    self.game.action = None;
                    self.show_grid = false;
                } else if self.game.can_execut_action(&action) {
                    self.game.action = Some(action);
                    self.show_grid = true;
                } else {
                    self.game.action = None;
                    self.show_grid = false;
                };
                false
            }
            Msg::UpgradePlayer => {
                self.game.upgrade_player();
                false
            }
            Msg::Pause(toggle) => {
                self.game.pause(toggle);
                false
            }
            Msg::NextWave => {
                self.game.start_next_wave();
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hover_props = HoverProps {
            game_stats: self.game.stats.clone(),
            help: matches!(self.game.stats, GameStats::Pause(_)),
            make_pause: self.link.callback(|_| Msg::Pause(false)),
        };

        let header_props = HeaderProps {
            money: self.game.money,
            turrets: self.game.turret_list(),
            turret_selected: self
                .game
                .action
                .as_ref()
                .map(|act| act.get_turret_level())
                .flatten(),
            player_level: self.game.player.level,
            upgrade_cost_text: self.game.player.upgrade_cost_text(),
            on_turret_selected: self
                .link
                .callback(|turret: Turret| Msg::NewAction(ActionOnBoard::PlaceTurret(turret))),
            upgrade_player: self.link.callback(|_| Msg::UpgradePlayer),
            make_pause: self.link.callback(|_| Msg::Pause(true)),
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
            <body oncontextmenu=self.link.callback(|e| Msg::AbortAction(Some(e)))>
                <Hover with hover_props/>
                <Header with header_props/>
                <Board show_grid=self.show_grid>
                    { for self.game.lines.iter().enumerate().map(|(y, line)| {
                        let cells = line.cells.iter().map(|opt| opt.as_ref().map(|turret| turret.level())).collect::<Vec<_>>();

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
            </body>
        }
    }
}

fn main() {
    yew::initialize();
    yew::App::<Model>::new().mount_as_body();
    yew::run_loop();
}
