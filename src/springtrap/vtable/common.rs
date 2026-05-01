use super::*;

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
            if attacker_kind == *WEAPON_KIND_KROOL_IRONBALL {
                if owner_kind == *FIGHTER_KIND_GANON {
                    if is_springtrap_slots(owner_boma) {
                        if attacker_status_kind == *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY {
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
                            WorkModule::set_int(attacker_boma, defender_object_id as i32, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
                            StatusModule::change_status_request_from_script(attacker_boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, false);
                            if StatusModule::status_kind(owner_boma) == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
                                StatusModule::change_status_request_from_script(owner_boma, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, false);
                            }
                        }
                    }
                }
            }
            if attacker_kind == *WEAPON_KIND_KOOPAJR_CANNONBALL {
                if owner_kind == *FIGHTER_KIND_GANON {
                    if is_springtrap_slots(owner_boma) {
                        if [*WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_IDLE, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_BB_FALL].contains(&attacker_status_kind) {
                            StatusModule::change_status_request_from_script(attacker_boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_PHANTOM_EXPLODE, false);
                        }
                        if [*WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_WALK, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_FALL, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_TURN].contains(&attacker_status_kind) {
                            StatusModule::change_status_request_from_script(attacker_boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_CHICA_ATTACK, false);
                        }
                        if [*WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FREDDY_WALK, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FREDDY_FALL, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FREDDY_TURN].contains(&attacker_status_kind) {
                            StatusModule::change_status_request_from_script(attacker_boma, *WEAPON_SPRINGTRAP_PHANTOM_STATUS_KIND_FREDDY_ATTACK, false);
                        }
                    }
                }
            }
        }
    }
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

pub fn install() {
	skyline::install_hook!(notify_log_event_collision_hit);
}