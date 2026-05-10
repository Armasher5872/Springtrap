//Credited to WuBoyTH
use super::*;

const GANON_VTABLE_ON_ATTACK_OFFSET: usize = 0xaa6540;
const GANON_VTABLE_STATUS_TRANSITION_OFFSET: usize = 0xaa6800;
const GANON_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x68d8a0;

//Ganondorf On Attack
#[skyline::hook(offset = GANON_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn ganon_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if is_springtrap_slots(boma) {
        let status_kind = StatusModule::status_kind(boma);
        let collision_log = log as *mut CollisionLogScuffed;
        let opponent_object_id = (*collision_log).opponent_object_id;
        let opponent_battle_object = get_battle_object_from_id(opponent_object_id);
        let opponent_battle_object_id = (*opponent_battle_object).battle_object_id;
        let opponent_category = sv_battle_object::category(opponent_battle_object_id);
        let collision_kind = (*collision_log).collision_kind;
        let lr = PostureModule::lr(boma);
        let mut_attack_data = AttackModule::attack_data(boma, (*collision_log).collider_id as i32, (*collision_log).x35);
        let attack_data = *(mut_attack_data);
        let sound_attr = attack_data.sound_attr as i32;
        let sound_level = attack_data.sound_level as i32;
        if opponent_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let opponent_boma = sv_battle_object::module_accessor(opponent_battle_object_id);
            let opponent_lr = PostureModule::lr(opponent_boma);
            let opponent_pos = *PostureModule::pos(opponent_boma);
            if opponent_lr == lr {
                EffectModule::req(opponent_boma, Hash40::new("springtrap_soul_burst"), &Vector3f{x: opponent_pos.x, y: opponent_pos.y+12.0, z: opponent_pos.z}, &Vector3f{x: 90.0, y: 90.0, z: 0.0}, 1.0, 0, -1, false, 0);
            }
            if status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_ATTACK {
                if collision_kind != *COLLISION_KIND_SHIELD as u8 && attack_data.attr == hash40("collision_attr_saving") && WorkModule::get_float(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE) >= 1.0 {
                    WorkModule::on_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CRIT);
                }
            }
        }
        if sound_attr == *COLLISION_SOUND_ATTR_SPRINGTRAP_KNIFE {
            let volume = match sound_level {
                0 => {0.3},
                1 => {0.5},
                2 => {0.7},
                3 => {1.0},
                _ => {1.0}
            };
            let crit = SoundModule::play_se(boma, Hash40::new("se_ganon_attackhard_h03"), true, false, false, false, enSEType(0));
            SoundModule::set_se_vol(boma, crit as i32, volume, 0);
        }
    }
    original!()(vtable, fighter, log)
}

//Ganondorf Status Transition
#[skyline::hook(offset = GANON_VTABLE_STATUS_TRANSITION_OFFSET)]
unsafe extern "C" fn ganon_status_transition(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if is_springtrap_slots(boma) {
        let active_axe = WorkModule::is_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
        let status_kind = StatusModule::status_kind(boma);
        let active_axe_statuses = [
            *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_CHARGE_LOOP, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_LOW_FIRE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_HIGH_FIRE, 
            *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, *FIGHTER_STATUS_KIND_WIN
        ];
        let unactive_axe_statuses = [
            *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_CHARGE_LOOP, 
            *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_LOW_FIRE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_HIGH_FIRE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, *FIGHTER_STATUS_KIND_WIN
        ];
        if active_axe {
            if active_axe_statuses.contains(&status_kind) {
                ArticleModule::generate_article_enable(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
            }
            else {
                ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        else {
            if unactive_axe_statuses.contains(&status_kind) {
                ArticleModule::generate_article_enable(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
            }
            else {
                ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
    }
    original!()(vtable, fighter)
}

//Ganondorf On Search
#[skyline::hook(offset = GANON_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn ganon_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        let collision_log = *(log as *const u64).add(0x10/0x8);
        let collision_log = collision_log as *const CollisionLog;
        let status_kind = StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
            let opponent_id = (*collision_log).opponent_battle_object_id;
            let opponent_boma = sv_battle_object::module_accessor(opponent_id);
            let opponent_kind = utility::get_kind(&mut *opponent_boma);
            if opponent_kind == *WEAPON_KIND_KROOL_IRONBALL {
                let owner_id = WorkModule::get_int(opponent_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
                if owner_id == fighter.battle_object.battle_object_id {
                    WorkModule::set_int(opponent_boma, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, false);
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        ganon_on_attack,
        ganon_status_transition,
        ganon_on_search
    );
}