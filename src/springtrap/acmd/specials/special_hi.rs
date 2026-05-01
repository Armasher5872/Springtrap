use super::*;

unsafe extern "C" fn springtrap_up_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 40.0/30.0);
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
    .acmd("game_specialhi", springtrap_up_special_acmd, Low)
    .acmd("effect_specialhi", springtrap_up_special_effect, Low)
    .acmd("sound_specialhi", springtrap_up_special_sound, Low)
    .acmd("expression_specialhi", springtrap_up_special_expression, Low)
    .acmd("game_specialairhi", springtrap_up_special_acmd, Low)
    .acmd("effect_specialairhi", springtrap_up_special_effect, Low)
    .acmd("sound_specialairhi", springtrap_up_special_sound, Low)
    .acmd("expression_specialairhi", springtrap_up_special_expression, Low)
    .install()
    ;
}