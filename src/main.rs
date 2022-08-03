use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="">
            <h1 class="text-3xl font-bold underline text-green-300">{"app page"}</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
