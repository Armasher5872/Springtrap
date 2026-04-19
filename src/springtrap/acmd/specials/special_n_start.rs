use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn springtrap_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
        FT_MOTION_RATE(agent, 15.0/30.0);
    }
}

//Grounded Neutral Special Start Effect
unsafe extern "C" fn springtrap_grounded_neutral_special_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_item_get"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

//Aerial Neutral Special Start Effect
unsafe extern "C" fn springtrap_aerial_neutral_special_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_item_get"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

//Neutral Special Start Sound
unsafe extern "C" fn springtrap_neutral_special_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_item_item_get"));
    }
}

//Neutral Special Start Expression
unsafe extern "C" fn springtrap_neutral_special_start_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_79_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_specialnstart", springtrap_neutral_special_start_acmd, Low)
    .effect_acmd("effect_specialnstart", springtrap_grounded_neutral_special_start_effect, Low)
    .sound_acmd("sound_specialnstart", springtrap_neutral_special_start_sound, Low)
    .expression_acmd("expression_specialnstart", springtrap_neutral_special_start_expression, Low)
    .game_acmd("game_specialairnstart", springtrap_neutral_special_start_acmd, Low)
    .effect_acmd("effect_specialairnstart", springtrap_aerial_neutral_special_start_effect, Low)
    .sound_acmd("sound_specialairnstart", springtrap_neutral_special_start_sound, Low)
    .expression_acmd("expression_specialairnstart", springtrap_neutral_special_start_expression, Low)
    .install()
    ;
}