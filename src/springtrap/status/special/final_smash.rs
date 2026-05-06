use super::*;

unsafe extern "C" fn springtrap_final_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, 0);
    0.into()
}

unsafe extern "C" fn springtrap_final_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    0.into()
}

unsafe extern "C" fn springtrap_final_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
    AreaModule::set_whole(fighter.module_accessor, false);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("final_start"), L2CValue::Hash40s("final_air_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(springtrap_final_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final_start"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("final_air_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return 1.into()
    }
    0.into()
}

unsafe extern "C" fn springtrap_final_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_final_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
    0.into()
}

unsafe extern "C" fn springtrap_final_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .status(Pre, *FIGHTER_STATUS_KIND_FINAL, springtrap_final_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_FINAL, springtrap_final_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_FINAL, springtrap_final_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_FINAL, springtrap_final_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_FINAL, springtrap_final_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_FINAL, springtrap_final_exit_status)
    .install()
    ;
}