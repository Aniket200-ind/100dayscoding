use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
            <div class="
            flex justify-center items-center flex-col h-screen bg-mint-green text-white p-4
            ">
            <h1 class="text-5xl font-bold text-raisin-black">
                { "Contact" }
            </h1>
            <p class="mt-4 text-2xl text-delft-blue">
                { "Welcome to the contact page." }
            </p>
            <p class="mt-4 text-2xl text-delft-blue">
                { "You can contact me at aniketbotre007@gmail.com" }
            </p>
        </div>
    }
}
