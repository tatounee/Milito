use std::rc::Rc;

use yew::prelude::*;

use super::card::Card;
use crate::game::turret::Turret;

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct HeaderProps {
    pub money: u32,
    pub turrets: Rc<Vec<Rc<Turret>>>,
    pub turret_selected: Option<u8>,
    pub player_level: u8,
    pub upgrade_cost_text: Rc<String>,
    pub on_turret_selected: Callback<Turret>,
    pub upgrade_player: Callback<()>,
}

pub struct Header {
    link: ComponentLink<Self>,
    props: HeaderProps,
}

pub enum Msg {
    TurretSelected(Turret),
    UpgradePlayer,
}

impl Component for Header {
    type Message = Msg;
    type Properties = HeaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
            Msg::TurretSelected(t) => self.props.on_turret_selected.emit(t),
            Msg::UpgradePlayer => self.props.upgrade_player.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("shop")>
                <div class=classes!("buy")>
                    <div class=classes!("upgrade")>
                        <Card<()>
                            onclick=self.link.callback(|_| Msg::UpgradePlayer)
                            level=self.props.player_level
                            definition=128
                            onclick_value=Rc::new(())
                            img="player-upgrade".to_owned()
                            price_text=self.props.upgrade_cost_text.clone()
                        />
                    </div>
                    <div class="turrets">
                        { for self.props.turrets.iter().map(|turret| {
                            let is_turret_selected = self.props.turret_selected.map(|lvl| lvl == turret.level()).unwrap_or(false);
                            html_nested!{
                            <Card<Turret>
                                selected=is_turret_selected
                                level=turret.level()
                                price_text=turret.price_text()
                                definition=128
                                onclick=self.link.callback(|rc_t: Rc<Turret>| Msg::TurretSelected((*rc_t).clone()))
                                onclick_value=turret.clone()
                                img="turret".to_owned()
                            />
                        }}) }
                    </div>
                </div>
                <div class=classes!("data")>
                    <div class=classes!("sold")>
                        { self.props.money }
                    </div>
                    <div class=classes!("help")>
                        { "?" }
                    </div>
                </div>
            </div>
        }
    }
}
