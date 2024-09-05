use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use tokio;
use std::time::{Duration, Instant};

use crate::file_system::traverse_directory;
use crate::markdown::generate_markdown;

pub struct TreeMapMD {
    pub selected_path: Option<PathBuf>,
    pub excluded_dirs: Vec<PathBuf>,
    pub markdown_preview: Arc<Mutex<String>>,
    pub is_processing: Arc<Mutex<bool>>,
    pub error_message: Arc<Mutex<Option<String>>>,
}

impl Default for TreeMapMD {
    fn default() -> Self {
        Self {
            selected_path: None,
            excluded_dirs: Vec::new(),
            markdown_preview: Arc::new(Mutex::new(String::new())),
            is_processing: Arc::new(Mutex::new(false)),
            error_message: Arc::new(Mutex::new(None)),
        }
    }
}

impl TreeMapMD {
pub fn update_markdown_preview(&self) {
    if let Some(path) = &self.selected_path {
        let excluded_dirs = self.excluded_dirs.clone();
        *self.is_processing.lock() = true;
        let path_clone = path.clone();
        let markdown_preview = Arc::clone(&self.markdown_preview);
        let error_message = Arc::clone(&self.error_message);
        let is_processing1 = Arc::clone(&self.is_processing);
        let is_processing2 = Arc::clone(&self.is_processing);

        tokio::spawn(async move {
            let path_clone_inner = path_clone.clone(); // Clone path_clone for use in the inner closure
            let result = tokio::task::spawn_blocking(move || {
                traverse_directory(&path_clone_inner, &excluded_dirs)
            }).await;

            match result {
                Ok(Ok(entries)) => {
                    let markdown = generate_markdown(&entries, &path_clone);
                    *markdown_preview.lock() = markdown;
                }
                Ok(Err(e)) => {
                    *error_message.lock() = Some(format!("Error processing directory: {}", e));
                }
                Err(e) => {
                    *error_message.lock() = Some(format!("Task join error: {}", e));
                }
            }

            *is_processing1.lock() = false;
        });
        // Set a timeout to check for completion
        let start_time = Instant::now();
        let timeout = Duration::from_secs(30); // 30 seconds timeout

        let error_message = Arc::clone(&self.error_message);

        tokio::spawn(async move {
            while start_time.elapsed() < timeout {
                if !*is_processing2.lock() {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            if *is_processing2.lock() {
                *is_processing2.lock() = false;
                *error_message.lock() = Some("Operation timed out".to_string());
            }
        });
    }
}

    pub fn add_excluded_directory(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.excluded_dirs.push(path);
            self.update_markdown_preview();
        }
    }

pub fn export_markdown(&self) -> Result<String, String> {
    println!("Starting export...");
    if let Some(path) = &self.selected_path {
        println!("Selected path: {:?}", path);
        let output_path = path.join("sourceTree.md");
        println!("Output path: {:?}", output_path);
        let markdown_content = self.markdown_preview.lock().clone();
        println!("Markdown content length: {}", markdown_content.len());
        match std::fs::write(&output_path, markdown_content) {
            Ok(_) => {
                println!("Successfully exported to: {}", output_path.display());
                Ok(format!("Exported to: {}", output_path.display()))
            },
            Err(e) => {
                println!("Failed to write file: {}", e);
                Err(format!("Failed to write file: {}", e))
            },
        }
    } else {
        println!("No directory selected");
        Err("No directory selected".to_string())
    }
}}