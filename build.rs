use horrorshow::helper::doctype;
use horrorshow::html;
use std::fs::{File,create_dir_all};
use std::io::Write;

fn main() {
  create_dir_all("dist").expect("Could not create 'dist' directory.");
  let html = html! {
    : doctype::HTML;
    html(lang="ja", data-theme="dim") {
        head {
            meta(name="viewport", content="width=device-width, initial-scale=1.0");
            title : "Geothelphusa.github.io"; 
        }
        body(class="flex flex-col min-h-screen") {
          div(id="app")
        }
    }
  };

  let mut index_html = File::create("dist/index.html").expect("Could not create `index.html`.");
  writeln!(index_html, "{html}").expect("Could not write to `index.html`.")
}