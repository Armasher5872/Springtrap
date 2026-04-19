use super::*;

//Axe Foxy Attack ACMD
unsafe extern "C" fn springtrap_axe_foxy_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new_raw(0x9193dceeb), 8.0, 150, 20, 0, 80, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
}

//Axe Foxy Attack Effect
unsafe extern "C" fn springtrap_axe_foxy_attack_effect(_agent: &mut L2CAgentBase) {}

//Axe Foxy Attack Sound
unsafe extern "C" fn springtrap_axe_foxy_attack_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_attackhard_h01"));
    }
}

pub fn install() {
    Agent::new("ganon_axe")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_foxyattack", springtrap_axe_foxy_attack_acmd, Low)
    .effect_acmd("effect_foxyattack", springtrap_axe_foxy_attack_effect, Low)
    .sound_acmd("sound_foxyattack", springtrap_axe_foxy_attack_sound, Low)
    .install()
    ;
}