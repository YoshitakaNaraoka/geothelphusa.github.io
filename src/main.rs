use yew::prelude::*;
// use yew::ServerRenderer;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello, world!" }</h1>
        </div>
    }
}

// #[tokio::main]
async fn no_main() {
    yew::ServerRenderer::<App>::new().render().await;
}