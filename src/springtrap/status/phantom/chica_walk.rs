use super::*;

unsafe extern "C" fn springtrap_phantom_chica_walk_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_walk_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let prev_status_kind = weapon.global_table[PREV_STATUS_KIND].get_i32();
    let current_x_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let owner_init_lr = WorkModule::get_float(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    if prev_status_kind != *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_TURN {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*owner_init_lr, 0.0);
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, current_x_speed*10.0, 0.0);
    }
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_walk_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    ReflectorModule::set_size(boma, *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY, 10.0, 0);
    MotionModule::change_motion(boma, Hash40::new("chica_walk"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_chica_walk_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_chica_walk_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    PostureModule::set_pos(boma, &Vector3f{x: pos_x, y: pos_y-3.0, z: pos_z});
    if should_remove_phantom(weapon) {
        remove_phantom(weapon);
    }
    if GroundModule::is_ottotto(boma, 1.5) 
    || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_LEFT as u32) 
    || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_RIGHT as u32) {
        weapon.change_status(WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_TURN.into(), false.into());
    }
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
        WorkModule::on_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_CAN_EXPLODE);
    }
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_CAN_EXPLODE);
    }
    if life == 40 {
        phantom_disappear(weapon, true, 0x31ed91fca);
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("chica_walk"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_walk_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_walk_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_phantom_chica_walk_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK, springtrap_phantom_chica_walk_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK, springtrap_phantom_chica_walk_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK, springtrap_phantom_chica_walk_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK, springtrap_phantom_chica_walk_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK, springtrap_phantom_chica_walk_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK, springtrap_phantom_chica_walk_exit_status)
    .install()
    ;
}