use ac_external_v2::playerent::playerent::CurrentWeapon;
use ac_external_v2::playerent::PlayerEnt;
use ac_external_v2::utils::*;
use eframe::egui;
use std::result::Result;

fn main() -> Result<(), eframe::Error> {
    let handle = get_process_handle(12436);
    let base_address = 0x400000;

    // Get base addresses of important classes. These should not change during the runtime of the program, only the values they contain.
    let local_player_addr = read_memory(handle, base_address + 0x18ac00);
    let current_weapon_pointer = resolve_pointer(handle, vec![0x18ac00, 0x374, 0x10], base_address);
    let current_weapon_addr = read_memory(handle, current_weapon_pointer);

    println!("Local Player Address: {:#X}", local_player_addr);
    println!("Current Weapon Address: {:#X}", current_weapon_addr);

    /*
    Almost all of the values we're working with are pointers that exist within the player entity. We'd generally want to get the root address
    of a given class, and then get the offsets after that. For example, you should only be getting the player root address once, not multiple times.
    You'd then go grab the start address of the current weapon based on that.
    You could call resolve_pointers on each item, but that's not efficient. You'd want to call it once and then store the values in a struct.
    */

    // Calculate the addresses of our interesting values. This is much more efficient than constantly resolving pointers.
    let player = PlayerEnt {
        base: local_player_addr,
        x: local_player_addr + 0x28,
        y: local_player_addr + 0x2c,
        z: local_player_addr + 0x30,
        yaw: local_player_addr + 0x34,
        pitch: local_player_addr + 0x38,
        health: local_player_addr + 0xec,
        armor: local_player_addr + 0xf0,
        current_weapon: CurrentWeapon {
            base: current_weapon_addr,
            ammo: current_weapon_addr + 0x24,

            reserve_ammo: current_weapon_addr + 0x0,
        },
    };

    println!("Current Weapon Ammo: {}", read_memory(handle, player.current_weapon.ammo));

    // Examples

    // Patch ammo decrementing. This is a single applied patch in memory.
    // Decrementing the ammo value is done @ 0x4C73EF. Only a two byte instruction.
    
    // write_memory(handle, &[0x90; 2], 0x4C73EF);

    // Continuously refill the current magazine ammo

    // loop {
    //     write_memory(handle, &[0x64], player.current_weapon.ammo);
    //     sleep(duration::from_millis(100));
    // }

    // Using EGUI to create a simple program that adds additional toggleability to the cheats

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Assault Cube External v1.0", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Assault Cube External v1.0");
            // if ui.button("Increment").clicked() {
            //     age += 1;
            // }
            ui.label(format!("Ammo located @ {:#X}", player.current_weapon.ammo));
            
            // Figure out how to add checkbox to toggle features
        });
    })

}
