use super::*;

//Phantom Freddy Attack ACMD
unsafe extern "C" fn springtrap_phantom_freddy_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 80, 80, 0, 10.0, 0.0, 12.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        FT_MOTION_RATE(agent, 20.0/40.0);
    }
}

//Phantom Freddy Attack Effect
unsafe extern "C" fn springtrap_phantom_freddy_attack_effect(_agent: &mut L2CAgentBase) {}

//Phantom Freddy Attack Sound
unsafe extern "C" fn springtrap_phantom_freddy_attack_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 35.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_h02"));
    }
}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_freddyattack", springtrap_phantom_freddy_attack_acmd, Low)
    .acmd("effect_freddyattack", springtrap_phantom_freddy_attack_effect, Low)
    .acmd("sound_freddyattack", springtrap_phantom_freddy_attack_sound, Low)
    .install()
    ;
}