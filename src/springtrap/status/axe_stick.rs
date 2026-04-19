use super::*;

unsafe extern "C" fn springtrap_axe_stick_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_stick_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    WorkModule::set_int(weapon.module_accessor, 900, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_stick_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stick"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_stick_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_stick_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-2.0, z: pos_z});
    if should_remove_axe(weapon) {
        remove_axe(weapon);
    }
    if MotionModule::is_end(weapon.module_accessor) {
        if [hash40("stick"), hash40("stuck")].contains(&motion_kind) {
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("stuck"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_stick_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_stick_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_axe_stick_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_exit_status)
    .install()
    ;
}