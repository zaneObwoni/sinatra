use yew::prelude::*;

#[function_component]
pub fn Discover() -> Html {
    let new_number = String::from("seven is the new 8");

    html! {
        <div class="w-screen grid text-black">
            <div class="text-orange-500">{"asdfa"}</div>
            <div class="text-green-500">{new_number}</div>
        </div>
    }
}
