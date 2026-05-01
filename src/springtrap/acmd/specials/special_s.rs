use super::*;

unsafe extern "C" fn springtrap_side_special_acmd(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_side_special_effect(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_side_special_sound(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_specials", springtrap_side_special_acmd, Low)
    .acmd("effect_specials", springtrap_side_special_effect, Low)
    .acmd("sound_specials", springtrap_side_special_sound, Low)
    .acmd("expression_specials", springtrap_side_special_expression, Low)
    .acmd("game_specialairs", springtrap_side_special_acmd, Low)
    .acmd("effect_specialairs", springtrap_side_special_effect, Low)
    .acmd("sound_specialairs", springtrap_side_special_sound, Low)
    .acmd("expression_specialairs", springtrap_side_special_expression, Low)
    .install()
    ;
}