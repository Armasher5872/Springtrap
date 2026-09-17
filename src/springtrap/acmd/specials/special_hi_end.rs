use super::*;

unsafe extern "C" fn springtrap_up_special_end_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 40.0/30.0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn springtrap_up_special_end_effect(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_up_special_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h03"));
    }
}

unsafe extern "C" fn springtrap_up_special_end_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_springtrap_costumes_acmd())
    .acmd("game_specialhiend", springtrap_up_special_end_acmd, Low)
    .acmd("effect_specialhiend", springtrap_up_special_end_effect, Low)
    .acmd("sound_specialhiend", springtrap_up_special_end_sound, Low)
    .acmd("expression_specialhiend", springtrap_up_special_end_expression, Low)
    .acmd("game_specialairhiend", springtrap_up_special_end_acmd, Low)
    .acmd("effect_specialairhiend", springtrap_up_special_end_effect, Low)
    .acmd("sound_specialairhiend", springtrap_up_special_end_sound, Low)
    .acmd("expression_specialairhiend", springtrap_up_special_end_expression, Low)
    .install()
    ;
}