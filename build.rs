// use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    // Yew アプリケーションのビルド
    /* let output = Command::new("trunk")
        .args(&["build"])
        .output()
        .expect("failed to execute trunk build");

    if !output.status.success() {
        panic!("trunk build failed: {}", String::from_utf8_lossy(&output.stderr));
    } */

    // 出力ディレクトリの作成
    let out_dir = Path::new("dist");
    fs::create_dir_all(&out_dir).expect("failed to create output directory");

    // 生成された HTML ファイルを dist ディレクトリに移動
    fs::rename("dist/index.html", out_dir.join("index.html")).expect("failed to move index.html");

    // 必要に応じて、他のアセット（CSS、JavaScriptなど）も同様に移動
    // 例: fs::copy("dist/style.css", out_dir.join("style.css")).expect("failed to copy style.css");

    println!("cargo:rustc-rerun-if-changed=static"); // static ディレクトリの変更を監視
}