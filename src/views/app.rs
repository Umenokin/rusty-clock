use yew::prelude::*;
use chrono::Utc;
use chrono_tz::{
    Asia::Tokyo,
    America::New_York,
    Europe::Paris,
};

use gloo::{
    timers::callback::{Interval}
};

use crate::components::*;

#[function_component]
pub fn App() -> Html {
    let time = use_state(|| Utc::now());
    let time_state: UseStateHandle<chrono::DateTime<Utc>> = time.clone();
    use_effect_with_deps(|_| {
        let interval = Interval::new(1000, move || time_state.set(Utc::now()));
        || drop(interval)
    }, ());

    html! {
        <div class="board">
            <div>
                <Clock time={time.with_timezone(&Tokyo)} />
                <Plate>{"Tokyo"}</Plate>
            </div>
            <div>
                <Clock time={time.with_timezone(&New_York)} />
                <Plate>{"New York"}</Plate>
            </div>

            <div>
                <Clock time={time.with_timezone(&Paris)} />
                <Plate>{"Paris"}</Plate>
            </div>
        </div>
    }
}
