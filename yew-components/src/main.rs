mod components;

use yew::prelude::*;
use crate::components::card::Card;
use crate::components::navbar::Navbar;

// TODO: Next learn how to fetch api and display data. Then build a project to display that displays github users using API

#[function_component(App)]
fn app() -> Html {
    html! {
        <section class="mx-auto">
            <Navbar />
            <div class="grid grid-cols-3 gap-4 p-4">
                <Card />
                <Card />
                <Card />
            </div>
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}