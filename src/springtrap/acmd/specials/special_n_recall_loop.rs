use super::*;

//Neutral Special Recall Loop ACMD
unsafe extern "C" fn springtrap_neutral_special_recall_loop_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("handl"), 4.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_specialnrecallloop", springtrap_neutral_special_recall_loop_acmd, Low)
    .acmd("game_specialairnrecallloop", springtrap_neutral_special_recall_loop_acmd, Low)
    .install()
    ;
}