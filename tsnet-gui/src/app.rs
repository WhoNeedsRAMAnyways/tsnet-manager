
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
            // Margin pad
            ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());

            ui.heading("Tailscale Networks");
            if ui.button("Refresh").clicked() {
                self.refresh();
            }
        });

        ui.separator();

        // Parse through profiles, indicate active
       
        let mut to_switch: Option<String> = None; 
        for p in &self.profiles {

            ui.horizontal(|ui| {
                // Margin Pad
                ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());

                // reserve space for active indicator
                let (rect, _response) = 
                    ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                let c = rect.center();

                // cPulse + Glow
                if p.active {
                    let t = ui.input(|i| i.time) as f32;
                    let pulse = (t * 2.0).sin() * 0.5 + 0.5;

                    let green = |a: u8| egui::Color32::from_rgba_unmultiplied(40, 200, 80, a);
                    ui.painter().circle_filled(c, 7.0 + 2.0 * pulse, green((20.0 + 50.0 * pulse ) as u8));
                    ui.painter().circle_filled(c, 7.0, green(60));

                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(33));
                }
                
                // Center Dot Color
                let core = if p.active {
                    egui::Color32::from_rgb(40, 200, 80)
                } else {
                    egui::Color32::GRAY
                };

                ui.painter().circle_filled(rect.center(), 5.0, core);


                ui.label(format!("ID:{}     Account: {}", p.id, p.account));

                // Account Switching 
                if !p.active && ui.button("Switch").clicked() {
                    to_switch = Some(p.id.clone());
                }

            });
        }

        // Button Logic for the account switcher button
        if let Some(id) = to_switch {
            match shell::switch_profile(&id) {
                Ok(_) => {
                    self.refresh();
                    self.status = format!("Switched to {}", id);
                }
                Err(e) => {
                    self.status = e;
                }
            }
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.label(&self.status);
            ui.separator();
        });
    }
}
