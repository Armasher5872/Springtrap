use super::*;

#[skyline::from_offset(0x3ac560)]
pub unsafe extern "C" fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

#[skyline::from_offset(0x68d530)]
pub fn get_fighter_vtable(id: u32) -> *const usize;

//The common on hit function for weapons
#[skyline::from_offset(0x33bdf70)]
pub fn normal_weapon_hit_handler(vtable: u64, weapon: *mut smash::app::Weapon, collision_bitmask: u32) -> u64;

#[skyline::from_offset(0x33bed40)]
pub fn get_weapon_vtable(id: u32) -> *const usize;