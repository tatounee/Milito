use yew::prelude::*;

pub struct Card {
    props: CardProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct CardProps {
    pub level: u8,
    pub definition: u32,
    pub price: u32,
    pub img: String,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let card_classes = format!(
            "level{}-{} {}-img",
            self.props.level, self.props.definition, self.props.img
        );

        html! {
            <div class=classes!("card")>
                <div class=classes!(card_classes)></div>
                <div>
                    { self.props.price }
                </div>
            </div>
        }
    }
}
