use super::*;

//Phantom Chica Walk ACMD
unsafe extern "C" fn springtrap_phantom_chica_walk_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 8.0, 0.0, 4.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 2, 0.0, 0, true, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        SEARCH(agent, 0, 0, Hash40::new("top"), 7.0, 0.0, 4.0, 0.0, Some(0.0), Some(14.0), Some(0.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
}

//Phantom Chica Walk Effect
unsafe extern "C" fn springtrap_phantom_chica_walk_effect(_agent: &mut L2CAgentBase) {}

//Phantom Chica Walk Sound
unsafe extern "C" fn springtrap_phantom_chica_walk_sound(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .acmd("game_chicawalk", springtrap_phantom_chica_walk_acmd, Low)
    .acmd("effect_chicawalk", springtrap_phantom_chica_walk_effect, Low)
    .acmd("sound_chicawalk", springtrap_phantom_chica_walk_sound, Low)
    .install()
    ;
}