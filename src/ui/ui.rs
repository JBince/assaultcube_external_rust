use eframe::egui;
use crate::playerent::PlayerEnt;
use crate::utils::*;
use crate::cheats::Cheats;
use winapi::um::winnt::HANDLE;

pub fn draw_ui(handle: HANDLE, mut cheats: Cheats, player: PlayerEnt) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Assault Cube External v1.0", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Assault Cube External v1.0");
            // Refill ammo
            if
                ui
                    .button("Refill Ammo")
                    .on_hover_text("Refills the current weapon's magazine")
                    .clicked()
            {
                cheats.refill_ammo(handle, &player)
            }
            // Infinite ammo patch
            if ui.checkbox(&mut cheats.infinite_ammo, "Infinite Ammo").clicked() {
                println!("Infinite Ammo Toggled: {}" ,cheats.infinite_ammo);
                cheats.toggle_infinite_ammo(handle, cheats.infinite_ammo);
            }
            // Infinite health loop
            if ui.checkbox(&mut cheats.infinite_health, "Infinite Health").clicked() {
                println!("Infinite Health Toggled: {}", cheats.infinite_health);
                cheats.toggle_ininite_health(handle, cheats.infinite_health, &player);
            }
        });
    })
}
