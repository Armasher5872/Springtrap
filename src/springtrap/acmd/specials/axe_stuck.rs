use super::*;

//Axe Stuck ACMD
unsafe extern "C" fn springtrap_axe_stuck_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 0, 0, 0, 0, 13.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 30, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
}

//Axe Stuck Effect
unsafe extern "C" fn springtrap_axe_stuck_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_air_distort"), Hash40::new("have"), 0, 11, 0, 0, 0, 0, 1.125, true);
        LAST_EFFECT_SET_RATE(agent, 0.15);
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 11, 0, 0, 0, 0, 2.0, true);
        EFFECT(agent, Hash40::new("springtrap_axe_ground_crack_stuck"), Hash40::new("top"), 6, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.15);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 11, 0, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 11, 0, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 11, 0, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("have"), 0, 11, 0, 0, 0, 0, 2.0, true);
    }
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_stuck", springtrap_axe_stuck_acmd, Low)
    .effect_acmd("effect_stuck", springtrap_axe_stuck_effect, Low)
    .install()
    ;
}