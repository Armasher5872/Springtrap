use super::*;

unsafe extern "C" fn springtrap_phantom_foxy_attack_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_foxy_attack_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    KineticModule::unable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    WorkModule::set_int(boma, 80, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_foxy_attack_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("foxy_attack"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_foxy_attack_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_foxy_attack_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_phantom(weapon) {
        remove_phantom(weapon);
    }
    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_LEFT as u32) 
    || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        WorkModule::set_int(boma, 40, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    if life == 40 {
        phantom_disappear(weapon, false, 0x9193dceeb);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_foxy_attack_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_foxy_attack_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_phantom_foxy_attack_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FOXY_ATTACK, springtrap_phantom_foxy_attack_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FOXY_ATTACK, springtrap_phantom_foxy_attack_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FOXY_ATTACK, springtrap_phantom_foxy_attack_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FOXY_ATTACK, springtrap_phantom_foxy_attack_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FOXY_ATTACK, springtrap_phantom_foxy_attack_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FOXY_ATTACK, springtrap_phantom_foxy_attack_exit_status)
    .install()
    ;
}