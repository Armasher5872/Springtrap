use super::*;

unsafe extern "C" fn springtrap_axe_fly_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_stick_x = ControlModule::get_stick_x(owner_boma);
    let owner_stick_y = ControlModule::get_stick_y(owner_boma);
    let is_charged = WorkModule::is_flag(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    let life = WorkModule::get_param_int(boma, hash40("param_axe"), hash40("life"));
    let speed_x_min = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("speed_x_min"));
    let speed_x_max = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("speed_x_max"));
    let speed_y_min = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("speed_y_min"));
    let speed_y_max = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("speed_y_max"));
    let brake_x_min = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("brake_x_min"));
    let brake_x_max = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("brake_x_max"));
    let gravity_min = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("gravity_min"));
    let gravity_max = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("gravity_max"));
    let stick = weapon.Vector2__create(owner_stick_x.into(), owner_stick_y.into());
    let normalize = weapon.Vector2__normalize(stick);
    let vec_stick_x = normalize["x"].get_f32();
    let vec_stick_y = normalize["y"].get_f32();
    let stick_angle = vec_stick_y.atan2(vec_stick_x);
    let stick_degrees = stick_angle.to_degrees().clamp(30.0, 70.0);
    let x_speed = if is_charged {speed_x_max} else {speed_x_min};
    let y_speed = if is_charged {speed_y_max} else {speed_y_min};
    let brake = if is_charged {-brake_x_max} else {-brake_x_min};
    let gravity = if is_charged {gravity_max} else {gravity_min};
    let speed_x = stick_degrees.to_radians().cos()*x_speed*owner_lr;
    let speed_y = stick_degrees.to_radians().sin()*y_speed;
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, brake*owner_lr, -gravity);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    ModelModule::set_scale(boma, 0.73);
    PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(12.0*owner_lr), y: owner_pos_y+18.0, z: owner_pos_z});
    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_fly_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    if frame > 1.0 {
        if GroundModule::is_floor_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN as u32) {
            WorkModule::on_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_GROUNDED);
            weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK.into(), false.into());
        }
        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT as u32) {
            WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_GROUNDED);
            weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK.into(), false.into());
        }
        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT as u32) {
            WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_GROUNDED);
            weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK.into(), false.into());
        }
    }
    if should_remove_axe(weapon) {
        remove_axe(weapon);
    }
    if owner_status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
        weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let is_charged = WorkModule::is_flag(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    let gravity_min = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("gravity_min"));
    let gravity_max = WorkModule::get_param_float(boma, hash40("param_axe"), hash40("gravity_max"));
    let gravity = if is_charged {gravity_max} else {gravity_min};
    let get_sum_speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if get_sum_speed_x < 0.0 {
        if get_sum_speed_x > -0.1 {
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0);
        }
    }
    if get_sum_speed_x > 0.0 {
        if get_sum_speed_x < 0.1 {
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0);
        }
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_exit_status)
    .install()
    ;
}