use super::*;

//Phantom Balloon Boy Idle ACMD
unsafe extern "C" fn springtrap_phantom_bb_idle_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new_raw(0x114266b361), 7.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
}

//Phantom Balloon Boy Idle Effect
unsafe extern "C" fn springtrap_phantom_bb_idle_effect(_agent: &mut L2CAgentBase) {}

//Phantom Balloon Boy Idle Sound
unsafe extern "C" fn springtrap_phantom_bb_idle_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 30.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_l01"));
    }
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .acmd("game_bbidle", springtrap_phantom_bb_idle_acmd, Low)
    .acmd("effect_bbidle", springtrap_phantom_bb_idle_effect, Low)
    .acmd("sound_bbidle", springtrap_phantom_bb_idle_sound, Low)
    .install()
    ;
}