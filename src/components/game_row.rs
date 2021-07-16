use yew::prelude::*;

pub struct GameRow {
    link: ComponentLink<Self>,
    props: GameRowProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct GameRowProps {
    pub player: Option<usize>,
    pub cells: Vec<Option<u8>>,
    pub execute_action: Callback<(usize, usize)>,
    pub y: usize,
    pub show_grid: bool,
}

pub enum Msg {
    ExectuteAction(usize)
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
            Msg::ExectuteAction(x) => self.props.execute_action.emit((x, self.props.y))
        }
        false
    }

    fn view(&self) -> Html {
        let player = if let Some(level) = self.props.player {
            let player_classes = format!("player-img level{}-128", level);
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
                <div class="board-row">
                    { for self.props.cells.iter().enumerate().map(|(x, turret)| {
                        let turret = if let Some(level) = turret {
                            let turret_classes = format!("turret-img level{}-128", level);
                            html_nested!(
                                <div class=classes!(turret_classes)></div>
                            )
                        } else {
                            html_nested!()
                        };
                        html_nested!(
                            <button class="cell" onclick=self.link.callback(move |_| Msg::ExectuteAction(x)) disabled=!self.props.show_grid>
                                { turret }
                            </button>
                        )
                    }) }
                </div>
            </div>
        }
    }
}
