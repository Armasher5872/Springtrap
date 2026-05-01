use super::*;

unsafe extern "C" fn springtrap_axe_fly_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let is_charged = WorkModule::is_flag(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    let life = 240;
    let speed_min = 6.63943f32;
    let speed_max = 6.16197f32;
    let brake_x = 0.03f32;
    let gravity = 0.18f32;
    let angle: f32 = if is_charged {40.0} else {30.0};
    let x_speed: f32 = if is_charged {speed_max} else {speed_min};
    let y_speed: f32 = 5.72958;
    let speed_x = angle.to_radians().sin()*x_speed*owner_lr;
    let speed_y = angle.to_radians().cos()*y_speed;
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, if is_charged{speed_x*0.6} else {speed_x*0.55}, if is_charged{speed_y*0.6} else {speed_y*0.4});
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -brake_x*owner_lr, -gravity);
    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_OFF), 0);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(boma, 0.73);
    PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(12.0*owner_lr), y: owner_pos_y+18.0, z: owner_pos_z});
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_fly_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    if GroundModule::is_floor_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN as u32) && frame > 1.0 {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        WorkModule::on_flag(boma, *WEAPON_KROOL_IRONBALL_INSTANCE_WORK_ID_FLAG_HOP);
        PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-2.0, z: pos_z});
        weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK.into(), false.into());
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
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, springtrap_axe_fly_exit_status)
    .install()
    ;
}