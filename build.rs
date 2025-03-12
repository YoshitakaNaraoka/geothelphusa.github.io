use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::{File, create_dir_all};
use std::io::Write;
// use std::path::Path;
// use std::process::Command;

pub fn main() {
    /* generate `index.html` */
    create_dir_all("dist").expect("Could not create `dist` directory.");
    let js_link = "index.js"; // Yewが生成したJavaScriptファイルのパス
    let html = html! {
        : doctype::HTML;
        html(lang="ja", data-theme="dim") {
            head {
                meta(charset="utf-8");
                meta(name="viewport", content="width=device-width, initial-scale=1");
                title : "Geothelphusa.github.io";
                // link(data-trunk, rel="copy-dir", href="assets");
            }
            body(class="flex flex-col min-h-screen") {
                script(src=js_link) {}
            }
        }
    };
    // let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut index_html = File::create("dist/index.html").expect("Could not create `index.html`.");
    // let dest_path = Path::new(&out_dir).join("index.html");
    writeln!(index_html, "{html}").expect("Could not write to `index.html`.");
/*
    Command::new("trunk")
    .arg("build")
    .status()
    .expect("Failed to execute trunk build");
*/
}