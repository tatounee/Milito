
use yew::worker::*;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

pub struct InputHandler {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>
}

impl Agent for InputHandler {
    type Reach = Context<Self>;

}