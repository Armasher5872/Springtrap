use super::*;

unsafe extern "C" fn springtrap_special_s_hold_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn springtrap_special_s_hold_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_s_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_hold"), L2CValue::Hash40s("special_air_s_hold"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(springtrap_special_s_hold_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_special_s_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_s_hold"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_s_hold"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_s_hold_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    WorkModule::set_float(fighter.module_accessor, current_frame/58.0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE);
    0.into()
}

unsafe extern "C" fn springtrap_special_s_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_ATTACK {
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE);
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_s_hold_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_ATTACK {
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE);
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .status(Pre, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_HOLD, springtrap_special_s_hold_pre_status)
    .status(Init, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_HOLD, springtrap_special_s_hold_init_status)
    .status(Main, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_HOLD, springtrap_special_s_hold_main_status)
    .status(Exec, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_HOLD, springtrap_special_s_hold_exec_status)
    .status(End, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_HOLD, springtrap_special_s_hold_end_status)
    .status(Exit, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_HOLD, springtrap_special_s_hold_exit_status)
    .install()
    ;
}