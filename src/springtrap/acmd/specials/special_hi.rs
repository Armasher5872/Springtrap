use super::*;

unsafe extern "C" fn springtrap_up_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
    }
}

unsafe extern "C" fn springtrap_up_special_effect(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_up_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h01"));
    }
}

unsafe extern "C" fn springtrap_up_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_specialhi", springtrap_up_special_acmd, Low)
    .effect_acmd("effect_specialhi", springtrap_up_special_effect, Low)
    .sound_acmd("sound_specialhi", springtrap_up_special_sound, Low)
    .expression_acmd("expression_specialhi", springtrap_up_special_expression, Low)
    .game_acmd("game_specialairhi", springtrap_up_special_acmd, Low)
    .effect_acmd("effect_specialairhi", springtrap_up_special_effect, Low)
    .sound_acmd("sound_specialairhi", springtrap_up_special_sound, Low)
    .expression_acmd("expression_specialairhi", springtrap_up_special_expression, Low)
    .install()
    ;
}