use super::*;

unsafe extern "C" fn springtrap_walk_middle_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 13.0);
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footl"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 46.0);
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("footr"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        agent.clear_lua_stack();
        wait_loop_sync_mot(lua_state);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn springtrap_walk_middle_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        frame(lua_state, 13.0);
        if is_excute(agent) {
            PLAY_STEP(agent, Hash40::new("se_ganon_step_left_s"));
        }
        frame(lua_state, 46.0);
        PLAY_STEP(agent, Hash40::new("se_ganon_step_right_s"));
        agent.clear_lua_stack();
        wait_loop_sync_mot(lua_state);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn springtrap_walk_middle_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    loop {
        if is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
        }
        frame(lua_state, 13.0);
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(lua_state, 46.0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        agent.clear_lua_stack();
        wait_loop_sync_mot(lua_state);
        agent.pop_lua_stack(1);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .acmd("effect_walkmiddle", springtrap_walk_middle_effect, Low)
    .acmd("sound_walkmiddle", springtrap_walk_middle_sound, Low)
    .acmd("expression_walkmiddle", springtrap_walk_middle_expression, Low)
    .install()
    ;
}