use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
            <div class="
            flex justify-center items-center flex-col h-screen bg-powder-blue text-white p-4
            ">
            <h1 class="text-5xl font-bold text-raisin-black"> 
                { "About" } 
            </h1>
            <p class="mt-4 text-2xl text-delft-blue"> 
                { "Welcome to the about page." } 
            </p>
            <p class="mt-4 text-2xl text-delft-blue"> 
                { "This website is built using Rust's framework Yew." } 
            </p>
        </div>
    }
}