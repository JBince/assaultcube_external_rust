use std::sync::atomic::{ AtomicBool, Ordering };
use std::thread::{ self, JoinHandle };
use std::time::Duration;
use std::sync::{ Arc, Mutex };

use crate::playerent::PlayerEnt;
use crate::utils::*;
use winapi::um::winnt::HANDLE;

pub struct Cheats {
    // TODO: infinite armor, infinite grenades, faster fire rate, no spread.
    pub infinite_ammo: bool,
    pub infinite_health: bool,
    pub infinite_health_status: Arc<AtomicBool>,
    pub infinite_health_handle: Mutex<Option<JoinHandle<()>>>,
    pub no_recoil: bool,
    pub no_spread: bool,
    pub faster_fire_rate: bool,
}

impl Cheats {
    pub fn new() -> Self {
        Self {
            infinite_ammo: false,
            infinite_health: false,
            infinite_health_status: Arc::new(AtomicBool::new(false)),
            infinite_health_handle: Mutex::new(None),
            no_recoil: false,
            no_spread: false,
            faster_fire_rate: false,
        }
    }

    pub fn toggle_infinite_ammo(&self, handle: HANDLE, infinite_ammo: bool) {
        if infinite_ammo {
            write_memory(handle, &[0x90; 2], 0x4c73ef);
        } else {
            write_memory(handle, &[0xff, 0x08], 0x4c73ef);
        }
    }

    pub fn toggle_infinite_health(&self, infinite_health: bool, player: &PlayerEnt, handle: HANDLE) {

        let mut thread_guard = self.infinite_health_handle.lock().unwrap();
        self.infinite_health_status.store(infinite_health, Ordering::Relaxed);
        let health_clone = Arc::clone(&self.infinite_health_status);
        
        // This is an extremely hacky workaround. 
        // You can't send raw pointers to new threads, so I cast it as an integer in an arc, send the arc, then cast it back to a pointer.
        // The cheat is set up this way so a new thread can run the loop, while you can still enable and disable other cheats.
        // Probably a stupid way to do this, but it works ¯\_(ツ)_/¯

        let handle = Arc::new(handle as i32);
        let health = Arc::new(player.health);

        if thread_guard.is_none() {
            let thread = thread::spawn(move || {
                while health_clone.load(Ordering::Relaxed) {
                    let handle = *handle.clone() as HANDLE;
                    write_memory(handle, &[0xFF], *health);
                    thread::sleep(Duration::from_millis(50));
                }
            });
            *thread_guard = Some(thread);
        } else {
            if let Some(thread) = thread_guard.take() {
                self.infinite_health_status.store(false, Ordering::Relaxed);
                thread.join().unwrap();
                println!("Infinite health terminated");
            }
        }
    }

    pub fn disabled_recoil(&self, handle: HANDLE, player: &PlayerEnt) {
        let recoil_pointer = read_memory(handle, player.current_weapon.weapon_stats) + 0x60;
        if self.no_recoil {
            write_memory(handle, &[0x00], recoil_pointer);
        } else {
            // This value changes based on the weapon. It's not a static value, so it's not a good idea to hardcode it. Will fix later
            write_memory(handle, &[0x32], recoil_pointer);
        }
    }

    pub fn disabled_spread(&self, handle: HANDLE) {
        if self.no_spread {
            write_memory(handle, &[0x00], 0x4c730e);
        } else {
            write_memory(handle, &[0xff, 0x46, 0x1c], 0x4c730e);
        }
    }

    pub fn faster_fire_rate(&self, handle: HANDLE) {
        if self.faster_fire_rate {
            write_memory(handle, &[0x90; 2], 0x4c73ea);
        } else {
            write_memory(handle, &[0x89, 0x08], 0x4c73ea);
        }
    }
}