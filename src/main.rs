#![allow(dead_code)]

const TURRET_LIST: &'static [Box<dyn Turret>] = &[];

const FPS: u64 = 10;
const FRAME_TIME: u64 = 1000 / FPS;

mod components;
mod game;

use std::time::Duration;

use yew::{
    prelude::*,
    services::{console::ConsoleService, IntervalService, Task},
};

use components::{Footer, FooterProps, Header, HeaderProps};
use game::{turret::Turret, ActionOnBoard, Game};

use crate::components::{Board, GameRow, GameRowProps};

enum Msg {
    Shoot,
    MoveUp,
    MoveDown,
    KillAll,
    ExectuteAction(usize, usize),
    ToggleDeleteMode,
    BuyTurrret(Box<dyn Turret>),
    Tick,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    game: Game,
    show_grid: bool,
    delete_mode: bool,
    ticker: Box<dyn Task>,
}

#[derive(Debug, Properties, Clone)]
struct ListProps {
    list_turrets: Vec<Box<dyn Turret>>,
    add_turret: Callback<Box<dyn Turret>>,
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

        Self {
            link,
            game: Game::default(),
            show_grid: false,
            ticker: Box::new(ticker),
            delete_mode: false,
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
                ConsoleService::log("Kill All");
                self.game.kill_all()
            }
            Msg::ExectuteAction(x, y) => {
                ConsoleService::log(&format!("x = {}, y = {}", x, y));
                if self.game.execute_action(x, y) {
                    self.delete_mode = false;
                    self.show_grid = false;
                    true
                } else {
                    false
                }
            }
            Msg::ToggleDeleteMode => {
                self.delete_mode = !self.delete_mode;
                ConsoleService::log(&format!("Toggle delete mode -> {}", self.delete_mode));
                if self.delete_mode {
                    self.game.action = Some(ActionOnBoard::Delete);
                    self.show_grid = true;
                } else {
                    self.game.action = None;
                    self.show_grid = false;
                }
                true
            }
            Msg::BuyTurrret(turret) => {
                self.game.action = Some(ActionOnBoard::PlaceTurret(turret));
                self.show_grid = true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let header_props = HeaderProps {
            money: self.game.money,
            turrets: TURRET_LIST,
            player_level: self.game.player.level,
            upgrade_cost: self.game.player.level as u32 * 1000,
            on_turret_selected: self
                .link
                .callback(|turret: Box<dyn Turret>| Msg::BuyTurrret(turret))
        };

        let footer_props = FooterProps {
            god_level: self.game.god_level(),
            wave: 42,
            active_god: self.link.callback(|_| Msg::KillAll),
            toggle_delete_mode: self.link.callback(|_| Msg::ToggleDeleteMode)
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
                        
                        let player = if self.game.player.line == y {
                            Some(self.game.player.line)
                        } else {
                            None
                        };

                        let execute_action = self.link.callback(|(x, y)| Msg::ExectuteAction(x, y));

                        let game_row_props = GameRowProps {
                            player,
                            cells,
                            y,
                            execute_action,
                            show_grid: self.show_grid,
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
