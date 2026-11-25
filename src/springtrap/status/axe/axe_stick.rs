use super::*;

unsafe extern "C" fn springtrap_axe_stick_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let is_grounded = WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_GROUNDED);
    let sit_kind = if is_grounded {*SITUATION_KIND_GROUND} else {*SITUATION_KIND_AIR};
    let correct_kind = if is_grounded {*GROUND_CORRECT_KIND_GROUND} else {*GROUND_CORRECT_KIND_AIR};
    StatusModule::init_settings(boma, SituationKind(sit_kind), *WEAPON_KINETIC_TYPE_NORMAL, correct_kind as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_stick_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let normal_x = GroundModule::get_touch_normal_x(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
        let angle = (normal_y/normal_x).atan().to_degrees()-90.0;
        WorkModule::set_float(boma, angle, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
        GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 3.0});
    }
    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_LEFT as u32) {
        let normal_x = GroundModule::get_touch_normal_x(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
        let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
        let angle = (normal_y/normal_x).atan().to_degrees()-90.0;
        WorkModule::set_float(boma, angle, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
        GroundModule::set_rhombus_offset(boma, &Vector2f{x: 3.0, y: 0.0});
    }
    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        let normal_x = GroundModule::get_touch_normal_x(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
        let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
        let angle = (normal_y/normal_x).atan().to_degrees()-90.0;
        WorkModule::set_float(boma, angle, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
        GroundModule::set_rhombus_offset(boma, &Vector2f{x: -3.0, y: 0.0});
    }
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    WorkModule::set_int(boma, 900, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_stick_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
    MotionModule::change_motion(boma, Hash40::new("stick"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_stick_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_stick_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let slope_rot_angle = WorkModule::get_float(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
    let founder_boma = get_founder_boma(weapon);
    let founder_status_kind = StatusModule::status_kind(founder_boma);
    PostureModule::set_rot(boma, &Vector3f{x: slope_rot_angle, y: 0.0, z: 0.0}, 0);
    if should_remove_axe(weapon) {
        remove_axe(weapon);
    }
    if founder_status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
        weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        if [hash40("stick"), hash40("stuck")].contains(&motion_kind) {
            MotionModule::change_motion(boma, Hash40::new("stuck"), 0.0, 1.0, false, 0.0, false, false);
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
    .set_costume(get_springtrap_costumes_acmd())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_exit_status)
    .install()
    ;
}