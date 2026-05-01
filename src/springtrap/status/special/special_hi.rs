use super::*;

unsafe extern "C" fn springtrap_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_NONE);
        KineticModule::clear_speed_all(boma);
        KineticModule::unable_energy_all(boma);
    }
    else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let effect = EffectModule::req_follow(boma, Hash40::new("springtrap_static"), Hash40::new("trans"), &Vector3f{x: 0.0, y: 15.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.5, true, 0, 0, 0, 0, 0, true, true);
    WorkModule::set_int(boma, effect as i32, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    if situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion(boma, Hash40::new("special_air_hi"), 0.0, 1.5, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("special_hi"), 0.0, 1.5, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(springtrap_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let effect_id = WorkModule::get_int(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new("special_hi"), -1.0, 1.0, 0.0);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0);
        }
    }
    EffectModule::set_alpha(boma, effect_id as u32, current_frame/24.0);
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let stick = fighter.Vector2__create(stick_x.into(), stick_y.into());
    let vec_stick_x = stick["x"].get_f32();
    let vec_stick_y = stick["y"].get_f32();
    let stick_angle = vec_stick_y.atan2(vec_stick_x);
    let stick_degrees = stick_angle.to_degrees();
    if current_frame >= 10.0 {
        WorkModule::set_int(fighter.module_accessor, stick_degrees as i32, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    }
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        VisibilityModule::set_whole(boma, true);
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        GroundModule::set_ignore_boss(boma, false);
        JostleModule::set_status(boma, true);
        WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
        WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_TIME);
        WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
    EFFECT_OFF_KIND(fighter, Hash40::new("springtrap_static"), true, true);
    0.into()
}

unsafe extern "C" fn springtrap_special_hi_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_MOVE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        VisibilityModule::set_whole(boma, true);
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        GroundModule::set_ignore_boss(boma, false);
        JostleModule::set_status(boma, true);
        WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
        WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_TIME);
        WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
    EFFECT_OFF_KIND(fighter, Hash40::new("springtrap_static"), true, true);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, springtrap_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, springtrap_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, springtrap_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, springtrap_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, springtrap_special_hi_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, springtrap_special_hi_exit_status)
    .install()
    ;
}