use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let increment = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let decrement = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! (
        <div class="min-h-screen bg-gray-100 flex items-center justify-center">
            <div class="bg-white p-4 rounded-lg shadow-lg">
                <h1 class="text-2xl font-bold text-center mb-4">{"Yew Counter"}</h1>
                <div class="flex justify-center items-center">
                    {
                        if *counter > 0 {
                            html! {
                                <button onclick={decrement} class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded">
                                    {"-"}
                                </button>
                            }
                        } else {
                            html! {
                                <button class="bg-red-500 text-white font-bold py-2 px-4 rounded cursor-not-allowed opacity-50" disabled=true>
                                    {"-"}
                                </button>
                            }
                        }
                    }
                    <p class="text-3xl font-bold mx-4">{*counter}</p>
                    <button onclick={increment} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                        {"+"}
                    </button>
                </div>
            </div>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
