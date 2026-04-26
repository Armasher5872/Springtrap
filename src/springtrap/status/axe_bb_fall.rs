use super::*;

unsafe extern "C" fn springtrap_axe_bb_fall_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_bb_fall_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    let speed_y = WorkModule::get_float(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.065);
    sv_kinetic_energy!(set_brake, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.04);
    0.into()
}

unsafe extern "C" fn springtrap_axe_bb_fall_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ReflectorModule::set_status(weapon.module_accessor, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_AXE_SHIELD_KIND_BALLOON_BOY_BODY);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("bb_idle"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_bb_fall_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_bb_fall_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if should_remove_axe(weapon) {
        remove_phantom(weapon);
    }
    if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) && frame > 1.0 {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-2.0, z: pos_z});
        weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_IDLE.into(), false.into());
    }
    if life == 40 {
        phantom_disappear(weapon);
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("bb_idle"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_bb_fall_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_bb_fall_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    0.into()
}

unsafe extern "C" fn springtrap_axe_bb_fall_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
    WorkModule::set_float(weapon.module_accessor, 0.0, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    0.into()
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL, springtrap_axe_bb_fall_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL, springtrap_axe_bb_fall_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL, springtrap_axe_bb_fall_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL, springtrap_axe_bb_fall_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL, springtrap_axe_bb_fall_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL, springtrap_axe_bb_fall_exit_status)
    .install()
    ;
}