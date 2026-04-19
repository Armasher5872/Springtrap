use super::*;

unsafe extern "C" fn springtrap_axe_fly_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let is_charged = WorkModule::is_flag(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    let phantom_type = WorkModule::get_int(owner_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_LW_PHANTOM_TYPE);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_axe"), hash40("life"));
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_axe"), hash40("speed_min"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_axe"), hash40("speed_max"));
    let brake_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_axe"), hash40("brake_x"));
    let gravity = WorkModule::get_param_float(weapon.module_accessor, hash40("param_axe"), hash40("gravity"));
    let angle: f32 = if is_charged {40.0} else {30.0};
    let x_speed: f32 = if is_charged {speed_max} else {speed_min};
    let y_speed: f32 = 5.72958;
    let speed_x = angle.to_radians().sin()*x_speed*owner_lr;
    let speed_y = angle.to_radians().cos()*y_speed;
    WorkModule::set_int(owner_boma, weapon.battle_object_id as i32, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_ACTIVE_AXE_BATTLE_OBJECT_ID);
    if owner_status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, if is_charged{speed_x*0.6} else {speed_x*0.55}, if is_charged{speed_y*0.6} else {speed_y*0.4});
        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -brake_x*owner_lr, -gravity);
        KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        WorkModule::set_int(weapon.module_accessor, -1, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
        ModelModule::set_scale(weapon.module_accessor, 0.73);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(12.0*owner_lr), y: owner_pos_y+18.0, z: owner_pos_z});
    }
    else {
        WorkModule::set_int(weapon.module_accessor, phantom_type, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(18.0*owner_lr), y: owner_pos_y+3.0, z: owner_pos_z});
        ModelModule::set_scale(weapon.module_accessor, 1.75);
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
            StatusModule::change_status_force(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_SUMMON, false);
        }
        else if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FOXY {
            StatusModule::change_status_force(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FOXY_ATTACK, false);
        }
        else if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
            StatusModule::change_status_force(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_SUMMON, false);
        }
        else {
            StatusModule::change_status_force(weapon.module_accessor, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_IDLE, false);
        }
    }
    0.into()
}

unsafe extern "C" fn springtrap_axe_fly_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_axe_fly_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_axe_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) && frame > 1.0 {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-2.0, z: pos_z});
        weapon.change_status(WEAPON_SPRINGTRAP_AXE_STATUS_KIND_STICK.into(), false.into());
    }
    if should_remove_axe(weapon) {
        remove_axe(weapon);
    }
    if MotionModule::is_end(weapon.module_accessor) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
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