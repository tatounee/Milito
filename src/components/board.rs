
use yew::{prelude::*, html::ChildrenWithProps};

use super::game_row::GameRow;

pub struct Board {
    props: BoardProps,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct BoardProps {
    pub show_grid: bool,
    pub children: ChildrenWithProps<GameRow>
}

impl Component for Board {
    type Message = ();
    type Properties = BoardProps;

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

        let board_classes = format!("board {}", if self.props.show_grid { "show" } else { "" });

        html! {
            <div class="game-board">
                <div class=classes!(board_classes)>
                    { for self.props.children.iter() }
                </div>
            </div>
        }
    }
}
