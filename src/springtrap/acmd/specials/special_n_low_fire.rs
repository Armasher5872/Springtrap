use super::*;

//Neutral Special Low Fire ACMD
unsafe extern "C" fn springtrap_neutral_special_low_fire_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 15.0/30.0);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE, false, -1);
        WorkModule::on_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
        FT_MOTION_RATE(agent, 1.0);
    }
}

//Grounded Neutral Special Low Fire Effect
unsafe extern "C" fn springtrap_grounded_neutral_special_low_fire_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}

//Aerial Neutral Special Low Fire Effect
unsafe extern "C" fn springtrap_aerial_neutral_special_low_fire_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_axe_fire_ash"), Hash40::new("havel"), 0, 13, 0, 0, 0, 0, 0.5, true);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Neutral Special Low Fire Sound
unsafe extern "C" fn springtrap_neutral_special_low_fire_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_smash_s01"));
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
    }
}

//Neutral Special Low Fire Expression
unsafe extern "C" fn springtrap_neutral_special_low_fire_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_specialnlowfire", springtrap_neutral_special_low_fire_acmd, Low)
    .effect_acmd("effect_specialnlowfire", springtrap_grounded_neutral_special_low_fire_effect, Low)
    .sound_acmd("sound_specialnlowfire", springtrap_neutral_special_low_fire_sound, Low)
    .expression_acmd("expression_specialnlowfire", springtrap_neutral_special_low_fire_expression, Low)
    .game_acmd("game_specialairnlowfire", springtrap_neutral_special_low_fire_acmd, Low)
    .effect_acmd("effect_specialairnlowfire", springtrap_aerial_neutral_special_low_fire_effect, Low)
    .sound_acmd("sound_specialairnlowfire", springtrap_neutral_special_low_fire_sound, Low)
    .expression_acmd("expression_specialairnlowfire", springtrap_neutral_special_low_fire_expression, Low)
    .install()
    ;
}