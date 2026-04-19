use super::*;

unsafe extern "C" fn springtrap_down_special_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 20.0/40.0);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ArticleModule::generate_article(boma, FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE, false, -1);
        WorkModule::on_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_PHANTOM);
    }
}

unsafe extern "C" fn springtrap_down_special_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("springtrap_phantom_summon"), Hash40::new("top"), 0, 12, 18, 0, 90, 0, 0.5, true);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn springtrap_down_special_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 15.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_step_left_m"));
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_h04"));
    }
}

unsafe extern "C" fn springtrap_grounded_down_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn springtrap_aerial_down_special_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit_attacks"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([16, 17, 18, 19, 20, 21, 22, 23].to_vec())
    .game_acmd("game_speciallw", springtrap_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", springtrap_down_special_effect, Low)
    .sound_acmd("sound_speciallw", springtrap_down_special_sound, Low)
    .expression_acmd("expression_speciallw", springtrap_grounded_down_special_expression, Low)
    .game_acmd("game_specialairlw", springtrap_down_special_acmd, Low)
    .effect_acmd("effect_specialairlw", springtrap_down_special_effect, Low)
    .sound_acmd("sound_specialairlw", springtrap_down_special_sound, Low)
    .expression_acmd("expression_specialairlw", springtrap_aerial_down_special_expression, Low)
    .install()
    ;
}