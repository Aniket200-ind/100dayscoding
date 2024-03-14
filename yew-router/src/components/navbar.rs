use crate::route::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="bg-raisin-black px-4 py-2">
            <div class="container mx-auto flex justify-between items-center">
                <div class="text-white font-bold text-3xl">
                    { "Yew" }
                </div>
                <div class="flex space-x-4">
                    <Link<Route> to={Route::Home}>
                        <p class="text-mint-green font-bold"> { "Home" } </p>
                    </Link<Route>>
                    <Link<Route> to={Route::About}>
                        <p class="text-mint-green font-bold"> { "About" } </p>
                    </Link<Route>>
                    <Link<Route> to={Route::Contact}>
                        <p class="text-mint-green font-bold"> { "Contact" } </p>
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}
