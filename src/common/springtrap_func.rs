#![allow(improper_ctypes_definitions)]
use super::*;

pub unsafe extern "C" fn is_springtrap_slots(boma: *mut BattleObjectModuleAccessor) -> bool {
    MARKED_COLORS[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize]
}

pub unsafe extern "C" fn is_glitchtrap_slots(boma: *mut BattleObjectModuleAccessor) -> bool {
    GLITCHTRAP_COLORS[WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as usize]
}

pub unsafe extern "C" fn should_remove_axe(weapon: &mut L2CWeaponCommon) -> bool {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let dead_range = dead_range(weapon.lua_state_agent);
    let remove_range = pos_x < dead_range.x || pos_x > dead_range.y || pos_y > dead_range.z || pos_y < dead_range.w;
    let object_id = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
    if life <= 0 || remove_range {
        return true;
    }
    if WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED) && object_id == *BATTLE_OBJECT_ID_INVALID {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn should_remove_phantom(weapon: &mut L2CWeaponCommon) -> bool {
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

pub unsafe extern "C" fn phantom_disappear(weapon: &mut L2CWeaponCommon, do_explode: bool, bone: u64) {
    let boma = weapon.module_accessor;
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::unable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    AttackModule::clear_all(boma);
    search!(weapon, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
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
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_foxy_body"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_foxy_endo"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_foxy_eye"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_body2"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_body"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_endo2"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_endo"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_eye"), false);
    ModelModule::set_mesh_visibility(boma, Hash40::new("p_freddy_teeth"), false);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    WorkModule::set_int(boma, 0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    if do_explode {
        EFFECT_FOLLOW(weapon, Hash40::new("springtrap_phantom_detonate"), Hash40::new_raw(bone), 0, 0.0, 0.0, 0, 90, 0, 0.05, true);
        EFFECT_FOLLOW(weapon, Hash40::new("springtrap_phantom_detonate_shock"), Hash40::new_raw(bone), 0, 0.0, 0.0, 0, 90, 0, 0.5, true);
    }
    else {
        EFFECT_FOLLOW(weapon, Hash40::new("springtrap_phantom_summon"), Hash40::new_raw(bone), 0, 6.0, 0.0, 0, 90, 0, 0.5, true);
    }
}

pub unsafe extern "C" fn remove_axe(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    if WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED) {
        LinkModule::remove_model_constraint(boma, true);
        if LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
            LinkModule::unlink(boma, *WEAPON_LINK_NO_CONSTRAINT);
        }
    }
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
    WorkModule::on_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_CAN_LINK);
    WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED);
    WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_GROUNDED);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
    WorkModule::off_flag(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub unsafe extern "C" fn remove_phantom(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    WorkModule::on_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_CAN_EXPLODE);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    WorkModule::set_int(boma, 0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    WorkModule::off_flag(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_PHANTOM);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}

pub unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_PHANTOM) {
        return 0.into();
    }
    1.into()
}