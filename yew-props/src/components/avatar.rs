use yew::prelude::*;

#[function_component(Avatar)]
pub fn avatar() -> Html{
    html!{
        <img 
        src="https://png.pngtree.com/png-vector/20220709/ourmid/pngtree-businessman-user-avatar-wearing-suit-with-red-tie-png-image_5809521.png" 
        alt="Avatar image"
        class="rounded-full h-16 w-16"
        />
    }
}