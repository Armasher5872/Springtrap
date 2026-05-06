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
    .set_costume(get_costumes())
    .acmd("game_specialhimove", springtrap_up_special_move_acmd, Low)
    .acmd("effect_specialhimove", springtrap_up_special_move_effect, Low)
    .acmd("sound_specialhimove", springtrap_up_special_move_sound, Low)
    .acmd("expression_specialhimove", springtrap_up_special_move_expression, Low)
    .acmd("game_specialairhimove", springtrap_up_special_move_acmd, Low)
    .acmd("effect_specialairhimove", springtrap_up_special_move_effect, Low)
    .acmd("sound_specialairhimove", springtrap_up_special_move_sound, Low)
    .acmd("expression_specialairhimove", springtrap_up_special_move_expression, Low)
    .install()
    ;
}