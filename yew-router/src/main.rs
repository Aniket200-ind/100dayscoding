mod components;
mod route;

use components::navbar::Navbar;
use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!(<components::home::Home />),
        Route::About => html!(<components::about::About />),
        Route::Contact => html!(<components::contact::Contact />),
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html!(
        <BrowserRouter>
            <Navbar />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    )
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
