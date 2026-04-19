use super::*;

unsafe extern "C" fn springtrap_attack_s4_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4Start_Common();
    fighter.sub_shift_status_main(L2CValue::Ptr(springtrap_attack_s4_start_main_loop as *const () as _))
}

unsafe extern "C" fn springtrap_attack_s4_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
        let axe_boma = get_article_boma(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD);
        ModelModule::set_scale(axe_boma, 0.8);
        LinkModule::set_model_constraint_pos_ort(axe_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("haver"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        LinkModule::set_constraint_translate_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        LinkModule::set_constraint_rot_offset(axe_boma, &Vector3f{x: 90.0, y: -90.0, z: 0.0});
    }
    fighter.status_AttackS4Start_Main()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_START, springtrap_attack_s4_start_main_status)
    .install()
    ;
}