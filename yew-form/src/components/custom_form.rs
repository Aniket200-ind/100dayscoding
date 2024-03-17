use yew::prelude::*;
use crate::components::text_field::TextField;

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let username_state = use_state(|| "".to_string());
    let email_state = use_state(|| "".to_string());
    let password_state = use_state(|| "".to_string());

    let username = username_state.clone();
    let email = email_state.clone();
    let password = password_state.clone();

    let username_changed = Callback::from(
        move |username: String| {
            username_state.set(username);
        }
    );

    let email_changed = Callback::from(
        move |email: String| {
            email_state.set(email);
        }
    );

    let password_changed = Callback::from(
        move |password: String| {
            password_state.set(password);
        }
    );

    html! {
        <form class="flex flex-col bg-oxford-blue p-4 rounded-lg w-2/6">
            <TextField 
                label_for_text={"Name".to_string()}
                label_value={"Name".to_string()}
                input_type={"text".to_string()}
                input_id={"Name".to_string()}
                input_name={"Name".to_string()}
                handle_change={username_changed}
            />
            <TextField 
                label_for_text={"Email".to_string()}
                label_value={"Email".to_string()}
                input_type={"email".to_string()}
                input_id={"Email".to_string()}
                input_name={"Email".to_string()}
                handle_change={email_changed}
            />
            <TextField 
                label_for_text={"Password".to_string()}
                label_value={"Password".to_string()}
                input_type={"password".to_string()}
                input_id={"Password".to_string()}
                input_name={"Password".to_string()}
                handle_change={password_changed}
            />
            <p class="text-platinum font-semibold">
                {"Name: "}
            <span class="text-orange font-semibold">
                {format!("{}", &*username)}
            </span>
            </p>
            <p class="text-platinum font-semibold">
                {"Email: "}
            <span class="text-orange font-semibold">
                {format!("{}", &*email)}
            </span>
            </p>
            <p class="text-platinum font-semibold">
                {"Password: "}
            <span class="text-orange font-semibold">
                {format!("{}", &*password)}
            </span>
            </p>
        </form>
    }
}
