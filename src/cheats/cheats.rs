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
    pub faster_fire_rate: bool,
}

impl Cheats {
    pub fn new() -> Self {
        Self {
            infinite_ammo: false,
            infinite_health: false,
            no_recoil: false,
            no_spread: false,
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

    // Write patch to negate damage function
    pub fn toggle_ininite_health(&self, handle: HANDLE, infinite_health: bool, player: &PlayerEnt) {
        if infinite_health {
            write_memory(handle, &[0x90;3], 0x41c223);
        } else {
            write_memory(handle, &[0x29, 0x73, 0x04], 0x41c223);
        }
        sleep(duration::from_millis(100));
    }

    pub fn disabled_recoil(&self, handle: HANDLE, player: &PlayerEnt, disable_recoil: bool) {
        let recoil_pointer = read_memory(handle, player.current_weapon.weapon_stats) + 0x60;
        if disable_recoil {
            write_memory(handle, &[0x00], recoil_pointer);
        } else {
            write_memory(handle, &[0x50], recoil_pointer);
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
