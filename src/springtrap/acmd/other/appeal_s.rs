use super::*;

unsafe extern "C" fn springtrap_side_taunt_effect(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_side_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let blahblahblah = if sv_math::randf(hash40("fighter"), 1.0) < 0.1 {true} else {false};
    frame(lua_state, 30.0);
    if !blahblahblah {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("vc_ganon_appeal_s01"));
        }
    }
    else {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_ganon_appeal_s02"));
        }   
    }
    frame(lua_state, 40.0);
    if !blahblahblah {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_ganon_appeal_s01"));
        }
    }
    frame(lua_state, 60.0);
    if !blahblahblah {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_ganon_appeal_s01"));
        }
    }
    frame(lua_state, 85.0);
    if !blahblahblah {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_ganon_appeal_s01"));
        }
    }
}

unsafe extern "C" fn springtrap_side_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 42.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 87.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 18, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 112.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .acmd("effect_appealsl", springtrap_side_taunt_effect, Low)
    .acmd("sound_appealsl", springtrap_side_taunt_sound, Low)
    .acmd("expression_appealsl", springtrap_side_taunt_expression, Low)
    .acmd("effect_appealsr", springtrap_side_taunt_effect, Low)
    .acmd("sound_appealsr", springtrap_side_taunt_sound, Low)
    .acmd("expression_appealsr", springtrap_side_taunt_expression, Low)
    .install()
    ;
}