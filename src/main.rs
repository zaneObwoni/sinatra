mod pages;

use crate::pages::discover::Discover;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="h-screen w-screen grid bg-red-500">
            <Discover />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
