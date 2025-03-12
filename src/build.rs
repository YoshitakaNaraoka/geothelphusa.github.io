use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::{File, create_dir_all};
use std::io::Write;
use yew::prelude::*;
// use std::path::Path;
// use std::process::Command;

#[function_component(App)]
pub fn main() -> Html {
    /* generate `index.html` */
    create_dir_all("dist").expect("Could not create `dist` directory.");
    html! {
        <div>
            <h1>{ "Hello, world!" }</h1>
        </div>
    }
    .into_string()
    .unwrap();

    let mut index_html = File::create("dist/index.html").expect("Could not create `index.html`.");

    writeln!(index_html, "{html}").expect("Could not write to `index.html`.");
    yew::Renderer::<App>::new().render();
}




#[function_component(App)]
fn app() -> Html {
    
}


fn main() {
    
}