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
    pub delete_mode: bool,
    pub active_god: Callback<()>,
    pub toggle_delete_mode: Callback<()>,
    pub start_next_wave: Callback<()>,
    pub wave_ended: bool,
}

pub enum Msg {
    God,
    DeleteMode,
    NextWave,
}

impl Component for Footer {
    type Message = Msg;
    type Properties = FooterProps;

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
            Msg::God => self.props.active_god.emit(()),
            Msg::DeleteMode => self.props.toggle_delete_mode.emit(()),
            Msg::NextWave => self.props.start_next_wave.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        let god_img_classes = format!("god-img level{}-64", self.props.god_level);

        let trashcan_classes = format!(
            "info-button {}",
            if self.props.delete_mode { "selected" } else { "" }
        );

        html! {
            <footer>
                <button class="god" onclick=self.link.callback(|_| Msg::God) disabled=self.props.god_level != GOD_LEVEL_MAX>
                    <div class=classes!(god_img_classes)></div>
                </button>
                <div class="info">
                    <button class=classes!(trashcan_classes) onclick=self.link.callback(|_| Msg::DeleteMode)>
                        { "🗑️" }
                    </button>
                    <button class="info-button" onclick=self.link.callback(|_| Msg::NextWave) disabled=!self.props.wave_ended>
                        { "➡" }
                    </button>
                    <div class="wave">
                        <span>{ "Wave " }</span>
                        <span>{ self.props.wave }</span>
                    </div>
                </div>
            </footer>
        }
    }
}
