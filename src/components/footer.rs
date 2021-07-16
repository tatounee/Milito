
use yew::prelude::*;

use crate::game::GOD_LEVEL_MAX;

pub struct Footer {
    props: FooterProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct FooterProps {
    pub god_level: u32,
    pub wave: u32,
    pub active_god: Callback<()>,
    pub toggle_delete_mode: Callback<()>,
}

pub enum Msg {
    God,
    DeleteMode
}

impl Component for Footer {
    type Message = Msg;
    type Properties = FooterProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
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
            Msg::God => {
                self.props.active_god.emit(());
            }
            Msg::DeleteMode => {
                self.props.toggle_delete_mode.emit(());
            }
        }
        false
    }

    fn view(&self) -> Html {
        let god_img_classes = format!("god-img level{}-64", self.props.god_level);

        html! {
            <footer>
                <button class="god" onclick=self.link.callback(|_| Msg::God) disabled = self.props.god_level != GOD_LEVEL_MAX>
                    <div class=classes!(god_img_classes)></div>
                </button>
                <div class="info">
                    <div class="trashcan" onclick=self.link.callback(|_| Msg::DeleteMode)>
                        { "🗑️" }
                    </div>
                    <div class="wave">
                        <span>{ "Wave " }</span>
                        <span>{ self.props.wave }</span>
                    </div>
                </div>
            </footer>
        }
    }
}