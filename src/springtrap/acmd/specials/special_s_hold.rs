use super::*;

unsafe extern "C" fn springtrap_side_special_hold_acmd(_agent: &mut L2CAgentBase) {}

unsafe extern "C" fn springtrap_side_special_hold_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    for _ in 0..12 {
        if is_excute(agent) {
            FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 15, 0, 10, 0, 0, 0, false);
        }
        wait(lua_state, 5.0);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
    }
}

unsafe extern "C" fn springtrap_side_special_hold_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_s01"));
    }
}

unsafe extern "C" fn springtrap_side_special_hold_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_specialshold", springtrap_side_special_hold_acmd, Low)
    .effect_acmd("effect_specialshold", springtrap_side_special_hold_effect, Low)
    .sound_acmd("sound_specialshold", springtrap_side_special_hold_sound, Low)
    .expression_acmd("expression_specialshold", springtrap_side_special_hold_expression, Low)
    .game_acmd("game_specialairshold", springtrap_side_special_hold_acmd, Low)
    .effect_acmd("effect_specialairshold", springtrap_side_special_hold_effect, Low)
    .sound_acmd("sound_specialairshold", springtrap_side_special_hold_sound, Low)
    .expression_acmd("expression_specialairshold", springtrap_side_special_hold_expression, Low)
    .install()
    ;
}