use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub label_for_text: String,
    pub label_value: String,
    pub input_type: String,
    pub input_id: String,
    pub input_name: String,
    pub handle_change: Callback<String>
}

// set state and log out the user in;put value in p tag

#[function_component(TextField)]
pub fn text_field(props: &Props) -> Html {
    let handle_change = props.handle_change.clone();

    let handle_onchange = Callback::from(
        move |event: Event| {
            let target = event.target().unwrap();
            let input_text = target.unchecked_into::<HtmlInputElement>().value();
            handle_change.emit(input_text);
        }
    );

    html! {
        <>
        <label 
            class="text-platinum font-semibold" 
            for={props.label_for_text.clone()}
        >
            {&props.label_value}
        </label>
            <input 
                class="bg-gray-700 p-2 rounded-lg my-2 outline-none text-orange focus:ring-2 focus:ring-orange"
                type={props.input_type.clone()} 
                id={props.input_id.clone()}   
                name={props.input_name.clone()}
                onchange={handle_onchange}
            />
        </>
    }
}