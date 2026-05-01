use super::*;

#[skyline::from_offset(0x3ac560)]
pub unsafe extern "C" fn get_battle_object_from_id(id: u32) -> *mut BattleObject;