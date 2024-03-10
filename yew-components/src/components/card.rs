use yew::prelude::*;

#[function_component(Card)]
pub fn card() -> Html {
    html! {
    // create a eye catchy design for card using tailwind css
    <div class="bg-emerald-100 shadow-lg rounded-lg p-4">
        <img src="https://mock.jpg" class="w-full rounded-lg" />
        <div class="flex justify-between items-center mt-4">
            <div>
                <div class="text-lg font-bold">{"Title"}</div>
                <div class="text-sm">{"Description"}</div>
            </div>
            <div class="text-xl font-bold text-green-700">{"$100"}</div>
        </div>
    </div>
    }
}