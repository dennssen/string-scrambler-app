/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    label: String,
    scrambled: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            scrambled: "".to_owned()
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("String Scrambler");
            egui::ScrollArea::vertical()
                .id_salt("input scroll")
                .max_height(100.0)
                .enable_scrolling(true)
                .stick_to_bottom(true)
                .show(ui, |ui|{
                    ui.label("Your String: ");
                    ui.text_edit_multiline(&mut self.label);
                });
            
            ui.horizontal(|ui|{
                if ui.button("Scramble!").clicked() {
                    self.scrambled = scramble(self.label.clone());
                    self.label = String::new();
                }
                if ui.button("📋").clicked(){
                    ui.output_mut(|o| o.copied_text = String::from(self.scrambled.clone()))
                }
            });

            egui::ScrollArea::vertical()
                .id_salt("output scroll")
                .max_height(100.0)
                .enable_scrolling(true)
                .stick_to_bottom(true)
                .show(ui, |ui|{
                    ui.label(&self.scrambled);
                });
                
            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn scramble(string: String) -> String{
    use rand::random_range;

    let string_count;
    let mut string_char: Vec<char> = string.chars().collect();
    let mut scrambled_string: String = String::new();

    string_count = string_char.len() as i32;

    for _ in 0..string_count {
        let random_index = random_range(0..string_char.len());
        scrambled_string.push(*string_char.get(random_index).unwrap());
        string_char.remove(random_index);
    }

    scrambled_string
}
