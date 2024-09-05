use std::path::PathBuf;
use rayon::prelude::*;

pub fn generate_markdown(entries: &[(PathBuf, bool, u64)], root: &PathBuf) -> String {
    let header = String::from("| Path | Type | Size | Depth |\n|------|------|------|-------|\n");

    let body = entries.par_iter().map(|(path, is_dir, size)| {
        let relative_path = path.strip_prefix(root).unwrap_or(path).display().to_string();
        let depth = relative_path.matches('/').count() + 1;
        let file_type = if *is_dir { "📁 Folder" } else { "📄 File" };
        let size_str = if *is_dir { "-" } else { &format!("{:.2} KB", *size as f64 / 1024.0) };

        format!("| {} | {} | {} | {} |\n", relative_path, file_type, size_str, depth)
    }).collect::<Vec<String>>().join("");

    header + &body
}