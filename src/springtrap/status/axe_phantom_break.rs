use super::*;

unsafe extern "C" fn springtrap_axe_phantom_break_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_break_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 60, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_break_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let phantom_type = WorkModule::get_int(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("chica_die"), 0.0, 1.0, false, 0.0, false, false);   
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("freddy_die"), 0.0, 1.0, false, 0.0, false, false);   
    }
    //ReflectorModule::set_status(weapon.module_accessor, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NONE), *WEAPON_SPRINGTRAP_AXE_SHIELD_KIND_BALLOON_BOY_BODY);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_phantom_break_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_phantom_break_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_axe(weapon) {
        remove_phantom(weapon);
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_break_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_break_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_phantom_break_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    0.into()
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_BREAK, springtrap_axe_phantom_break_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_BREAK, springtrap_axe_phantom_break_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_BREAK, springtrap_axe_phantom_break_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_BREAK, springtrap_axe_phantom_break_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_BREAK, springtrap_axe_phantom_break_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_BREAK, springtrap_axe_phantom_break_exit_status)
    .install()
    ;
}