use super::*;

unsafe extern "C" fn springtrap_axe_phantom_summon_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_summon_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    WorkModule::set_float(weapon.module_accessor, owner_lr, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_summon_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let team_no = TeamModule::team_no(owner_boma) as i32;
    let phantom_type = WorkModule::get_int(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    ReflectorModule::set_status(weapon.module_accessor, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_AXE_SHIELD_KIND_BALLOON_BOY_BODY);
    ReflectorModule::set_no_team(weapon.module_accessor, false);
    TeamModule::set_team_owner_id(weapon.module_accessor, (*owner_boma).battle_object_id);
    TeamModule::set_team(weapon.module_accessor, team_no, true);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
        WorkModule::set_int(weapon.module_accessor, 640, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        ReflectorModule::set_size(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_SHIELD_KIND_BALLOON_BOY_BODY, 12.0, 0);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("freddy_summon"), 0.0, 1.0, false, 0.0, false, false);
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
        WorkModule::set_int(weapon.module_accessor, 340, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        ReflectorModule::set_size(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_SHIELD_KIND_BALLOON_BOY_BODY, 12.0, 0);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("chica_summon"), 0.0, 1.0, false, 0.0, false, false);   
    }
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_phantom_summon_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_phantom_summon_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    let phantom_type = WorkModule::get_int(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_axe(weapon) {
        remove_phantom(weapon);
    }
    if MotionModule::is_end(weapon.module_accessor) {
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
            weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK.into(), false.into());
        }
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
            weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_CHICA_WALK.into(), false.into()); 
        }
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_summon_exec_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_summon_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_summon_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_SUMMON, springtrap_axe_phantom_summon_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_SUMMON, springtrap_axe_phantom_summon_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_SUMMON, springtrap_axe_phantom_summon_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_SUMMON, springtrap_axe_phantom_summon_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_SUMMON, springtrap_axe_phantom_summon_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_SUMMON, springtrap_axe_phantom_summon_exit_status)
    .install()
    ;
}