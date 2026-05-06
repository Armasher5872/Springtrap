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