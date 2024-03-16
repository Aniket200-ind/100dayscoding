mod components;

use yew::prelude::*;
use components::custom_form::CustomForm;

#[function_component(App)]
fn app() -> Html{
    html!(
        // use tailwind css for styling
        <section class="flex flex-col space-y-6 bg-black justify-center items-center h-screen">
            <h1 class="text-4xl font-bold text-platinum font-serif">{"User Form"}</h1>
            <CustomForm />
        </section>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
