use super::*;

unsafe extern "C" fn springtrap_phantom_chica_fall_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_fall_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    WorkModule::set_int(boma, -1, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.07);
    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 3.0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_fall_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    ReflectorModule::set_size(boma, *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY, 10.0, 0);
    MotionModule::change_motion(boma, Hash40::new("chica_fall"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_chica_fall_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_chica_fall_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_phantom(weapon) {
        remove_phantom(weapon);
    }
    if GroundModule::is_floor_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN as u32) && frame > 1.0 {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x2f89bbb63a));
        WorkModule::on_flag(boma, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
        PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
        weapon.change_status(WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK.into(), false.into());
    }
    if life == 40 {
        phantom_disappear(weapon, false, 0x31ed91fca);
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("chica_fall"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_fall_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_fall_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_fall_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_FALL, springtrap_phantom_chica_fall_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_FALL, springtrap_phantom_chica_fall_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_FALL, springtrap_phantom_chica_fall_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_FALL, springtrap_phantom_chica_fall_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_FALL, springtrap_phantom_chica_fall_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_FALL, springtrap_phantom_chica_fall_exit_status)
    .install()
    ;
}