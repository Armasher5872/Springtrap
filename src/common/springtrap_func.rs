#![allow(improper_ctypes_definitions)]
use super::*;

pub unsafe extern "C" fn is_springtrap_slots(boma: *mut BattleObjectModuleAccessor) -> bool {
    let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    (16..=23).contains(&color)
}

pub unsafe extern "C" fn should_stop_recall(fighter: &mut L2CFighterCommon, start_axe_homing: bool) -> bool {
    let boma = fighter.module_accessor;
    if ArticleModule::is_exist(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE) {
        let active_axe_count = ArticleModule::get_active_num(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE);
        for idx in 0..active_axe_count {
            let axe_battle_object_id = WorkModule::get_int(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_ACTIVE_AXE_BATTLE_OBJECT_ID);
            if sv_battle_object::is_active(axe_battle_object_id as u32) {
                let axe_article = get_article_from_no(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE, idx as i32);
                let axe_battle_object_id = smash::app::lua_bind::Article::get_battle_object_id(axe_article) as u32;
                if axe_battle_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                    if start_axe_homing {
                        let axe_boma = sv_battle_object::module_accessor(axe_battle_object_id);
                        let axe_status_kind = StatusModule::status_kind(axe_boma);
                        if [*WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK].contains(&axe_status_kind) {
                            StatusModule::change_status_request_from_script(axe_boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL, false);
                            return false;
                        }
                        else {
                            return true;
                        }
                    }
                    else {
                        return false;
                    }
                }
                else {
                    return true;
                }
            }
            else {
                return true;
            }
        }
    }
    true
}

pub unsafe extern "C" fn should_remove_axe(weapon: &mut L2CWeaponCommon) -> bool {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let dead_range = dead_range(weapon.lua_state_agent);
    let remove_range = pos_x < dead_range.x || pos_x > dead_range.y || pos_y > dead_range.z || pos_y < dead_range.w;
    if life <= 0 || remove_range {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn phantom_disappear(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::unable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    AttackModule::clear_all(boma);
    ModelModule::set_mesh_visibility(boma, Hash40::new("axe"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_bb_body"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_bb_eye"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_chica_beakfoot"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_chica_bib"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_chica_body"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_chica_endo2"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_chica_endo"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_chica_eye"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_chica_teeth"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_chica_tongue"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_body2"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_body"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_endo2"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_endo"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_eye"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_teeth"), false);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    WorkModule::set_int(boma, 0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    EFFECT_FOLLOW(weapon, Hash40::new("springtrap_phantom_summon"), Hash40::new("top"), 0, 12.0, 0.0, 0, 90, 0, 0.5, true);
}

pub unsafe extern "C" fn remove_axe(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
    WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED);
    WorkModule::set_int(owner_boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_ACTIVE_AXE_BATTLE_OBJECT_ID);
    WorkModule::off_flag(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}

pub unsafe extern "C" fn remove_phantom(weapon: &mut L2CWeaponCommon) {
    let owner_boma = get_owner_boma(weapon);
    WorkModule::set_int(owner_boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_ACTIVE_AXE_BATTLE_OBJECT_ID);
    WorkModule::off_flag(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_PHANTOM);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}

pub unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_PHANTOM) {
        return 0.into();
    }
    1.into()
}