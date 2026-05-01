use super::*;

unsafe extern "C" fn springtrap_attack_s4_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let hold_frame = WorkModule::get_param_float(boma, hash40("attack_4_hold_frame"), 0);
    let s4_hold_frame = WorkModule::get_param_int(boma, hash40("attack_s4_hold_frame"), 0);
    let keep_frame = WorkModule::get_param_int(boma, hash40("attack_s4_hold_keep_frame"), 0);
    let ratio = s4_hold_frame as f32/hold_frame;
    if !StopModule::is_stop(boma) {
        fighter.sub_smash_hold_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_smash_hold_uniq as *const () as _));
    WorkModule::set_int(boma, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_BASE_FRAME);
    WorkModule::set_int(boma, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    WorkModule::set_int(boma, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_TOTAL_FRAME);
    WorkModule::set_int(boma, keep_frame, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);
    MotionModule::change_motion(boma, Hash40::new("attack_s4_hold"), 0.0, s4_hold_frame as f32/ratio, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(springtrap_attack_s4_hold_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_attack_s4_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if ArticleModule::is_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
        let axe_boma = get_article_boma(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD);
        ModelModule::set_scale(axe_boma, 0.8);
        LinkModule::set_model_constraint_pos_ort(axe_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("haver"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        LinkModule::set_constraint_translate_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        LinkModule::set_constraint_rot_offset(axe_boma, &Vector3f{x: 90.0, y: -90.0, z: 0.0});
    }
    fighter.status_AttackS4Hold_main()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, springtrap_attack_s4_hold_main_status)
    .install()
    ;
}