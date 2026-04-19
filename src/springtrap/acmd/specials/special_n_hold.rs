use super::*;

//Neutral Special Charge Loop ACMD
unsafe extern "C" fn springtrap_neutral_special_charge_loop_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 15.0/30.0);
    }
}

//Grounded Neutral Special Charge Loop Effect
unsafe extern "C" fn springtrap_grounded_neutral_special_charge_loop_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
    }
}

//Aerial Neutral Special Charge Loop Effect
unsafe extern "C" fn springtrap_aerial_neutral_special_charge_loop_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
    }
}

//Neutral Special Charge Loop Sound
unsafe extern "C" fn springtrap_neutral_special_charge_loop_sound(_agent: &mut L2CAgentBase) {
  
}

//Neutral Special Charge Loop Expression
unsafe extern "C" fn springtrap_neutral_special_charge_loop_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_23_hold"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_specialnhold", springtrap_neutral_special_charge_loop_acmd, Low)
    .effect_acmd("effect_specialnhold", springtrap_grounded_neutral_special_charge_loop_effect, Low)
    .sound_acmd("sound_specialnhold", springtrap_neutral_special_charge_loop_sound, Low)
    .expression_acmd("expression_specialnhold", springtrap_neutral_special_charge_loop_expression, Low)
    .game_acmd("game_specialairnhold", springtrap_neutral_special_charge_loop_acmd, Low)
    .effect_acmd("effect_specialairnhold", springtrap_aerial_neutral_special_charge_loop_effect, Low)
    .sound_acmd("sound_specialairnhold", springtrap_neutral_special_charge_loop_sound, Low)
    .expression_acmd("expression_specialairnhold", springtrap_neutral_special_charge_loop_expression, Low)
    .install()
    ;
}