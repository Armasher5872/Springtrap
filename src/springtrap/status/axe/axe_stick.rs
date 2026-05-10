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
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_CAN_LINK);
    WorkModule::set_int(boma, 900, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_stick_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let lr = PostureModule::lr(boma);
    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let down_touch_normal = GroundModule::get_touch_normal(boma, *GROUND_TOUCH_FLAG_DOWN as u32);
        let atan = down_touch_normal.x.atan2(down_touch_normal.y);
        let down_touch_angle = atan.to_degrees();
        println!("Down Touch Angle: {}", down_touch_angle);
        WorkModule::set_float(boma, down_touch_angle, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
    }
    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_LEFT as u32) {
        let left_touch_normal = GroundModule::get_touch_normal(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
        let atan = left_touch_normal.x.atan2(left_touch_normal.y)*lr;
        let left_touch_angle = atan.to_degrees();
        println!("Left Touch Angle: {}", left_touch_angle);
        WorkModule::set_float(boma, left_touch_angle, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
    }
    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        let right_touch_normal = GroundModule::get_touch_normal(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
        let atan = right_touch_normal.x.atan2(right_touch_normal.y)*lr;
        let right_touch_angle = atan.to_degrees();
        println!("Right Touch Angle: {}", right_touch_angle);
        WorkModule::set_float(boma, right_touch_angle, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
    }
    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
    MotionModule::change_motion(boma, Hash40::new("stick"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_stick_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_stick_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let slope_rot_angle = WorkModule::get_float(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_SLOPE_ROT_ANGLE);
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    PostureModule::set_rot(boma, &Vector3f{x: slope_rot_angle, y: 0.0, z: 0.0}, 0);
    if should_remove_axe(weapon) {
        remove_axe(weapon);
    }
    if owner_status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
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
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK, springtrap_axe_stick_exit_status)
    .install()
    ;
}