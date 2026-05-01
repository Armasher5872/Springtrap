use super::*;

unsafe extern "C" fn springtrap_down_taunt_game(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_down_taunt_effect(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_down_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 55.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_l01"));
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_appeal_l01"));
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_l01"));
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_l01"));
    }
    frame(lua_state, 100.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_l01"));
    }
    frame(lua_state, 110.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_appeal_l02"));
    }
    frame(lua_state, 200.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_attackhard_h01"));
    }
}

unsafe extern "C" fn springtrap_down_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 110.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(lua_state, 150.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 40, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_appeallwl", springtrap_down_taunt_game, Low)
    .acmd("effect_appeallwl", springtrap_down_taunt_effect, Low)
    .acmd("sound_appeallwl", springtrap_down_taunt_sound, Low)
    .acmd("expression_appeallwl", springtrap_down_taunt_expression, Low)
    .acmd("game_appeallwr", springtrap_down_taunt_game, Low)
    .acmd("effect_appeallwr", springtrap_down_taunt_effect, Low)
    .acmd("sound_appeallwr", springtrap_down_taunt_sound, Low)
    .acmd("expression_appeallwr", springtrap_down_taunt_expression, Low)
    .install()
    ;
}