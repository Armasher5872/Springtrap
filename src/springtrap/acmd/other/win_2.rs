use super::*;

unsafe extern "C" fn springtrap_win_2_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 183.0);
    if is_excute(agent) {
        VisibilityModule::set_model_visible(boma, false);
    }
}

unsafe extern "C" fn springtrap_win_2_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 183.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("springtrap_static"), Hash40::new("trans"), 0.0, 15.0, -10.0, 0.0, 0.0, 0.0, 5.0, 0, 0, 0, 0, 0, 0, false, 0.5);
        LAST_EFFECT_SET_COLOR(agent, 0.1, 0.13, 0.1);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_bg_black"), 0, 0, 0, 0, 0, 0, 1);
        sv_animcmd::EFFECT_GLOBAL_BACK_GROUND(agent.lua_state_agent);
    }
}

unsafe extern "C" fn springtrap_win_2_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
    frame(lua_state, 53.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
    frame(lua_state, 57.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_win2"));
    }
    frame(lua_state, 155.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
    frame(lua_state, 183.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_win02"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_h02"));
    }
}

unsafe extern "C" fn springtrap_win_2_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .acmd("game_win2", springtrap_win_2_game, Low)
    .acmd("effect_win2", springtrap_win_2_effect, Low)
    .acmd("sound_win2", springtrap_win_2_sound, Low)
    .acmd("expression_win2", springtrap_win_2_expression, Low)
    .install()
    ;
}