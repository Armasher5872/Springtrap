use super::*;

unsafe extern "C" fn krool_ironball_on_attack_event(vtable: u64, weapon: *mut smash::app::Weapon, collision_bitmask: u32) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_GANON {
        if is_springtrap_slots(owner_boma) {
            *(weapon as *mut bool).add(0x90) = true;
        }
    }
    else {
        if status_kind == *WEAPON_KROOL_IRONBALL_STATUS_KIND_SHOOT && WorkModule::is_flag(boma, *WEAPON_KROOL_IRONBALL_INSTANCE_WORK_ID_FLAG_HOP) {
            StatusModule::change_status_request(boma, *WEAPON_KROOL_IRONBALL_STATUS_KIND_HOP, false);
        }
    }
    normal_weapon_hit_handler(vtable, weapon, collision_bitmask)
}

unsafe extern "C" fn krool_ironball_on_search_event(_vtable: u64, weapon: &mut smash::app::Weapon, log: *mut CollisionLogScuffed) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let collision_kind = (*log).collision_kind;
    let opponent_object_id = (*log).opponent_object_id;
    if owner_kind == *FIGHTER_KIND_GANON {
        if is_springtrap_slots(owner_boma) {
            if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                let opponent_category = sv_battle_object::category(opponent_object_id);
                if opponent_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    let opponent_battle_object = get_battle_object_from_id(opponent_object_id);
                    let opponent_battle_object_id = (*opponent_battle_object).battle_object_id;
                    let opponent_boma = (*opponent_battle_object).module_accessor;
                    let opponent_scale = PostureModule::scale(opponent_boma);
                    if StatusModule::status_kind(boma) == *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY {
                        if collision_kind == 1 && opponent_battle_object_id >> 0x1C == 0 && HitModule::get_status((*opponent_battle_object).module_accessor, (*log).receiver_id as i32, 0) == 0 {
                            LinkModule::remove_model_constraint(boma, true);
                            if LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
                                LinkModule::unlink(boma, *WEAPON_LINK_NO_CONSTRAINT);
                            }
                            if !LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
                                VisibilityModule::set_whole(boma, true);
                                LinkModule::link(boma, *WEAPON_LINK_NO_CONSTRAINT, (*opponent_battle_object).battle_object_id);
                                LinkModule::set_attribute(boma, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SHAKE as u8}, true);
                                LinkModule::set_model_constraint_pos_ort(boma, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("bust"), (*CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_SCALE | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_ORIENTATION) as u32, true);
                                ModelModule::set_scale(boma, 0.73*opponent_scale);
                                LinkModule::set_constraint_rot_offset(boma, &Vector3f{x: -90.0, y: 0.0, z: -90.0});
                                LinkModule::set_constraint_translate_offset(boma, &Vector3f{x: -2.0*opponent_scale, y: -7.0*opponent_scale, z: -4.0*opponent_scale});
                            }
                            WorkModule::on_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED);
                            WorkModule::set_int(boma, opponent_object_id as i32, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
                            StatusModule::change_status_request(boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, false);
                        }
                    }
                }
            }
        }
    }
}

pub fn install() {
    weapon_initialise_module(*WEAPON_KIND_KROOL_IRONBALL, ModuleInitModules::SearchModule);
    let _ = skyline::patching::Patch::in_text(0x51da8a8).data(krool_ironball_on_attack_event as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x51da8d8).data(krool_ironball_on_search_event as *const () as u64);
}