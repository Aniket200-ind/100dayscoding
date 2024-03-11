mod components;

use yew::prelude::*;
use components::profile::Profile;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub name: String,
    pub age: u8,
}


#[function_component(App)]
fn app() -> Html {
    html!{
        <section class="flex flex-wrap justify-center h-full bg-gray-900">
            <Profile name="Jake Smith" age=25 />
            <Profile name="John Doe" age=30 />
            <Profile name="Jane Doe" age=28 />
            <Profile name="John Smith" age=35 />
            <Profile name="Jane Smith" age=40 />
            <Profile name="Jake Doe" age=45 />
            <Profile name="Jake Smith" age=50 />
            <Profile name="Zack Synder" age=55 />
            <Profile name="John Wick" age=60 />
            <Profile name="Jane Wick" age=65 />
        </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}