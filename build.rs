use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::File;
use std::io::Write;

pub fn main() {
    /* generate `index.html` */
    let html = html! {
        : doctype::HTML;
        html(lang="ja", data-theme="dim") {
            head {
                meta(charset="utf-8");
                meta(name="viewport", content="width=device-width, initial-scale=1");
                title : "Geothelphusa.github.io";
                // link(data-trunk, rel="copy-dir", href="assets");
            }
            body(class="flex flex-col min-h-screen");
        }
    };
    let mut index_html = File::create("index.html").expect("Could not create `index.html`.");
    writeln!(index_html, "{html}").expect("Could not write to `index.html`.");
}