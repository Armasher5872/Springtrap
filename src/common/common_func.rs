use super::*;

pub unsafe extern "C" fn spawn_hit_effects(weapon: &mut L2CWeaponCommon, attr: u64) {
    let boma = weapon.module_accessor;
    let pos = *PostureModule::pos(boma);
    match attr {
        _ if attr == hash40("collision_attr_aura") => {EffectModule::req(boma, Hash40::new("sys_hit_aura_s"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_coin") => {EffectModule::req(boma, Hash40::new("sys_hit_coin"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_curse_poison") => {EffectModule::req(boma, Hash40::new("sys_hit_curse"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_cutup") => {EffectModule::req(boma, Hash40::new("sys_hit_cut"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_elec") => {EffectModule::req(boma, Hash40::new("sys_hit_elec"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_fire") => {EffectModule::req(boma, Hash40::new("sys_hit_fire"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_ice") => {EffectModule::req(boma, Hash40::new("sys_hit_ice"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_magic") => {EffectModule::req(boma, Hash40::new("sys_hit_magic"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_normal_poison") => {EffectModule::req(boma, Hash40::new("sys_hit_poison"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_paralyze") => {EffectModule::req(boma, Hash40::new("sys_damage_paralyze"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_pierce") => {EffectModule::req(boma, Hash40::new("sys_hit_sting"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_purple") => {EffectModule::req(boma, Hash40::new("sys_hit_purple"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_rush") => {EffectModule::req(boma, Hash40::new("sys_hit_rush"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_sting") => {EffectModule::req(boma, Hash40::new("sys_hit_sting"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ if attr == hash40("collision_attr_water") => {EffectModule::req(boma, Hash40::new("sys_hit_sweat"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);},
        _ => {EffectModule::req(boma, Hash40::new("sys_hit_normal_s"), &Vector3f{x: pos.x, y: pos.y, z: pos.z}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);}
    }
}

pub fn get_costumes() -> Vec<usize> {
	let costumes = &mut Vec::new();
    unsafe {
        let marked = *&raw mut MARKED_COLORS;
        for i in 0..marked.len() {
            if marked[i] {
                costumes.push(i);
            }
        }
    }
    costumes.to_vec()
}

pub unsafe extern "C" fn set_front_cliff_hangdata(boma: &mut BattleObjectModuleAccessor, x: f32, y: f32) {
    let ground_module = *(boma as *mut BattleObjectModuleAccessor as *const u64).add(0x58/0x8);
    let ground_data = *((ground_module+0x28) as *mut *mut f32);
    *ground_data.add(0x530/0x4) = x;
    *ground_data.add(0x534/0x4) = y;
}

pub unsafe extern "C" fn set_back_cliff_hangdata(boma: &mut BattleObjectModuleAccessor, x: f32, y: f32) {
    let ground_module = *(boma as *mut BattleObjectModuleAccessor as *const u64).add(0x58/0x8);
    let ground_data = *((ground_module+0x28) as *mut *mut f32);
    *ground_data.add(0x540/0x4) = x;
    *ground_data.add(0x544/0x4) = y;
}

pub unsafe extern "C" fn set_center_cliff_hangdata(boma: &mut BattleObjectModuleAccessor, x: f32, y: f32) {
    let ground_module = *(boma as *mut BattleObjectModuleAccessor as *const u64).add(0x58/0x8);
    let ground_data = *((ground_module+0x28) as *mut *mut f32);
    *ground_data.add(0x520/0x4) = x;
    *ground_data.add(0x524/0x4) = y;
}