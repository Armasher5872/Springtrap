use super::*;

unsafe extern "C" fn springtrap_phantom_phantom_break_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_break_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 100, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_break_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
        MotionModule::change_motion(boma, Hash40::new("chica_die"), 0.0, 1.0, false, 0.0, false, false);   
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
        MotionModule::change_motion(boma, Hash40::new("freddy_die"), 0.0, 1.0, false, 0.0, false, false);   
    }
    ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_phantom_break_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_phantom_break_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_phantom(weapon) {
        remove_phantom(weapon);
    }
    if life == 40 {
        phantom_disappear(weapon, true, 0x31ed91fca);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_break_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_break_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_int(boma, 0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_break_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    WorkModule::set_int(boma, 0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    0.into()
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_BREAK, springtrap_phantom_phantom_break_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_BREAK, springtrap_phantom_phantom_break_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_BREAK, springtrap_phantom_phantom_break_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_BREAK, springtrap_phantom_phantom_break_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_BREAK, springtrap_phantom_phantom_break_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_BREAK, springtrap_phantom_phantom_break_exit_status)
    .install()
    ;
}