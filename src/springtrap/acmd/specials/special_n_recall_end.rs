use super::*;

//Neutral Special Recall End Game
unsafe extern "C" fn springtrap_neutral_special_recall_end_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE) {
            if ArticleModule::is_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
                let axe_boma = get_article_boma(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD);
                ModelModule::set_scale(axe_boma, 0.73);
                LinkModule::set_model_constraint_pos_ort(axe_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_OFFSET_SCALE | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
                LinkModule::set_constraint_translate_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
                LinkModule::set_constraint_rot_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
            }
        }
        else {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//Neutral Special Recall End Effect
unsafe extern "C" fn springtrap_neutral_special_recall_end_effect(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
            EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
        }
    }
}

//Neutral Special Recall End Sound
unsafe extern "C" fn springtrap_neutral_special_recall_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h05"));
    }
}

//Neutral Special Recall End Expression
unsafe extern "C" fn springtrap_neutral_special_recall_end_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_79_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .acmd("game_specialnrecallend", springtrap_neutral_special_recall_end_game, Low)
    .acmd("effect_specialnrecallend", springtrap_neutral_special_recall_end_effect, Low)
    .acmd("sound_specialnrecallend", springtrap_neutral_special_recall_end_sound, Low)
    .acmd("expression_specialnrecallend", springtrap_neutral_special_recall_end_expression, Low)
    .acmd("game_specialairnrecallend", springtrap_neutral_special_recall_end_game, Low)
    .acmd("effect_specialairnrecallend", springtrap_neutral_special_recall_end_effect, Low)
    .acmd("sound_specialairnrecallend", springtrap_neutral_special_recall_end_sound, Low)
    .acmd("expression_specialairnrecallend", springtrap_neutral_special_recall_end_expression, Low)
    .install()
    ;
}