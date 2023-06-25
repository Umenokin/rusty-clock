use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn Plate(props: &Props) -> Html {
    html! {
        <div class="plate">{for props.children.iter()}</div>
    }
}