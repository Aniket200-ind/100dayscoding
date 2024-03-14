use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="
        flex justify-center items-center flex-col h-screen bg-delft-blue text-white p-4
        ">
            <h1 class="text-5xl font-bold"> { "Home" } </h1>
            <p class="mt-4 text-2xl"> { "Welcome to the home page." } </p>
        </div>
    }
}
