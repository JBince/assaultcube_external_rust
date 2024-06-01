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

    // TODO: Add a custom switch that doesn't look like shit
    eframe::run_simple_native("Assault Cube External v1.0", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Assault Cube External v1.0");
            ui.add_space(10.0);
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
            // Recoil patch
            if ui.checkbox(&mut cheats.no_recoil, "No Recoil").clicked() {
                println!("No Recoil Toggled: {}", cheats.no_recoil);
                cheats.disabled_recoil(handle, &player, cheats.no_recoil);
            }
            // No spread patch
            if ui.checkbox(&mut cheats.no_spread, "No Spread").clicked() {
                println!("No Spread Toggled: {}", cheats.no_spread);
                cheats.disabled_spread(handle, &player, cheats.no_spread);
            }
            // infinite fire rate patch
            if ui.checkbox(&mut cheats.faster_fire_rate, "Maximum fire rate").clicked() {
                print!("Maximum fire rate toggled: {}", cheats.faster_fire_rate);
                cheats.faster_fire_rate(handle, cheats.faster_fire_rate);
            }
        });
    })
}
