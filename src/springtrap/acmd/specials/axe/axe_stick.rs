use super::*;

//Axe Stick Effect
unsafe extern "C" fn springtrap_axe_stick_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("springtrap_axe_ground_crack"), Hash40::new("top"), 6, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 13, 0, 0, 0, 0, 2.0, true);
    }
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume(get_costumes())
    .acmd("effect_stick", springtrap_axe_stick_effect, Low)
    .install()
    ;
}