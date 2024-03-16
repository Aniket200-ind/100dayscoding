use yew::prelude::*;
use crate::components::text_field::TextField;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html! {
        <form class="flex flex-col bg-oxford-blue p-4 rounded-lg w-2/6">
            <TextField 
                label_for_text={"Name".to_string()}
                label_value={"Name".to_string()}
                input_type={"text".to_string()}
                input_id={"Name".to_string()}
                input_name={"Name".to_string()}
            />
            <TextField 
                label_for_text={"Email".to_string()}
                label_value={"Email".to_string()}
                input_type={"email".to_string()}
                input_id={"Email".to_string()}
                input_name={"Email".to_string()}
            />
            <TextField 
                label_for_text={"Password".to_string()}
                label_value={"Password".to_string()}
                input_type={"password".to_string()}
                input_id={"Password".to_string()}
                input_name={"Password".to_string()}
            />
            <button 
            class="bg-orange text-black p-2 rounded-lg mt-4 font-bold font-monospace text-xl outline-none border-none" 
            type="submit"
            >
                {"Submit"}
            </button>
        </form>
    }
}
