use super::*;

unsafe extern "C" fn springtrap_up_special_end_acmd(_agent: &mut L2CAgentBase) {}

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
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_specialhiend", springtrap_up_special_end_acmd, Low)
    .effect_acmd("effect_specialhiend", springtrap_up_special_end_effect, Low)
    .sound_acmd("sound_specialhiend", springtrap_up_special_end_sound, Low)
    .expression_acmd("expression_specialhiend", springtrap_up_special_end_expression, Low)
    .game_acmd("game_specialairhiend", springtrap_up_special_end_acmd, Low)
    .effect_acmd("effect_specialairhiend", springtrap_up_special_end_effect, Low)
    .sound_acmd("sound_specialairhiend", springtrap_up_special_end_sound, Low)
    .expression_acmd("expression_specialairhiend", springtrap_up_special_end_expression, Low)
    .install()
    ;
}