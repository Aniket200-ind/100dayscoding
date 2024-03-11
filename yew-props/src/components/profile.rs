use yew::prelude::*;
use crate::components::avatar::*;
use crate::Props;

#[function_component(Profile)]
pub fn profile(props: &Props) -> Html{
    html!{
        <div class="flex flex-col items-center space-y-8 bg-gray-800 rounded-lg p-4 m-4 w-96">
            <Avatar />
            <div class="text-center">
                <h1 class="text-2xl font-bold text-white">{&props.name}</h1>
                <p class="text-rose-700 font-bold">{"Age: "}{props.age}</p>
                <p class="text-cyan-500">{"Software Developer"}</p>
                <p class="text-yellow-400">{"I am a software developer with 5 years of experience in web development. I am a full stack developer with experience in both front-end and back-end development. I am a quick learner and a team player. I am passionate about learning new technologies and building new projects."}</p>
            </div>
        </div>
    }
}
