use super::*;

//Phantom Freddy Turn ACMD
unsafe extern "C" fn springtrap_phantom_freddy_turn_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 7.0, 0.0, 0.0, 0.0, Some(0.0), Some(14.0), Some(0.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
}

//Phantom Freddy Turn Effect
unsafe extern "C" fn springtrap_phantom_freddy_turn_effect(_agent: &mut L2CAgentBase) {}

//Phantom Freddy Turn Sound
unsafe extern "C" fn springtrap_phantom_freddy_turn_sound(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon_phantom")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_freddyturn", springtrap_phantom_freddy_turn_acmd, Low)
    .acmd("effect_freddyturn", springtrap_phantom_freddy_turn_effect, Low)
    .acmd("sound_freddyturn", springtrap_phantom_freddy_turn_sound, Low)
    .install()
    ;
}