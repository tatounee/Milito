use std::cell::RefCell;

use yew::prelude::*;

use crate::game::{enemy::Enemy, projectile::Projectile};

pub struct GameRow {
    link: ComponentLink<Self>,
    props: GameRowProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct GameRowProps {
    pub player_level: Option<u8>,
    pub cells: Vec<Option<u8>>,
    pub execute_action: Callback<(usize, usize)>,
    pub y: usize,
    pub show_grid: bool,
    pub delete_mode: bool,
    pub projectiles: RefCell<Vec<Projectile>>,
    pub enemies: RefCell<Vec<Enemy>>,
}

pub enum Msg {
    ExectuteAction(usize),
}

impl Component for GameRow {
    type Message = Msg;
    type Properties = GameRowProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ExectuteAction(x) => self.props.execute_action.emit((x, self.props.y)),
        }
        false
    }

    fn view(&self) -> Html {
        let player = if let Some(level) = self.props.player_level {
            let player_classes = format!("player-img level{}-128 free", level);
            html_nested!(
                <div class=classes!(player_classes)/>
            )
        } else {
            html_nested!()
        };

        html! {
            <div class="game-row">
                <div>
                    { player }
                </div>
                <div class="path">
                    <img src="assets/images/laser_balise.png" alt="balise" />
                    <div class="laser-img"></div>
                    <img src="assets/images/laser_balise.png" alt="balise" />
                </div>
                { for self.props.projectiles.borrow().iter().map(|proj| {
                    let player_proj = if proj.from_player() { "player-" } else { "" };
                    let projectile_classes = format!("{}projectile-img level{}-32 free projectile", player_proj, proj.level());
                    let projectile_pos = format!("left: {}%", proj.x());
                    html_nested! {
                        <div class=classes!(projectile_classes) style=projectile_pos/>
                    }
                }) }
                { for self.props.enemies.borrow().iter().map(|enemy| {
                    let enemy_classes = format!("enemy-img level{}-128 free", enemy.level());
                    let enemy_datas = format!("left: {}%; transform: scale({})", enemy.x(), enemy.scale());
                    html_nested! {
                        <div class=classes!(enemy_classes) style=enemy_datas/>
                    }
                }) }
                <div class="board-row">
                    { for self.props.cells.iter().enumerate().map(|(x, turret)| {
                        let turret = turret.map(|level| {
                            let turret_classes = format!("turret-img level{}-128 free", level);
                            html_nested!(
                                <div class=classes!(turret_classes) />
                            )
                        });
                        html_nested!(
                            <button class="cell" onclick=self.link.callback(move |_| Msg::ExectuteAction(x)) disabled=!self.props.show_grid || (turret.is_some() && !self.props.delete_mode)>
                                { turret.unwrap_or_else(|| html_nested!()) }
                            </button>
                        )
                    }) }
                </div>
            </div>
        }
    }
}
