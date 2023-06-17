use yew::prelude::*;

mod about;
mod counter;
mod display;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <about::AboutPage />
            <counter::Counter />
            <display::Display />
        </>
    }
}



fn main() {
    yew::Renderer::<App>::new().render();
}
