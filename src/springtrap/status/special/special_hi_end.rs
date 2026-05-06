use super::*;

unsafe extern "C" fn springtrap_special_hi_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_end_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_all(boma);
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    VisibilityModule::set_whole(boma, true);
    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    GroundModule::set_ignore_boss(boma, false);
    JostleModule::set_status(boma, true);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let effect = EffectModule::req_follow(boma, Hash40::new("springtrap_static"), Hash40::new("trans"), &Vector3f{x: 0.0, y: 15.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.5, true, 0, 0, 0, 0, 0, true, true);
    WorkModule::set_int(boma, effect as i32, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_end"), L2CValue::Hash40s("special_air_hi_end"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(springtrap_special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let effect_id = WorkModule::get_int(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if current_frame > 2.0 {
                WorkModule::set_float(boma, 40.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
            else {    
                MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new("special_hi_end"), -1.0, 1.0, 0.0);
            }
            return 1.into();
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    EffectModule::set_alpha(boma, effect_id as u32, 0.5-(current_frame/60.0));
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_end_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("springtrap_static"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_end_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("springtrap_static"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .status(Pre, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END, springtrap_special_hi_end_pre_status)
    .status(Init, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END, springtrap_special_hi_end_init_status)
    .status(Main, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END, springtrap_special_hi_end_main_status)
    .status(Exec, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END, springtrap_special_hi_end_exec_status)
    .status(End, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END, springtrap_special_hi_end_end_status)
    .status(Exit, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END, springtrap_special_hi_end_exit_status)
    .install()
    ;
}