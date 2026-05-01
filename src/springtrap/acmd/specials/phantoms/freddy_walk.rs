use super::*;

//Phantom Freddy Walk ACMD
unsafe extern "C" fn springtrap_phantom_freddy_walk_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new_raw(0x114266b361), 0.0, 0, 0, 0, 0, 7.0, 0.0, 0.0, 0.0, Some(0.0), Some(14.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
    }
}

//Phantom Freddy Walk Effect
unsafe extern "C" fn springtrap_phantom_freddy_walk_effect(_agent: &mut L2CAgentBase) {}

//Phantom Freddy Walk Sound
unsafe extern "C" fn springtrap_phantom_freddy_walk_sound(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_freddywalk", springtrap_phantom_freddy_walk_acmd, Low)
    .acmd("effect_freddywalk", springtrap_phantom_freddy_walk_effect, Low)
    .acmd("sound_freddywalk", springtrap_phantom_freddy_walk_sound, Low)
    .install()
    ;
}