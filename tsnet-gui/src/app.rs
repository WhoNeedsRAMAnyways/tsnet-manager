
use crate::shell;

pub struct TSNetApp {
    status: String,
    profiles: String,
}

impl Default for TSNetApp {
    fn default() -> Self {
        Self {
            status: "Ready".to_owned(),
            profiles: String::new(),
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
            Ok(text) => { self.status = "Refreshed".to_owned(); self.profiles = text; },
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

        // TODO: one row per profile, with a Switch button on inactive ones
        ui.label(&self.profiles);

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.label(&self.status);
            ui.separator();
        });
    }
}
