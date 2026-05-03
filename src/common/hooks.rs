use super::*;

#[skyline::from_offset(0x3ac560)]
pub unsafe extern "C" fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

#[skyline::from_offset(0x33bd9c0)]
pub unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut smash::app::Weapon, collision_bitmask: u32) -> u64;