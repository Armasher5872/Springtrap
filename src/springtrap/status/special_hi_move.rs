use super::*;

unsafe extern "C" fn springtrap_special_hi_move_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_BRAKE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_ATTACK);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_move_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rot_angle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE) as f32;
    let speed_x = (rot_angle+90.0).to_radians().sin()*12.0;
    let speed_y = (rot_angle-90.0).to_radians().cos()*12.0;
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.04, 0.04);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 20.0, 20.0);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_move_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let pos = PostureModule::pos(fighter.module_accessor);
    let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("springtrap_static"), Hash40::new("hip"), &Vector3f{x: (*pos).x, y: (*pos).y, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.5, true, 0, 0, 0, 0, 0, true, true);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    JostleModule::set_status(fighter.module_accessor, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    EffectModule::set_alpha(fighter.module_accessor, effect as u32, 0.5);
    WorkModule::set_int(fighter.module_accessor, effect as i32, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_move"), L2CValue::Hash40s("special_air_hi_move"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(springtrap_special_hi_move_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_special_hi_move_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let move_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_TIME);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::set_float(fighter.module_accessor, 40.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    if move_time > 5 {
        fighter.change_status(FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_move_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_TIME);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_move_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
    EFFECT_OFF_KIND(fighter, Hash40::new("springtrap_static"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_TIME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_move_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
    EFFECT_OFF_KIND(fighter, Hash40::new("springtrap_static"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_TIME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    GroundModule::set_ignore_boss(fighter.module_accessor, false);
    JostleModule::set_status(fighter.module_accessor, true);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE, springtrap_special_hi_move_pre_status)
    .status(Init, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE, springtrap_special_hi_move_init_status)
    .status(Main, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE, springtrap_special_hi_move_main_status)
    .status(Exec, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE, springtrap_special_hi_move_exec_status)
    .status(End, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE, springtrap_special_hi_move_end_status)
    .status(Exit, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE, springtrap_special_hi_move_exit_status)
    .install()
    ;
}