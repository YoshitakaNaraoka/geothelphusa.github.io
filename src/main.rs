mod styles;

use stylist::yew::styled_component;
use yew::prelude::*;
use crate::styles::*;

#[styled_component(App)]
fn app() -> Html {
const LOGO_PATH: &str = "https://raw.githubusercontent.com/Geothelphusa/geothelphusa.github.io/refs/heads/main/static/Geothelphusa.jpeg";
    let logo_path = LOGO_PATH;

    // モードの状態を保持する変数(初期値はライトモード)
    let dark_mode = use_state(|| false);

    let toggle_light: Callback<MouseEvent> = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // ライトモードに設定
            dark_mode.set(false);
        })
    };
    let toggle_dark: Callback<MouseEvent> = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            dark_mode.set(true);
        })
    };

    let mut main_classes = Classes::new();
    main_classes.push(container_styles());
    if *dark_mode {
        main_classes.push(dark_mode_styles());
    } else {
        main_classes.push(light_mode_styles());
    };

    html! {
        <global class={classes!(base_styles())}>
            <main class={main_classes}>
                <h1>{"Welcome to Geothelphusa site !"}</h1>

                <div class={classes!(center_styles())}>
                <a href="https://github.com/Geothelphusa">
                    <img class={classes!(title_logo())} src={logo_path}/>
                </a>
                </div>

                <div class={classes!(center_styles())}>
                <img  class= {classes!(title_logo())} src="https://raw.githubusercontent.com/Geothelphusa/geothelphusa.github.io/refs/heads/main/static/Geothelphusa.jpeg"/>
                </div>

                <div class={classes!(center_styles())}>
                <button class={classes!(input_and_button())} type="submit" onclick={toggle_light}>{"Toggle Light Mode"}</button>
                <button class={classes!(input_and_button())} type="submit" onclick={toggle_dark}>{"Toggle Dark Mode"}</button>               </div>
                <p>{if *dark_mode {"Dark Mode"} else {"Light Mode"}}</p>
            </main>
        </global>
        
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}