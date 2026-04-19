use super::*;

unsafe extern "C" fn springtrap_up_special_move_acmd(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_up_special_move_effect(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_up_special_move_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_h02"));
    }
}

unsafe extern "C" fn springtrap_up_special_move_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_specialhimove", springtrap_up_special_move_acmd, Low)
    .effect_acmd("effect_specialhimove", springtrap_up_special_move_effect, Low)
    .sound_acmd("sound_specialhimove", springtrap_up_special_move_sound, Low)
    .expression_acmd("expression_specialhimove", springtrap_up_special_move_expression, Low)
    .game_acmd("game_specialairhimove", springtrap_up_special_move_acmd, Low)
    .effect_acmd("effect_specialairhimove", springtrap_up_special_move_effect, Low)
    .sound_acmd("sound_specialairhimove", springtrap_up_special_move_sound, Low)
    .expression_acmd("expression_specialairhimove", springtrap_up_special_move_expression, Low)
    .install()
    ;
}