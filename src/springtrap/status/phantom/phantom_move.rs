use super::*;

unsafe extern "C" fn springtrap_phantom_phantom_move_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_move_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let prev_status_kind = weapon.global_table[PREV_STATUS_KIND].get_i32();
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(boma);
    let current_x_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let owner_init_lr = WorkModule::get_float(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    if GroundModule::is_floor_touch_line(boma, *GROUND_TOUCH_FLAG_DOWN as u32) {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        weapon.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::enable_energy(boma, *WEAPON_KOOPAJR_CANNONBALL_KINETIC_ENERGY_ID_GRAVITY);
    }
    sv_kinetic_energy!(reset_energy, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0, 0.0, 0.0, 0.0);
    if prev_status_kind != *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN {
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
            if situation_kind != *SITUATION_KIND_GROUND {
                sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.07);
                sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -3.0);
                sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 3.0);
            }
            else {
                if !WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_WAS_INIT_SPAWN) {
                    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*lr, 0.0);
                    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*lr, 0.0);
                    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5, 0.0);
                }
                else {
                    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*owner_init_lr, 0.0);
                    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*owner_init_lr, 0.0);
                    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5, 0.0);
                }
            }
        }
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
            if situation_kind != *SITUATION_KIND_GROUND {
                sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.07);
                sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -3.0);
                sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 3.0);
            }
            else {
                if !WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_WAS_INIT_SPAWN) {
                    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*lr, 0.0);
                    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*lr, 0.0);
                    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25, 0.0);
                }
                else {
                    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*owner_init_lr, 0.0);
                    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*owner_init_lr, 0.0);
                    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25, 0.0);
                }
            }
        }
    }
    else {
        sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -(current_x_speed*10.0), 0.0);
    }
    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_move_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let boma = weapon.module_accessor;
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NORMAL), *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
    ReflectorModule::set_size(boma, *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY, 10.0, 0);
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
        if situation_kind == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(boma, Hash40::new("chica_walk"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, Hash40::new("chica_fall"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
        if situation_kind == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(boma, Hash40::new("freddy_walk"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, Hash40::new("freddy_fall"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    weapon.fastshift(L2CValue::Ptr(springtrap_phantom_phantom_move_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_phantom_phantom_move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = weapon.module_accessor;
    let lr = PostureModule::lr(boma);
    let owner_init_lr = WorkModule::get_float(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_OWNER_INIT_LR);
    let phantom_type = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_PHANTOM_TYPE);
    let life = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if should_remove_phantom(weapon) {
        remove_phantom(weapon);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if GroundModule::is_ottotto(boma, 3.0) 
        || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_LEFT as u32) 
        || GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_RIGHT as u32) {
            weapon.change_status(WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN.into(), false.into());
        }
        if prev_situation_kind == *SITUATION_KIND_AIR {
            weapon.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            sv_kinetic_energy!(reset_energy, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0, 0.0, 0.0, 0.0);
            if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
                if WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_WAS_INIT_SPAWN) {
                    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*owner_init_lr, 0.0);
                    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*owner_init_lr, 0.0);
                    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5, 0.0);
                }
                else {
                    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*lr, 0.0);
                    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5*lr, 0.0);
                    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5, 0.0);
                }
                MotionModule::change_motion(boma, Hash40::new("chica_walk"), 0.0, 1.0, false, 0.0, false, false);
            }
            if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
                if WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_WAS_INIT_SPAWN) {
                    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*owner_init_lr, 0.0);
                    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*owner_init_lr, 0.0);
                    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25, 0.0);
                }
                else {
                    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*lr, 0.0);
                    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25*lr, 0.0);
                    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.25, 0.0);
                }
                MotionModule::change_motion(boma, Hash40::new("freddy_walk"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            weapon.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            sv_kinetic_energy!(reset_energy, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -0.07);
            sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -3.0);
            sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 3.0);
            KineticModule::enable_energy(boma, *WEAPON_KOOPAJR_CANNONBALL_KINETIC_ENERGY_ID_GRAVITY);
            if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
                MotionModule::change_motion(boma, Hash40::new("chica_fall"), 0.0, 1.0, false, 0.0, false, false);
            }
            if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
                MotionModule::change_motion(boma, Hash40::new("freddy_fall"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
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
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_CHICA {
            if situation_kind == *SITUATION_KIND_GROUND {
                MotionModule::change_motion(boma, Hash40::new("chica_walk"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("chica_fall"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if phantom_type == *SPRINGTRAP_PHANTOM_TYPE_FREDDY {
            if situation_kind == *SITUATION_KIND_GROUND {
                MotionModule::change_motion(boma, Hash40::new("freddy_walk"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(boma, Hash40::new("freddy_fall"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_move_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_move_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_phantom_phantom_move_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_cannonballcloned")
    .set_costume(get_springtrap_costumes_acmd())
    .status(Pre, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE, springtrap_phantom_phantom_move_pre_status)
    .status(Init, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE, springtrap_phantom_phantom_move_init_status)
    .status(Main, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE, springtrap_phantom_phantom_move_main_status)
    .status(Exec, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE, springtrap_phantom_phantom_move_exec_status)
    .status(End, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE, springtrap_phantom_phantom_move_end_status)
    .status(Exit, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE, springtrap_phantom_phantom_move_exit_status)
    .install()
    ;
}