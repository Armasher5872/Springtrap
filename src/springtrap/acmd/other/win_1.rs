use super::*;

unsafe extern "C" fn springtrap_win_1_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 55.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
        if ArticleModule::is_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
            let axe_boma = get_article_boma(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD);
            ModelModule::set_scale(axe_boma, 0.73);
            LinkModule::set_model_constraint_pos_ort(axe_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
            LinkModule::set_constraint_translate_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
            LinkModule::set_constraint_rot_offset(axe_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        }
    }
    frame(lua_state, 131.0);
    if is_excute(agent) {
        VisibilityModule::set_model_visible(boma, false);
        if ArticleModule::is_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD) {
            let axe_boma = get_article_boma(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD);
            VisibilityModule::set_model_visible(axe_boma, false);
        }
    }
}

unsafe extern "C" fn springtrap_win_1_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 55.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 13, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 129.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_springtrap_knife1"), Hash40::new("tex_springtrap_knife2"), 4, Hash40::new("havel"), 0, 0.0, 0, Hash40::new("havel"), 0, 9.5, 0, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(lua_state, 131.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("springtrap_static"), Hash40::new("trans"), 0.0, 15.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0, 0, 0, 0, 0, 0, false, 0.5);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.13, 0.1);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_bg_black"), 0, 0, 0, 0, 0, 0, 1);
        sv_animcmd::EFFECT_GLOBAL_BACK_GROUND(agent.lua_state_agent);
    }
    frame(lua_state, 138.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn springtrap_win_1_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 40.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appear01"));
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_smash_s01"));
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
    frame(lua_state, 125.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_swing_ll"));
    }
    frame(lua_state, 135.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h02"));
        let crit = SoundModule::play_se(boma, Hash40::new("se_ganon_attackhard_h03"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, crit as i32, 2.0, 0);
    }
    frame(lua_state, 155.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h02"));
    }
}

unsafe extern "C" fn springtrap_win_1_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_win1", springtrap_win_1_game, Low)
    .effect_acmd("effect_win1", springtrap_win_1_effect, Low)
    .sound_acmd("sound_win1", springtrap_win_1_sound, Low)
    .expression_acmd("expression_win1", springtrap_win_1_expression, Low)
    .install()
    ;
}