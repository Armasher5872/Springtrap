//Credited to WuBoyTH
use super::*;

const GANON_VTABLE_ON_ATTACK_OFFSET: usize = 0xaa6540;
const GANON_VTABLE_STATUS_TRANSITION_OFFSET: usize = 0xaa6800;
const GANON_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x68d8a0;
const KOOPAJR_CANNONBALL_VTABLE_INITIALIZATION_EVENT_OFFSET: usize = 0x3425870;
const KOOPAJR_CANNONBALL_VTABLE_WEAPON_MODULE_ACCESSOR_INITIALIZATION_EVENT_OFFSET: usize = 0x34257f0;

//Ganondorf On Attack
#[skyline::hook(offset = GANON_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn ganon_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if is_springtrap_slots(boma) {
        let collision_log = log as *mut CollisionLogScuffed;
        let opponent_object_id = (*collision_log).opponent_object_id;
        let opponent_battle_object = get_battle_object_from_id(opponent_object_id);
        let opponent_battle_object_id = (*opponent_battle_object).battle_object_id;
        let opponent_category = sv_battle_object::category(opponent_battle_object_id);
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
        }
        if sound_attr == *COLLISION_SOUND_ATTR_SPRINGTRAP_KNIFE {
            let volume = match sound_level {
                0 => {0.3},
                1 => {0.5},
                2 => {1.0},
                3 => {1.5},
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
        if [
            *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_CHARGE_LOOP, 
            *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_LOW_FIRE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_HIGH_FIRE, *FIGHTER_STATUS_KIND_WIN
        ].contains(&StatusModule::status_kind(boma)) {
            ArticleModule::generate_article_enable(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
        }
        else {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
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
            if opponent_kind == *WEAPON_KIND_KOOPAJR_CANNONBALL {
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

//Notify Log Event Collision Hit
#[skyline::hook(offset=0x67a7b0)]
unsafe extern "C" fn notify_log_event_collision_hit(fighter_manager: u64, attacker_object_id: u32, defender_object_id: u32, move_type: u64, arg5: u64, move_type_again: u64) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_object_id);
    let defender_boma = sv_battle_object::module_accessor(defender_object_id);
    let attacker_category = utility::get_category(&mut *attacker_boma);
    let attacker_status_kind = StatusModule::status_kind(attacker_boma);
    let attacker_kind = sv_battle_object::kind(attacker_object_id);
    let defender_category = utility::get_category(&mut *defender_boma);
    let defender_scale = PostureModule::scale(defender_boma);
    let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if attacker_category == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if defender_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            if attacker_kind == *WEAPON_KIND_KOOPAJR_CANNONBALL {
                if owner_kind == *FIGHTER_KIND_GANON {
                    if is_springtrap_slots(owner_boma) {
                        if [*WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_RECALL].contains(&attacker_status_kind) && !WorkModule::is_flag(attacker_boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_PREVIOUSLY_LINKED) {
                            LinkModule::remove_model_constraint(attacker_boma, true);
                            if LinkModule::is_link(attacker_boma, *WEAPON_LINK_NO_CONSTRAINT) {
                                LinkModule::unlink(attacker_boma, *WEAPON_LINK_NO_CONSTRAINT);
                            }
                            if !LinkModule::is_link(attacker_boma, *WEAPON_LINK_NO_CONSTRAINT) {
                                VisibilityModule::set_whole(attacker_boma, true);
                                LinkModule::link(attacker_boma, *WEAPON_LINK_NO_CONSTRAINT, (*defender_boma).battle_object_id);
                                LinkModule::set_attribute(attacker_boma, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SHAKE as u8}, true);
                                LinkModule::set_model_constraint_pos_ort(attacker_boma, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("bust"), (*CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_SCALE | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_ORIENTATION) as u32, true);
                                ModelModule::set_scale(attacker_boma, 0.73*defender_scale);
                                LinkModule::set_constraint_rot_offset(attacker_boma, &Vector3f{x: -90.0, y: 0.0, z: -90.0});
                                LinkModule::set_constraint_translate_offset(attacker_boma, &Vector3f{x: -2.0*defender_scale, y: -7.0*defender_scale, z: -4.0*defender_scale});
                            }
                            WorkModule::on_flag(attacker_boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED);
                            WorkModule::on_flag(attacker_boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_PREVIOUSLY_LINKED);
                            WorkModule::set_int(attacker_boma, defender_object_id as i32, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
                            StatusModule::change_status_request_from_script(attacker_boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, false);
                            if StatusModule::status_kind(owner_boma) == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
                                StatusModule::change_status_request_from_script(owner_boma, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, false);
                            }
                        }
                        if [*WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_IDLE, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL].contains(&attacker_status_kind) {
                            StatusModule::change_status_request_from_script(attacker_boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_EXPLODE, false);
                        }
                        if [*WEAPON_SPRINGTRAP_AXE_STATUS_KIND_CHICA_WALK, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_CHICA_FALL, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_CHICA_TURN].contains(&attacker_status_kind) {
                            StatusModule::change_status_request_from_script(attacker_boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_CHICA_ATTACK, false);
                        }
                        if [*WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_FALL, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_TURN].contains(&attacker_status_kind) {
                            StatusModule::change_status_request_from_script(attacker_boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_ATTACK, false);
                        }
                    }
                }
            }
        }
    }
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

//Bowser Jr Cannonball Initialization Event Offset
#[skyline::hook(offset = KOOPAJR_CANNONBALL_VTABLE_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn koopajr_cannonball_initialization_event(vtable: u64, weapon: *mut smash::app::Weapon, param_3: u64) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = *(param_3 as *mut u32).add(0x2c/4);
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let ptr = get_module_vtable_func(boma, 0x108, 0x60);
    let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource2, i32) = std::mem::transmute(ptr);
    let reflector_module = *(boma as *mut *mut u64).add(0x108/8);
    if owner_kind == *FIGHTER_KIND_GANON {
        let bb_shield_data = ShieldData::new(0.0, 6.0, 0.0, 0.0, 6.0, 0.0, 4.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
        let bb_shield_datas = &mut (ShieldDatas2::new().add(bb_shield_data, 0));
        let bb_resource = &mut ShieldGroupResource2::new(bb_shield_datas, 1, 1.0, 1.0, 50.0, 0.0, false, 0);
        set_shield_group2(reflector_module, bb_resource, *WEAPON_SPRINGTRAP_AXE_SHIELD_KIND_BALLOON_BOY_BODY);
        ReflectorModule::set_status(boma, *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD, ShieldStatus(*SHIELD_STATUS_NONE), *WEAPON_SPRINGTRAP_AXE_SHIELD_KIND_BALLOON_BOY_BODY);
    }
    call_original!(vtable, weapon, param_3)
}

//Bowser Jr Cannonball Reflector Clean Event Offset
unsafe extern "C" fn koopajr_cannonball_reflector_clean_event(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_GANON {
        ReflectorModule::clean(boma);
    }
}

//Bowser Jr Cannonball On Reflection Event Offset
unsafe extern "C" fn koopajr_cannonball_on_reflection_event(_vtable: u64, weapon: *mut smash::app::Weapon, log: *mut ShieldAttackCollisionEvent) {
    let boma = (*weapon).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let opponent_id = (*(*log).collision_log).opponent_object_id;
    let opponent_battle_object = get_battle_object_from_id(opponent_id);
    let opponent_boma = (*opponent_battle_object).module_accessor;
    let opponent_power = (*log).real_power;
    let opponent_attack_data = AttackModule::attack_data(owner_boma, (*(*log).collision_log).collider_id as i32, (*(*log).collision_log).x35);
    let vec = (*opponent_attack_data).vector;
    let opponent_angle = if vec > 360 {32} else {(*opponent_attack_data).vector} as f32;
    let opponent_lr = PostureModule::lr(opponent_boma);
    let speed = opponent_power/8.0;
    let speed_x = ((opponent_angle+90.0).to_radians().sin()*speed)*opponent_lr;
    let speed_y = (opponent_angle-90.0).to_radians().cos()*speed;
    if owner_kind == *FIGHTER_KIND_GANON {
        WorkModule::set_float(boma, speed_x, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
        WorkModule::set_float(boma, speed_y, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
        if [*WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_IDLE, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL].contains(&status_kind) {
            StatusModule::change_status_request_from_script(boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_BB_FALL, false);
        }
        if [
            *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_CHICA_WALK, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_CHICA_FALL, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_CHICA_TURN, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_WALK, 
            *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_FALL, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FREDDY_TURN
        ].contains(&status_kind) {
            StatusModule::change_status_request_from_script(boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_PHANTOM_BREAK, false);
        }
    }
}

//Bowser Jr Cannonball Initialize Weapon Module Accessor Event Offset
#[skyline::hook(offset = KOOPAJR_CANNONBALL_VTABLE_WEAPON_MODULE_ACCESSOR_INITIALIZATION_EVENT_OFFSET)]
unsafe extern "C" fn koopajr_cannonball_initialize_weapon_module_accessor(vtable: u64, boma: *mut BattleObjectModuleAccessor, param_3: u64) -> u64 {
    *(param_3 as *mut i32).add(0x288/4) = *COLLISION_KIND_SHIELD;
    call_original!(vtable, boma, param_3)
}

pub fn install() {
    //Fuck it we ball type code (Patches the initialization of Bowser Jr's Cannonball modules to instead use Palutena's Reflection Board Module Initialization so that the former can call to ReflectorModule functions correctly)
    let koopajr_cannonball_module_initialization_offset = get_article_module_initialization_offset(*WEAPON_KIND_KOOPAJR_CANNONBALL) as usize;
    let initialize_reflectormodule = unsafe {skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64+0x33b9830};
    let _ = skyline::patching::Patch::in_text(koopajr_cannonball_module_initialization_offset).data(initialize_reflectormodule);
    let _ = skyline::patching::Patch::in_text(0x51d8348).data(koopajr_cannonball_reflector_clean_event as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x51d8468).data(koopajr_cannonball_on_reflection_event as *const () as u64);
	skyline::install_hooks!(
        ganon_on_attack,
        ganon_status_transition,
        ganon_on_search,
        notify_log_event_collision_hit,
        koopajr_cannonball_initialization_event,
        koopajr_cannonball_initialize_weapon_module_accessor
    );
}