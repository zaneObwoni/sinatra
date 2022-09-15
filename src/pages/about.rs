use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let new_number = "seven is the new 8".to_string();

    html! {
        <div class="w-screen grid text-black">
            <div class="text-orange-500">{"asdfa"}</div>
            <div class="text-green-500">{new_number}</div>
        </div>
    }
}
