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

use std::fs;
// use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dist_dir = "dist";
    let index_path = format!("{}/index.html", dist_dir);
    
    // Trunk が生成した JS ファイルを探す
    let js_file = fs::read_dir(dist_dir)?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_string_lossy().ends_with(".js"))
        .ok_or("Trunk JS file not found")?
        .path();

    // let new_js_path = Path::new(dist_dir).join("app.js");

    // 既存の app.js を削除（必要なら）
    // if new_js_path.exists() {
    //     fs::remove_file(&new_js_path)?;
    // }

    // Trunk の JS ファイルを app.js にリネーム
    // fs::rename(&js_file, &new_js_path)?;

    // println!("✅ Renamed Trunk's JS to app.js");

    // index.html の読み込み
    let mut content = fs::read_to_string(&index_path)?;

    // `<script>` タグの `src` を `app.js` に変更
    content = content.replace(
        &format!(r#"src="/geothelphusa.github.io/{}""#, js_file.file_name().unwrap().to_string_lossy()),
        r#"src="app.js""#,
    );

    // index.html の更新
    fs::write(&index_path, content)?;
    println!("✅ Updated index.html to reference app.js");

    Ok(())
}

