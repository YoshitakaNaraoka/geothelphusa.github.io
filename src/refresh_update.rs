use std::fs;
use std::process::Command;
use regex::Regex;

pub fn update_integrity_attributes() -> Result<(), Box<dyn std::error::Error>> {
    let js_file = fs::read_dir("dist")?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_string_lossy().ends_with(".js"))
        .ok_or("JavaScript file not found")?;

    let wasm_file = fs::read_dir("dist")?
        .filter_map(|entry| entry.ok())
        .find(|entry| entry.file_name().to_string_lossy().ends_with("_bg.wasm"))
        .ok_or("WASM file not found")?;

    let js_hash = Command::new("openssl")
        .args(&["dgst", "-sha384", "-binary", js_file.path().to_str().unwrap()])
        .output()?;
    let js_hash = Command::new("base64")
        .stdin(std::process::Stdio::from(std::io::Cursor::new(js_hash.stdout)))
        .output()?;
    let js_hash = String::from_utf8(js_hash.stdout)?.trim().to_string();

    let wasm_hash = Command::new("openssl")
        .args(&["dgst", "-sha384", "-binary", wasm_file.path().to_str().unwrap()])
        .output()?;
    let wasm_hash = Command::new("base64")
        .stdin(std::process::Stdio::from(std::io::Cursor::new(wasm_hash.stdout)))
        .output()?;
    let wasm_hash = String::from_utf8(wasm_hash.stdout)?.trim().to_string();

    let html_file = "dist/index.html";
    let content = fs::read_to_string(html_file)?;

    let js_re = Regex::new(r#"integrity=".*geothelphusa-github-io-.*\.js""#)?;
    let new_content = js_re.replace_all(&content, format!("integrity=\"sha384-{}\"", js_hash));

    let wasm_re = Regex::new(r#"integrity=".*geothelphusa-github-io-.*_bg\.wasm""#)?;
    let final_content = wasm_re.replace_all(&new_content, format!("integrity=\"sha384-{}\"", wasm_hash));

    fs::write(html_file, final_content.as_bytes())?;
    Ok(())
}
