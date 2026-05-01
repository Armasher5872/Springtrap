use super::*;

//Axe Hit Stick Effect
unsafe extern "C" fn springtrap_axe_hit_stick_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_soul_burst"), Hash40::new("have"), 0, 13, 0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 13, 0, 0, 0, 0, 1, true);
    }
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("effect_hitstick", springtrap_axe_hit_stick_effect, Low)
    .install()
    ;
}