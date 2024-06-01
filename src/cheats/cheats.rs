use std::thread;
use std::thread::{sleep, Thread};
use std::time::Duration as duration;

use crate::playerent::PlayerEnt;
use crate::utils::*;
use winapi::um::winnt::HANDLE;

pub struct Cheats {
    // TODO: infinite armor, infinite grenades, faster fire rate, no spread.
    pub infinite_ammo: bool,
    pub infinite_health: bool,
    pub no_recoil: bool,
    pub no_spread: bool,
    pub infinite_armor: bool,
    pub infinite_grenades: bool,
    pub faster_fire_rate: bool,
}

impl Cheats {
    pub fn new() -> Self {
        Self {
            infinite_ammo: false,
            infinite_health: false,
            no_recoil: false,
            no_spread: false,
            infinite_armor: false,
            infinite_grenades: false,
            faster_fire_rate: false,
        }
    }

    pub fn toggle_infinite_ammo(&self, handle: HANDLE, infinite_ammo: bool) {
        if infinite_ammo {
            write_memory(handle, &[0x90; 2], 0x4C73EF);
        } else {
            write_memory(handle, &[0xFF, 0x08], 0x4C73EF);
        }
    }

    // It starts a loop that modifies that specific players health, rather than patching the damange function. This works
    // in single and multiplayer. This obviously will need a new thread to do this, and work with other cheats, otherwise it cannot be disabled.
    // That thread will need to check if infinite_health is false, and then break the loop and exit.
    pub fn toggle_ininite_health(&self, handle: HANDLE, infinite_health: bool, player: &PlayerEnt) {
        thread::spawn(move || loop {
            if infinite_health {
                println!("Infinite health is enabled!")
            } else {
                print!("Infinite health has been disabled");
                break;
            }
            sleep(duration::from_millis(100));
        });
    }

    pub fn disabled_recoil(&self, handle: HANDLE, player: &PlayerEnt, disable_recoil: bool) {
        let recoil_pointer = read_memory(handle, player.current_weapon.weapon_stats) + 0x60;
        if disable_recoil {
            write_memory(handle, &[0x00], recoil_pointer);
        } else {
            write_memory(handle, &[0x50], recoil_pointer);
        }
    }

    pub fn infinite_grenades(&self, handle: HANDLE, infinite_grenades: bool) {
        if infinite_grenades {
            write_memory(handle, &[0x90; 2], 0x4C73EF);
        } else {
            write_memory(handle, &[0xFF, 0x08], 0x4C73EF);
        }
    }

    pub fn disabled_spread(&self, handle: HANDLE, player: &PlayerEnt, no_spread: bool) {
        if no_spread {
            write_memory(handle, &[0x00], 0x4C730E);
        } else {
            write_memory(handle, &[0xff,0x46,0x1C], 0x4C730E);
        }
    }

    pub fn faster_fire_rate(&self, handle: HANDLE, faster_fire_rate: bool) {
        if faster_fire_rate {
            write_memory(handle, &[0x90; 2], 0x4C73EA);
        } else {
            write_memory(handle, &[0x89, 0x08], 0x4C73EA);
        }
    }

}
