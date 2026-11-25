use super::*;

//Springtrap Phantom Initialization Event Offset
unsafe extern "C" fn springtrap_phantom_initialization_event(_vtable: u64, weapon: *mut smash::app::Weapon, _param_3: u64) {
    let boma = (*weapon).battle_object.module_accessor;
    let shield_data = ShieldData::new(0.0, 6.0, 0.0, 0.0, 6.0, 0.0, 4.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let shield_datas = &mut (ShieldDatas2::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource2::new(shield_datas, 1, 1.0, 1.0, 50.0, 0.0, false, 0);
    add_reflector_group(boma, resource, *WEAPON_SPRINGTRAP_PHANTOM_SHIELD_KIND_BALLOON_BOY_BODY);
}

//Springtrap Phantom On Despawn Offset
unsafe extern "C" fn springtrap_phantom_on_despawn(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    ReflectorModule::clean(boma);
}

//Springtrap Phantom On Attack Offset
unsafe extern "C" fn springtrap_phantom_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, collision_bitmask: u32) -> u64 {
    *(weapon as *mut bool).add(0x90) = true;
    normal_weapon_hit_handler(vtable, weapon, collision_bitmask)
}

//Springtrap Phantom On Search Event
unsafe extern "C" fn springtrap_phantom_on_search_event(_vtable: u64, weapon: &mut smash::app::Weapon, log: *mut CollisionLogScuffed) {
    let boma = (*weapon).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let opponent_object_id = (*log).opponent_object_id;
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let opponent_category = sv_battle_object::category(opponent_object_id);
        if opponent_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let opponent_object = get_battle_object_from_id(opponent_object_id);
            let opponent_battle_object_id = (*opponent_object).battle_object_id;
            let opponent_boma = (*opponent_object).module_accessor;
            if opponent_battle_object_id >> 0x1C == 0 
            && HitModule::get_status(opponent_boma, (*log).receiver_id as i32, 0) == 0
            && WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLAG_CAN_EXPLODE) {
                if [*WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL].contains(&status_kind) {
                    StatusModule::change_status_request(boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_EXPLODE, false);
                }
                if [*WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN].contains(&status_kind) {
                    StatusModule::change_status_request(boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_ATTACK, false);
                    if StatusModule::situation_kind(opponent_boma) == *SITUATION_KIND_AIR {
                        KineticModule::clear_speed_all(opponent_boma);
                        KineticModule::unable_energy_all(opponent_boma);
                        StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
                    }
                    else {
                        StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_FURAFURA_END, false);
                    }
                }
            }
        }
    }
}

//Springtrap Phantom On Reflection Event
unsafe extern "C" fn springtrap_phantom_on_reflection_event(_vtable: u64, weapon: *mut smash::app::Weapon, log: *mut ShieldAttackCollisionEvent) {
    let boma = (*weapon).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let agent = get_weapon_common_from_accessor(&mut *boma);
    let opponent_id = (*(*log).collision_log).opponent_object_id;
    let opponent_power = (*log).real_power;
    if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let opponent_battle_object = get_battle_object_from_id(opponent_id);
        let opponent_boma = (*opponent_battle_object).module_accessor;
        let attack_data = *AttackModule::attack_data(opponent_boma, (*(*log).collision_log).collider_id as i32, (*(*log).collision_log).x35);
        let vec = attack_data.vector;
        let attr = attack_data.attr;
        EffectModule::kill_kind(boma, Hash40::new("sys_reflection"), true, true);
        spawn_hit_effects(agent, attr);
        if opponent_power > 0.0 {
            let opponent_angle = if vec > 360 {32} else {attack_data.vector} as f32;
            let opponent_lr = PostureModule::lr(opponent_boma);
            let speed = opponent_power/8.0;
            let speed_x = opponent_angle.to_radians().cos()*speed*opponent_lr;
            let speed_y = opponent_angle.to_radians().sin()*speed;
            let hit_count = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_HIT_COUNT);
            WorkModule::set_float(boma, speed_x, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_X);
            WorkModule::set_float(boma, speed_y, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_FLOAT_BB_SPEED_Y);
            WorkModule::dec_int(boma, *WEAPON_SPRINGTRAP_PHANTOM_INSTANCE_WORK_ID_INT_HIT_COUNT);
            if [*WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL].contains(&status_kind) {
                StatusModule::change_status_request_from_script(boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL, false);
            }
            if [*WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_MOVE, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_TURN].contains(&status_kind) && hit_count <= 0 {
                StatusModule::change_status_request_from_script(boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_BREAK, false);
            }
        }
    }
}

//Springtrap Phantom Initialize Weapon Module Accessor Event Offset
unsafe extern "C" fn springtrap_phantom_initialize_weapon_module_accessor(_vtable: u64, _boma: *mut BattleObjectModuleAccessor, param_3: u64) {
    *(param_3 as *mut i32).add(0x288/4) = *COLLISION_KIND_SHIELD;
}

pub fn install() {
    unsafe {
        enable_weapon_module(WEAPON_KIND_SPRINGTRAP_PHANTOM, WeaponModule::Reflector);
        enable_weapon_module(WEAPON_KIND_SPRINGTRAP_PHANTOM, WeaponModule::Search);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_PHANTOM, WeaponVtableSlot::WeaponGenerate1, springtrap_phantom_initialization_event as *const () as usize);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_PHANTOM, WeaponVtableSlot::OnWeaponRemove, springtrap_phantom_on_despawn as *const () as usize);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_PHANTOM, WeaponVtableSlot::WeaponOnAttackMask, springtrap_phantom_on_attack as *const () as usize);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_PHANTOM, WeaponVtableSlot::OnSearch, springtrap_phantom_on_search_event as *const () as usize);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_PHANTOM, WeaponVtableSlot::OnReflectorCollision1, springtrap_phantom_on_reflection_event as *const () as usize);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_PHANTOM, WeaponVtableSlot::Unk55, springtrap_phantom_initialize_weapon_module_accessor as *const () as usize);
    }
}