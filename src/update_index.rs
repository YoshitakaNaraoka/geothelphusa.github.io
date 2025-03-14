// use std::fs;
// use regex::Regex;

// fn main() {
//     // dist フォルダから `.js` ファイルを探す
//     let js_file = fs::read_dir("dist")
//         .unwrap()
//         .filter_map(|entry| entry.ok())
//         .find(|entry| entry.file_name().to_string_lossy().ends_with(".js"))
//         .expect("JavaScript file not found in dist directory");

//     // JavaScript のファイル名
//     let js_filename = js_file.file_name().to_string_lossy().into_owned();

//     // `index.html` を読み込む
//     let html_file = "dist/index.html";
//     let content = fs::read_to_string(html_file).expect("Could not read `index.html`");

//     // `<script src="古いJSファイル名"></script>` を更新
//     let re = Regex::new(r#"<script src=".*\.js"></script>"#).unwrap();
//     let new_script_tag = format!(r#"<script src="{}"></script>"#, js_filename);
//     let updated_content = re.replace_all(&content, new_script_tag);

//     // `index.html` を更新
//     fs::write(html_file, updated_content.as_bytes()).expect("Could not write to `index.html`");
// }
