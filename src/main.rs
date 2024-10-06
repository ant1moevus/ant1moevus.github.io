use yew::prelude::*;

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

    let btn_classes = String::from("bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4");

    html!{
        <>
            <button {onclick}  class={classes!(btn_classes)}>{ " +1 " }</button>
            <p>{ *counter }</p>
            <p class={classes!("bg-red-100")}>{"Test!"}</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}