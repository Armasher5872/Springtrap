use super::*;

//Phantom Freddy Walk ACMD
unsafe extern "C" fn springtrap_phantom_freddy_walk_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 7.0, 0.0, 0.0, 0.0, Some(0.0), Some(14.0), Some(0.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
}

//Phantom Freddy Walk Effect
unsafe extern "C" fn springtrap_phantom_freddy_walk_effect(_agent: &mut L2CAgentBase) {}

//Phantom Freddy Walk Sound
unsafe extern "C" fn springtrap_phantom_freddy_walk_sound(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume(get_costumes())
    .acmd("game_freddywalk", springtrap_phantom_freddy_walk_acmd, Low)
    .acmd("effect_freddywalk", springtrap_phantom_freddy_walk_effect, Low)
    .acmd("sound_freddywalk", springtrap_phantom_freddy_walk_sound, Low)
    .install()
    ;
}