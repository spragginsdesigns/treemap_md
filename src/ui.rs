use eframe::egui;
use crate::app::TreeMapMD;

impl eframe::App for TreeMapMD {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dark_mode = egui::Visuals::dark();
        ctx.set_visuals(dark_mode);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("TreeMapMD");
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button("Select Directory").clicked() && !*self.is_processing.lock() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.selected_path = Some(path);
                        self.update_markdown_preview();
                    }
                }

                if let Some(path) = &self.selected_path {
                    ui.label(format!("Selected: {}", path.display()));
                }
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("Excluded Directories:");
                let mut dirs_to_remove = Vec::new();
                for (index, excluded_dir) in self.excluded_dirs.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(excluded_dir.to_string_lossy());
                        if ui.small_button("❌").clicked() {
                            dirs_to_remove.push(index);
                        }
                    });
                }
                for index in dirs_to_remove.iter().rev() {
                    self.excluded_dirs.remove(*index);
                }
                if !dirs_to_remove.is_empty() {
                    self.update_markdown_preview();
                }

                if ui.button("Add Excluded Directory").clicked() {
                    self.add_excluded_directory();
                }
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("Markdown Preview:");
                let preview = self.markdown_preview.lock().clone();
                egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                    ui.text_edit_multiline(&mut preview.to_owned());
                });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Generate Preview").clicked() && !*self.is_processing.lock() {
                    self.update_markdown_preview();
                }

                if ui.button("Export").clicked() && !*self.is_processing.lock() {
    match self.export_markdown() {
        Ok(message) => {
            *self.error_message.lock() = Some(message);
        }
        Err(e) => {
            *self.error_message.lock() = Some(e);
        }
    }
    ctx.request_repaint(); // Force a repaint to immediately show the message
}
            });

            if *self.is_processing.lock() {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Processing...");
                });
            }

            if let Some(error) = &*self.error_message.lock() {
                ui.add_space(10.0);
                ui.colored_label(egui::Color32::RED, error);
            }
        });

        ctx.request_repaint();
    }
}