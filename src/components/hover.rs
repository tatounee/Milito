use yew::prelude::*;

use crate::game::GameStats;

pub struct Hover {
    link: ComponentLink<Self>,
    props: HoverProps,
}

#[derive(Properties, PartialEq, Clone)]
pub struct HoverProps {
    pub game_stats: GameStats,
    pub make_pause: Callback<()>,
    pub more_wave: Callback<u32>,
}

pub enum Msg {
    Unpause,
    MoreWave(u32),
}

impl Component for Hover {
    type Message = Msg;
    type Properties = HoverProps;

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
            Msg::Unpause => {self.props.make_pause.emit(()); false},
            Msg::MoreWave(amount) => {self.props.more_wave.emit(amount); true},
        }
    }

    fn view(&self) -> Html {
        match &self.props.game_stats {
            GameStats::Playing => html! {},
            GameStats::Pause(_) => html! {
                <div class="hover" onclick=self.link.callback(|_| Msg::Unpause)>
                    <div class="help-text">
                        <p>
                            { "The story begin when the Pikpik empire have been attack by the Ronron empire. Your king has assigned you the task of defending the kingdom, good luck." }
                        </p>
                        <h2> { "Controle" } </h2>
                        <ul>
                            <li><kbd>{"↑"}</kbd> {" and "} <kbd>{"↓"}</kbd> {" to move the player up and down"}</li>
                            <li><kbd>{"→"}</kbd>{" to shoot"}</li>
                            <li><kbd title="The space bar" style="cursor: help;">{"␣"}</kbd>{" to start the game and call the next wave"}</li>
                            <li><kbd>{"G"}</kbd> {" to call god"}</li>
                            <li><kbd>{"U"}</kbd> {" to upgrade the player"}</li>
                            <li><kbd>{"D"}</kbd> {" to delete a turret"}</li>
                            <li> <kbd title="The keys above your letters, not the numerical keypad" style="cursor: help;">{"1..6"}</kbd> {" to buy a new turret"}</li>
                            <li><kbd>{"Esc"}</kbd> {" or "} <kbd>{"Right click"}</kbd> {" to abort the current action"}</li>
                        </ul>
                        <p>{"Made with ♥ by "} <a href="https://github.com/tatounee/milito" target="_blank" >{"Tatoune"}</a> {"."}</p>
                    </div>
                </div>
            },
            x @ (GameStats::Defeat | GameStats::Victory) => html! {
                <div class="hover" onclick=self.link.callback(|_| Msg::Unpause)>
                    <div class="game-stats">
                        <div class="stats">{if matches!(x, GameStats::Victory) { "Victory"} else { "Defeat" }}</div>
                        <p>
                        {"If you want to replay, press " } <kbd> { "F5" } </kbd> { " or reload the page." } <br/>
                        { if matches!(x, GameStats::Victory) { 
                            html_nested! { <> <br/>
                                { "PS: I will tell you a secret, you can active a cheat mode by typing \"ilovetatoune\" and then, you can skip a wave if you press " } <kbd> { "S" } </kbd> { " when no wave is running." } 
                                </>}
                        } else {
                            html_nested! {}
                        }}
                        </p>
                        { if matches!(x, GameStats::Victory) {
                            html_nested! { 
                                <button class="more-wave" onclick=self.link.callback(|_| Msg::MoreWave(5))>{"I want to play 5 more wave !"}</button> }
                        } else {
                            html_nested! {}
                        }}
                    </div>
                </div>
            },
        }
    }
}
