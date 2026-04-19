use super::*;

unsafe extern "C" fn springtrap_axe_freddy_walk_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_freddy_walk_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let prev_status_kind = weapon.global_table[PREV_STATUS_KIND].get_i32();
    let current_x_speed = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let owner_init_lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    if prev_status_kind != *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_TURN {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*owner_init_lr, 0.0);
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, current_x_speed*10.0, 0.0);
    }
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_freddy_walk_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ReflectorModule::set_status(weapon.module_accessor, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_AXE_SHIELD_KIND_BALLOON_BOY_BODY);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("freddy_walk"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_freddy_walk_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_freddy_walk_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_axe(weapon) {
        remove_phantom(weapon);
    }
    if GroundModule::is_ottotto(weapon.module_accessor, 1.5) {
        weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_TURN.into(), false.into());
    }
    if life == 40 {
        phantom_disappear(weapon);
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("freddy_walk"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_freddy_walk_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_freddy_walk_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_axe_freddy_walk_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK, springtrap_axe_freddy_walk_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK, springtrap_axe_freddy_walk_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK, springtrap_axe_freddy_walk_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK, springtrap_axe_freddy_walk_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK, springtrap_axe_freddy_walk_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK, springtrap_axe_freddy_walk_exit_status)
    .install()
    ;
}