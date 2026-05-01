use super::*;

//Neutral Special High Fire ACMD
unsafe extern "C" fn springtrap_neutral_special_high_fire_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE, false, -1);
        WorkModule::on_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
    }
}

//Grounded Neutral Special High Fire Effect
unsafe extern "C" fn springtrap_grounded_neutral_special_high_fire_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

//Aerial Neutral Special High Fire Effect
unsafe extern "C" fn springtrap_aerial_neutral_special_high_fire_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Neutral Special High Fire Sound
unsafe extern "C" fn springtrap_neutral_special_high_fire_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n02"));
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_n01"));
    }
}

//Neutral Special High Fire Expression
unsafe extern "C" fn springtrap_neutral_special_high_fire_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .acmd("game_specialnhighfire", springtrap_neutral_special_high_fire_acmd, Low)
    .acmd("effect_specialnhighfire", springtrap_grounded_neutral_special_high_fire_effect, Low)
    .acmd("sound_specialnhighfire", springtrap_neutral_special_high_fire_sound, Low)
    .acmd("expression_specialnhighfire", springtrap_neutral_special_high_fire_expression, Low)
    .acmd("game_specialairnhighfire", springtrap_neutral_special_high_fire_acmd, Low)
    .acmd("effect_specialairnhighfire", springtrap_aerial_neutral_special_high_fire_effect, Low)
    .acmd("sound_specialairnhighfire", springtrap_neutral_special_high_fire_sound, Low)
    .acmd("expression_specialairnhighfire", springtrap_neutral_special_high_fire_expression, Low)
    .install()
    ;
}