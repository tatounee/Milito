
use std::rc::Rc;

use yew::prelude::*;

use crate::log;

pub struct Card<T: 'static + Clone + PartialEq> {
    link: ComponentLink<Self>,
    props: CardProps<T>,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct CardProps<T: Clone + PartialEq> {
    pub level: u8,
    pub definition: u32,
    pub price_text: Rc<String>,
    pub img: String,
    pub onclick: Callback<Rc<T>>,
    pub onclick_value: Rc<T>,
    #[prop_or(false)]
    pub selected: bool,
}

pub enum Msg {
    Onclick
}

impl<T: 'static + Clone + PartialEq> Component for Card<T> {
    type Message = Msg;
    type Properties = CardProps<T>;

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
            Msg::Onclick => self.props.onclick.emit(self.props.onclick_value.clone())
        }
        false
    }

    fn view(&self) -> Html {
        let card_div_classes = format!(
            "level{}-{} {}-img",
            self.props.level, self.props.definition, self.props.img
        );

        let card_classes = format!("card {}",  if self.props.selected { "selected" } else { "" });

        html! {
            <div class=classes!(card_classes) onclick=self.link.callback(|_| Msg::Onclick)>
                <div class=classes!(card_div_classes)></div>
                <div>
                    { self.props.price_text.clone() }
                </div>
            </div>
        }
    }
}
