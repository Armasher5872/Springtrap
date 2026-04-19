use super::*;

unsafe extern "C" fn springtrap_up_taunt_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 60.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 20, 0, 20, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 65.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 20, 0, 20, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 20, 0, 20, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 75.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 20, 0, 20, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 80.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 20, 0, 20, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 85.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 20, 0, 20, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 20, 0, 20, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
    frame(lua_state, 95.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 20, 0, 20, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(agent, 0.5);
    }
}

unsafe extern "C" fn springtrap_up_taunt_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 60.0);
    if is_excute(agent) {
        let vc_index = if sv_math::randf(hash40("fighter"), 1.0) > 0.5 {Hash40::new("vc_ganon_appeal_h01")} else {Hash40::new("vc_ganon_attackhard_h01")};
        PLAY_SE(agent, vc_index);
    }
}

unsafe extern "C" fn springtrap_up_taunt_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 60.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 40, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    for _ in 0..4 {
        if is_excute(agent) {
            QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
        wait(lua_state, 10.0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .effect_acmd("effect_appealhil", springtrap_up_taunt_effect, Low)
    .sound_acmd("sound_appealhil", springtrap_up_taunt_sound, Low)
    .expression_acmd("expression_appealhil", springtrap_up_taunt_expression, Low)
    .effect_acmd("effect_appealhir", springtrap_up_taunt_effect, Low)
    .sound_acmd("sound_appealhir", springtrap_up_taunt_sound, Low)
    .expression_acmd("expression_appealhir", springtrap_up_taunt_expression, Low)
    .install()
    ;
}