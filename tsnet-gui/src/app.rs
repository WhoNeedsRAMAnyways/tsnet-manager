
use crate::shell;

pub struct TSNetApp {
    status: String,
    profiles: Vec<shell::Profile>,
}

impl Default for TSNetApp {
    fn default() -> Self {
        Self {
            status: "Ready".to_owned(),
            profiles: Vec::new(),
        }
    }
}

impl TSNetApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        app.refresh();

        app
    }

    fn refresh(&mut self){
        match shell::list_profiles() {
            Ok(text) => { self.status = "Refreshed".to_owned(); self.profiles = shell::parse_profiles(&text); },
            Err(e) => { self.status = e; },
        }
    }
}

impl eframe::App for TSNetApp {

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.heading("Tailscale Networks");
            if ui.button("Refresh").clicked() {
                self.refresh();
            }
        });

        ui.separator();

        // TODO: Switch button on inactive profiles

        // Parse through profiles, indicate active
        for p in &self.profiles {

            ui.horizontal(|ui| {

                // reserve space for active indicator
                let (rect, _response) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());

                // colorize the dot on active
                let color = if p.active {
                    egui::Color32::from_rgb(40, 200, 80)
                } else {
                    egui::Color32::GRAY
                };

                ui.painter().circle_filled(rect.center(), 5.0, color);


                ui.label(format!("ID:{}     Account: {}", p.id, p.account));

            });
        }



        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.label(&self.status);
            ui.separator();
        });
    }
}
