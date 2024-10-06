use yew::prelude::*;
const PRIMARY_COLOR: &str = "emerald-900";
const SECONDARY_COLOR: &str = "amber-200";
const ACCENT_COLOR: &str = "zinc-900";
#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let btn_classes = String::from("bg-zinc-900 hover:bg-amber-600 text-amber-200 font-bold py-2 px-4");

    html!{
        <>
            <h2 class={classes!("bg-zinc-900", "text-amber-200", "text-4xl", "text-center", "p-5")}>{"Antoine Moevus"}</h2>
            <br/>
            <button {onclick}  class={classes!(btn_classes)}>{ " +1 " }</button>
            <p class={classes!("text-amber-100", "text-xl", "text-center")}>{ *counter }</p>
        <br/>
            <p class={classes!("bg-amber-900", "text-amber-100", "text-4xl", "text-center")}>{"Work in progress"}</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}