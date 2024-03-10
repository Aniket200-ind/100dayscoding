use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
    <nav class="bg-gray-800 py-4 px-2">
        <div class="container mx-auto flex justify-between items-center">
            <div class="text-white font-bold text-2xl">{"Yew"}</div>
            <div>
                <a href="#" class="text-white p-2">{"Home"}</a>
                <a href="#" class="text-white p-2">{"About"}</a>
                <a href="#" class="text-white p-2">{"Contact"}</a>
            </div>
        </div>
    </nav>
    }
}