use super::*;

//Phantom Freddy Attack ACMD
unsafe extern "C" fn springtrap_phantom_freddy_attack_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 20.0/40.0);
    }
}

//Phantom Freddy Attack Effect
unsafe extern "C" fn springtrap_phantom_freddy_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_phantom_vortex"), Hash40::new("top"), 0, 10.0, 0, 0, 0, 0, 0.5, true);
    }
}

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
    .set_costume(get_costumes())
    .acmd("game_freddyattack", springtrap_phantom_freddy_attack_acmd, Low)
    .acmd("effect_freddyattack", springtrap_phantom_freddy_attack_effect, Low)
    .acmd("sound_freddyattack", springtrap_phantom_freddy_attack_sound, Low)
    .install()
    ;
}