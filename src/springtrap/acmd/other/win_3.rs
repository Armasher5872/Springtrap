use super::*;

unsafe extern "C" fn springtrap_win_3_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
        if ArticleModule::is_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
            let axe_boma = get_article_boma(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD);
            ModelModule::set_scale(axe_boma, 0.73);
            LinkModule::set_model_constraint_pos_ort(axe_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("throw"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
            LinkModule::set_constraint_translate_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
            LinkModule::set_constraint_rot_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        }
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        if ArticleModule::is_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
            let axe_boma = get_article_boma(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD);
            LinkModule::set_model_constraint_pos_ort(axe_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
            LinkModule::set_constraint_translate_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
            LinkModule::set_constraint_rot_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        }
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        VisibilityModule::set_model_visible(boma, false);
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(0));
    }
    frame(lua_state, 160.0);
    if is_excute(agent) {
        VisibilityModule::set_model_visible(boma, true);
    }
    frame(lua_state, 218.0);
    if is_excute(agent) {
        VisibilityModule::set_model_visible(boma, false);
    }
}

unsafe extern "C" fn springtrap_win_3_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 70.0);
    if is_excute(agent) {
        let effect = EffectModule::req_on_joint(boma, Hash40::new("springtrap_static"), Hash40::new("trans"), &Vector3f{x: 0.0, y: 11.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.5, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, false, 0, 0, 0);
        WorkModule::set_int(boma, effect as i32, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
    frame(lua_state, 91.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("springtrap_static"), true, true);
        WorkModule::set_int(boma, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    }
    frame(lua_state, 218.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("springtrap_static"), Hash40::new("trans"), 0.0, 15.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0, 0, 0, 0, 0, 0, false, 0.5);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.13, 0.1);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_bg_black"), 0, 0, 0, 0, 0, 0, 1);
        sv_animcmd::EFFECT_GLOBAL_BACK_GROUND(agent.lua_state_agent);
    }
}

unsafe extern "C" fn springtrap_win_3_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h05"));
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h01"));
    }
    frame(lua_state, 160.0);
    if is_excute(agent) {
        let vc_index = if sv_math::randf(hash40("fighter"), 1.0) > 0.5 {Hash40::new("vc_ganon_appeal_h01")} else {Hash40::new("vc_ganon_attackhard_h01")};
        PLAY_SE(agent, vc_index);
    }
    frame(lua_state, 218.0);
    if is_excute(agent) {
        let crit = SoundModule::play_se(boma, Hash40::new("se_ganon_attackhard_h03"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, crit as i32, 2.0, 0);
        PLAY_SE(agent, Hash40::new("vc_ganon_win03"));
    }
}

unsafe extern "C" fn springtrap_win_3_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_win3", springtrap_win_3_game, Low)
    .acmd("effect_win3", springtrap_win_3_effect, Low)
    .acmd("sound_win3", springtrap_win_3_sound, Low)
    .acmd("expression_win3", springtrap_win_3_expression, Low)
    .install()
    ;
}