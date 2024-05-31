// Consists of the PlayerEnt struct which is used to store player data
#[derive(Debug)]
pub struct PlayerEnt{
    pub base: u32,
    pub y: u32,
    pub x: u32,     // Stores the addresses of the values, not the values themselves
    pub z: u32,
    pub yaw: u32,
    pub pitch: u32,
    pub health: u32,
    pub armor: u32,
    // Start addresses of two important classes. They're nested in the player ent and should only be calculated once.
    pub current_weapon: CurrentWeapon,
}

#[derive(Debug)]
pub struct CurrentWeapon{
    pub base: u32,
    pub weapon_stats: u32,
    pub weapon_instance: u32,
    pub ammo: u32,
    pub reserve_ammo: u32,

}


// Most efficient way to modify would be the entire struct at once rather than multiple API calls
// Relevant data is within 0x250 bytes from the base address. Make it a nice round 0x200 for 512 bytes of data.

/*
Important offsets to know:

// Positioning
0x28 - X Coordinates
0x2C - Y Coordinates
0x30 - Z Coordinates
0x34 - Yaw
0x38 - Pitch

// Player Stats
0xEC - Health
0xF0 - Armor
0x11C - Reserve Ammo
0x140 - Mag Ammo
0x12C - Secondary Ammo
0x205 - Player Name

Current Weapon Pointer
[[ac_client.exe + 18AC00] + 374]

Current Ammo Pointer:
----Player Entity----------curWweap-Weapon--Ammo
V ----------------------- v ---- v --- v --- v
[[[[ac_client.exe + 18AC00] + 374] + 10] + 24]

Current Weapon Reserve Ammo Pointer:
[[[[ac_client.exe + 18AC00] + 374] + 10] + 0]

Recoil Value Pointer
[[[[ac_client.exe + 0x18AC00] + 0x374] + 0xC] + 0x60]


// Should instantiate a struct that gets and modifies that stats of the weapon.
These values may be stored statically in memory, rather than dynamically generated (I.e. the way ammo is)
May need to periodically recheck it when you change weapons. 

*/