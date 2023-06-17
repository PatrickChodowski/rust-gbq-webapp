// example component

use yew::prelude::*;

#[function_component]
pub fn Display() -> Html {

    let label: &str = "This is Display Page";

    html! {
        <div>
            <p>{label}</p>
        </div>
    }
}