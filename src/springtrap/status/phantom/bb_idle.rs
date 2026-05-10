use super::*;

unsafe extern "C" fn springtrap_phantom_bb_idle_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_bb_idle_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let prev_status_kind = weapon.global_table[PREV_STATUS_KIND].get_i32();
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    if prev_status_kind != *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL {
        WorkModule::set_int(weapon.module_accessor, 1800, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_bb_idle_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    ReflectorModule::set_no_team(boma, true);
    TeamModule::set_team(boma, *TEAM_NONE, false);
    MotionModule::change_motion(boma, Hash40::new("bb_idle"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_bb_idle_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_bb_idle_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_phantom(weapon) {
        remove_phantom(weapon);
    }
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        WorkModule::on_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_CAN_EXPLODE);
    }
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_CAN_EXPLODE);
    }
    if life == 40 {
        phantom_disappear(weapon, false, 0x31ed91fca);
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("bb_idle"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_bb_idle_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_bb_idle_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let status_kind = weapon.global_table[STATUS_KIND].get_i32();
    if status_kind != *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL {
        WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
        WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_bb_idle_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let status_kind = weapon.global_table[STATUS_KIND].get_i32();
    if status_kind != *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL {
        WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
        WorkModule::set_float(boma, 0.0, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, springtrap_phantom_bb_idle_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, springtrap_phantom_bb_idle_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, springtrap_phantom_bb_idle_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, springtrap_phantom_bb_idle_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, springtrap_phantom_bb_idle_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, springtrap_phantom_bb_idle_exit_status)
    .install()
    ;
}