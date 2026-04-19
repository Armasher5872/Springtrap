use super::*;

//Neutral Special Recall End Effect
unsafe extern "C" fn springtrap_grounded_neutral_special_recall_end_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_item_get"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 0, 0.8, true);
    }
}

//Neutral Special Recall End Sound
unsafe extern "C" fn springtrap_neutral_special_recall_end_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
}

//Neutral Special Recall End Expression
unsafe extern "C" fn springtrap_neutral_special_recall_end_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_79_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .effect_acmd("effect_specialnrecallend", springtrap_grounded_neutral_special_recall_end_effect, Low)
    .sound_acmd("sound_specialnrecallend", springtrap_neutral_special_recall_end_sound, Low)
    .expression_acmd("expression_specialnrecallend", springtrap_neutral_special_recall_end_expression, Low)
    .effect_acmd("effect_specialairnrecallend", springtrap_grounded_neutral_special_recall_end_effect, Low)
    .sound_acmd("sound_specialairnrecallend", springtrap_neutral_special_recall_end_sound, Low)
    .expression_acmd("expression_specialairnrecallend", springtrap_neutral_special_recall_end_expression, Low)
    .install()
    ;
}