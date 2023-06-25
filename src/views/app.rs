use yew::prelude::*;
use chrono::Utc;
use chrono_tz::{
    Asia::Tokyo,
    America::New_York,
    Europe::Paris,
};


use crate::components::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="board">
            <div>
                <Clock time={Utc::now().with_timezone(&Tokyo)} />
                <Plate>{"Tokyo"}</Plate>
            </div>
            <div>
                <Clock time={Utc::now().with_timezone(&New_York)} />
                <Plate>{"New York"}</Plate>
            </div>

            <div>
                <Clock time={Utc::now().with_timezone(&Paris)} />
                <Plate>{"Paris"}</Plate>
            </div>
        </div>
    }
}
