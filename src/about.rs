// example component

use yew::prelude::*;

#[function_component]
pub fn AboutPage() -> Html {

    let label: &str = "This is About Page";

    html! {
        <div>
            <p>{label}</p>
        </div>
    }
}