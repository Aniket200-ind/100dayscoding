use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{"Hello, Rustaceans!"}</h1>
            <p>{"This is a simple static html page build using Yew.rs"}</p>
        </div>
        }
}

fn main() {
    yew::Renderer::<App>::new().render();
}