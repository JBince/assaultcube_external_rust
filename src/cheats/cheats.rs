use std::thread::sleep;
use std::time::Duration as duration;

use crate::utils::*;
use crate::playerent::PlayerEnt;
use winapi::um::winnt::HANDLE;

pub struct Cheats {
    // To add: infinite health, infinite armor, infinite grenades, faster fire rate, no recoil, no spread.
    pub infinite_ammo: bool,
    pub infinite_health: bool,
}

impl Cheats {
    pub fn new() -> Self {
        Self {
            infinite_ammo: false,
            infinite_health: false,
        }
    }
    
    pub fn refill_ammo(&self, handle: HANDLE, player: &PlayerEnt) {
        write_memory(handle, &[0x64], player.current_weapon.ammo);
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
    pub fn toggle_ininite_health(&self, handle: HANDLE, infinite_health: bool, player: &PlayerEnt) {
        loop {
            if infinite_health {
                write_memory(handle, &[0x64], player.health);
                sleep(duration::from_millis(100));
            } else {
                break;
            }
        }
    }
}