use yew::prelude::*;

use super::card::Card;
use crate::game::turret::Turret;

#[derive(Debug, Properties, Clone)]
pub struct HeaderProps {
    pub money: u32,
    pub turrets: &'static [Box<dyn Turret>],
    pub player_level: u8,
    pub upgrade_cost: u32,
    pub on_turret_selected: Callback<Box<dyn Turret>>,
}

pub struct Header {
    props: HeaderProps,
}

impl Component for Header {
    type Message = ();
    type Properties = HeaderProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.money != props.money {
            self.props.money = props.money;
            true
        } else if self.props.player_level != props.player_level {
            self.props.player_level = props.player_level;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("shop")>
                <div class=classes!("buy")>
                    <div class=classes!("upgrade")>
                        <Card level=self.props.player_level definition=128 img="player-upgrade".to_owned() price=self.props.upgrade_cost/>
                    </div>
                    <div>
                        { for self.props.turrets.iter().map(|turret| html_nested!{ <Card level=turret.level() price=turret.price() definition=128 img="turret".to_owned()/> })}
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
