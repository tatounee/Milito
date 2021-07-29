
mod components;
mod game;
mod utils;

use std::time::Duration;

use yew::{
    prelude::*, 
    services::{
        IntervalService,
        DialogService,
        Task, 
        keyboard::{
            KeyListenerHandle, 
            KeyboardService
        }
    },
    utils::window,
};
use lazy_static::lazy_static;

use components::{Footer, FooterProps, Header, HeaderProps};
use game::{ActionOnBoard, Game, turret::Turret, wave::Wave};

use crate::components::{Board, GameRow, GameRowProps};

lazy_static! {
    static ref WAVE_1: Wave = wave![
        0 => [1],
        12 => [1],
        25 => [1],
        30 => [1],
    ];

    static ref WAVE_2: Wave = wave![
        0 => [1, 1],
        15 => [1],
        25 => [1],
        30 => [1],
        47 => [1, 1],
    ];

    static ref WAVE_3: Wave = wave![
        0 => [2],
        15 => [1],
        25 => [1],
        35 => [1, 1],
        40 => [1],
        50 => [1],
        55 => [2, 1],
        60 => [1, 1, 1, 1],
        65 => [2, 2],
    ];

    static ref WAVE_4: Wave = wave![
        0 => [1, 1],
        5 => [1],
        15 => [3],
        25 => [1, 1],
        35 => [2, 1],
        40 => [1],
        50 => [2, 3],
    ];
    
    static ref WAVE_5: Wave = wave![
        0 => [3, 3],
        10 => [1, 1, 1],
        20 => [2],
        25 => [1, 1],
        35 => [3, 1],
        45 => [2],
        60 => [1, 1, 1, 1, 1],
    ];

    static ref WAVE_6: Wave = wave![
        0 => [2, 1],
        10 => [2, 2],
        20 => [3],
        25 => [3, 3],
        35 => [1, 1, 1, 1],
        50 => [5],
        60 => [1, 1, 2],
        70 => [2, 2, 3],
        80 => [1, 1, 1, 1],
        90 => [3, 3, 1]
    ];

    static ref WAVE_7: Wave = wave![
        0 => [3, 3, 3],
        5 => [1, 1],
        20 => [3],
        25 => [2, 1],
        35 => [1, 1, 3],
        40 => [2, 2, 2],
        50 => [2, 2],
        60 => [1, 1, 2],
        70 => [4, 1],
        80 => [3, 3, 3, 1],
        85 => [2, 3],
        95 => [1, 1, 2, 3],
        100 => [1, 1, 2, 3],
    ];


    static ref WAVE_8: Wave = wave![
        0 => [2, 2, 1],
        5 => [2, 2],
        10 => [1],
        15 => [1],
        20 => [3, 3, 3],
        25 => [4],
        35 => [1, 1],
        45 => [2, 2, 2, 2],
        50 => [2, 2, 2],
        60 => [1, 1, 1],
        70 => [4, 1],
        80 => [1, 1, 2],
        85 => [2, 3],
        95 => [4, 1, 1, 3],
        100 => [1, 1, 1],
        110 => [1],
        120 => [2, 2, 2, 2, 2],
    ];


    static ref WAVE_9: Wave = wave![
        0 => [2, 2, 1, 1, 3],
        5 => [3, 3],
        10 => [1, 1],
        20 => [4],
        35 => [4],
        45 => [1, 1, 1, 1],
        48 => [1, 1, 1],
        52 => [2, 1, 1, 1],
        60 => [3, 2],
        70 => [1, 1],
        80 => [4, 1, 2],
        85 => [2, 3, 1],
        95 => [3, 1, 1, 3],
        100 => [1, 1, 2, 2],
        110 => [1, 2, 3, 3],
        120 => [2, 2, 3, 3],
        125 => [1, 1],
        130 => [4],
        140 => [2, 2, 2, 3],
    ];


    static ref WAVE_10: Wave = wave![
        0 => [1],
        1 => [1],
        2 => [1],
        3 => [1],
        4 => [1],
        5 => [2],
        6 => [2],
        7 => [2],
        8 => [2],
        9 => [2],
        10 => [3, 3, 3],
        20 => [3, 3, 3, 3],
        35 => [3, 3, 3, 3, 3],
        45 => [4, 1],
        50 => [2, 2],
        60 => [2, 4, 4],
        70 => [2, 3, 3, 2],
        80 => [2, 2, 1, 1],
        85 => [2, 3, 4],
        95 => [3, 1, 1, 4],
        100 => [3, 2, 2, 2],
        110 => [2, 3, 3],
        120 => [2, 3, 2],
        125 => [1, 1, 1, 1, 1],
        130 => [4],
        140 => [2, 2, 2, 2, 2],
        145 => [2, 2, 2, 2, 2],
        150 => [2, 2, 2, 2, 2],
    ];

    static ref WAVE_11: Wave = wave![
        0 => [4],
        10 => [1, 1, 1, 1, 1],
        13 => [1, 1, 1, 1],
        18 => [1, 1, 1, 1],
        20 => [1, 1, 1, 1, 1],
        25 => [1, 1, 1, 1],
        35 => [4],
        45 => [4, 3, 3],
        50 => [2, 2, 2],
        60 => [2, 2, 2, 1],
        70 => [2, 4],
        80 => [2, 4],
        85 => [2, 3, 4],
        95 => [3, 3, 3, 3],
        100 => [3, 3, 3, 3],
        105 => [3, 3, 3, 3, 3],
        110 => [3, 3, 3, 3],
        115 => [3, 3, 3, 3, 3],
        120 => [3, 3, 3, 3],
        125 => [1, 1, 1, 1, 1],
        130 => [4],
        140 => [2, 3, 3],
        150 => [2, 2, 2, 1],
        155 => [1, 1],
        160 => [3, 2],
        170 => [2, 2, 2, 2, 2],
        175 => [2, 2, 2, 2, 2],
        180 => [2, 2, 2, 2, 4],
        185 => [2, 2, 2, 2, 4],
        195 => [2, 2, 2, 2, 2],
        200 => [4, 1, 1, 1, 1],
        210 => [3, 3, 3, 3, 3],
        220 => [4, 1, 2, 3, 1],
    ];

    static ref WAVES: Vec<Wave> = vec![
        WAVE_1.clone(), 
        WAVE_2.clone(), 
        WAVE_3.clone(),
        WAVE_4.clone(),
        WAVE_5.clone(),
        WAVE_6.clone(),
        WAVE_7.clone(),
        WAVE_8.clone(),
        WAVE_9.clone(),
        WAVE_10.clone(),
        WAVE_11.clone(),
    ];
}

const FPS: u64 = 20;
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
    input_handler: KeyListenerHandle
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

        let input_handler = KeyboardService::register_key_down(&window(), 
            link.callback(|key_event: KeyboardEvent| Msg::KeyDown(key_event)));

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
            input_handler
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
                    _ => ()
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
            turret_selected: self.game.action.as_ref().as_ref().map(|act| act.get_turret_level()).flatten(),
            player_level: self.game.player.level,
            upgrade_cost_text: self.game.player.upgrade_cost_text(),
            on_turret_selected: self.link.callback(|turret: Turret| Msg::NewAction(ActionOnBoard::PlaceTurret(turret))),
            upgrade_player: self.link.callback(|_| Msg::UpgradePlayer)
        };

        let footer_props = FooterProps {
            god_level: self.game.god_level(),
            wave: self.game.wave(),
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
