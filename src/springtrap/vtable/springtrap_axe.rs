use super::*;

//Springtrap Axe On Despawn
unsafe extern "C" fn springtrap_axe_on_despawn(_vtable: u64, weapon: *mut smash::app::Weapon) {
    let boma = (*weapon).battle_object.module_accessor;
    let agent = get_weapon_common_from_accessor(&mut *boma);
    let founder_boma = get_founder_boma(agent);
    if WorkModule::is_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED) {
        LinkModule::remove_model_constraint(boma, true);
        if LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
            LinkModule::unlink(boma, *WEAPON_LINK_NO_CONSTRAINT);
        }
    }
    springtrap_axe_var(boma);
    WorkModule::off_flag(founder_boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
}

//Springtrap Axe On Attack Mask
unsafe extern "C" fn springtrap_axe_on_attack_mask(vtable: u64, weapon: *mut smash::app::Weapon, collision_bitmask: u32) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let axe_object_id = WorkModule::get_int(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
    if StatusModule::status_kind(boma) == *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY {
        if collision_bitmask as i32 & *COLLISION_KIND_MASK_SHIELD == 0 {
            if axe_object_id != *BATTLE_OBJECT_ID_INVALID {
                let axe_boma = sv_battle_object::module_accessor(axe_object_id as u32);
                let axe_scale = PostureModule::scale(axe_boma);
                LinkModule::remove_model_constraint(boma, true);
                if LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
                    LinkModule::unlink(boma, *WEAPON_LINK_NO_CONSTRAINT);
                }
                if !LinkModule::is_link(boma, *WEAPON_LINK_NO_CONSTRAINT) {
                    VisibilityModule::set_whole(boma, true);
                    LinkModule::link(boma, *WEAPON_LINK_NO_CONSTRAINT, axe_object_id as u32);
                    LinkModule::set_attribute(boma, *WEAPON_LINK_NO_CONSTRAINT, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_SHAKE as u8}, true);
                    LinkModule::set_model_constraint_pos_ort(boma, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("bust"), (*CONSTRAINT_FLAG_OFFSET_ROT | *CONSTRAINT_FLAG_OFFSET_SCALE | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_ORIENTATION) as u32, true);
                    ModelModule::set_scale(boma, 0.73*axe_scale);
                    LinkModule::set_constraint_rot_offset(boma, &Vector3f{x: -90.0, y: 0.0, z: -90.0});
                    LinkModule::set_constraint_translate_offset(boma, &Vector3f{x: -2.0*axe_scale, y: -7.0*axe_scale, z: -4.0*axe_scale});
                }
                WorkModule::on_flag(boma, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_FLAG_LINKED);
                StatusModule::change_status_request(boma, *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_HIT_STICK, false);
            }
        }
    }
    *(weapon as *mut bool).add(0x90) = true;
    normal_weapon_hit_handler(vtable, weapon, collision_bitmask)
}

//Springtrap Axe On Search
unsafe extern "C" fn springtrap_axe_on_search_event(_vtable: u64, weapon: &mut smash::app::Weapon, log: *mut CollisionLogScuffed) {
    let boma = (*weapon).battle_object.module_accessor;
    let opponent_object_id = (*log).opponent_object_id;
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        if StatusModule::status_kind(boma) == *WEAPON_SPRINGTRAP_AXE_STATUS_KIND_FLY {
            let opponent_battle_object = get_battle_object_from_id(opponent_object_id);
            let opponent_battle_object_id = (*opponent_battle_object).battle_object_id;
            let opponent_boma = (*opponent_battle_object).module_accessor;
            if opponent_battle_object_id >> 0x1C == 0 && HitModule::get_status(opponent_boma, (*log).receiver_id as i32, 0) == 0 {
                WorkModule::set_int(boma, opponent_object_id as i32, *WEAPON_SPRINGTRAP_AXE_INSTANCE_WORK_ID_INT_OBJECT_ID);
            }
        }
    }
}

pub fn install() {
    unsafe {
        enable_weapon_module(WEAPON_KIND_SPRINGTRAP_AXE, WeaponModule::Search);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_AXE, WeaponVtableSlot::OnWeaponRemove, springtrap_axe_on_despawn as *const () as usize);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_AXE, WeaponVtableSlot::WeaponOnAttackMask, springtrap_axe_on_attack_mask as *const () as usize);
        replace_weapon_vtable(WEAPON_KIND_SPRINGTRAP_AXE, WeaponVtableSlot::OnSearch, springtrap_axe_on_search_event as *const () as usize);
    }
}