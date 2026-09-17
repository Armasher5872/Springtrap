use super::*;

unsafe extern "C" fn springtrap_phantom_phantom_turn_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_turn_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let current_x_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, current_x_speed*0.1, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::reverse_lr(boma);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_turn_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    WorkModule::off_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_WAS_INIT_SPAWN);
    ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    ReflectorModule::set_size(boma, *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY, 10.0, 0);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
        MotionModule::change_motion(boma, Hash40::new("chica_turn"), 0.0, 1.0, false, 0.0, false, false);
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
        MotionModule::change_motion(boma, Hash40::new("freddy_turn"), 0.0, 1.0, false, 0.0, false, false);
    }
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_phantom_turn_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_phantom_turn_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
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
        weapon.change_status(WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_turn_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_turn_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_turn_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_cannonballcloned")
    .set_costume(get_springtrap_costumes_acmd())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN, springtrap_phantom_phantom_turn_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN, springtrap_phantom_phantom_turn_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN, springtrap_phantom_phantom_turn_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN, springtrap_phantom_phantom_turn_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN, springtrap_phantom_phantom_turn_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN, springtrap_phantom_phantom_turn_exit_status)
    .install()
    ;
}