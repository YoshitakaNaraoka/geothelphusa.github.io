use std::fs;
use regex::Regex;

fn remove_integrity_attribute(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let re = Regex::new(r#" integrity="[^"]*""#)?;
    let new_content = re.replace_all(&content, "");
    fs::write(file_path, new_content.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html_files = fs::read_dir("dist")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "html"));

    for file in html_files {
        remove_integrity_attribute(file.path().to_str().unwrap())?;
    }

    Ok(())
}