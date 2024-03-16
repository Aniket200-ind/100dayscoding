use yew::prelude::*;
use gloo::console::log;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub label_for_text: String,
    pub label_value: String,
    pub input_type: String,
    pub input_id: String,
    pub input_name: String
}

// set state and log out the user in;put value in p tag

#[function_component(TextField)]
pub fn text_field(props: &Props) -> Html {
    let handle_change = Callback::from(
        |event: Event| {
            let target = event.target().unwrap();
            log!(target);
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
                onchange={handle_change}
            />
        </>
    }
}