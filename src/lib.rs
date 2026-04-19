#![feature(proc_macro_hygiene)]
use {
    param_config::*,
    smash::{
        hash40,
        lib::lua_const::FIGHTER_KIND_GANON
    },
    std::collections::HashMap
};

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];
pub static mut LAST_COLOR: i32 = -1;

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const MARKER_FILE: &str = "springtrap.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/ganon/model/body/c{:02}/{}",
            x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }
    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }
    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };
    println!("LOWEST: {} - COLOR NUM: {}", lowest_color, color_num);
    //Param Edits
    disable_kirby_copy(*FIGHTER_KIND_GANON, marked_slots.clone());
    disable_villager_pocket(*FIGHTER_KIND_GANON, marked_slots.clone(), 0);
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("walk_accel_mul"), 0, 0.104)); //Vanilla Value is 0.084
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("walk_accel_add"), 0, 0.06)); //Vanilla Value is 0.0315
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("walk_speed_max"), 0, 0.89)); //Vanilla Value is 0.767
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("dash_speed"), 0, 1.77)); //Vanilla Value is 1.87
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("run_accel_mul"), 0, 0.0995)); //Vanilla Value is 0.10593
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("run_accel_add"), 0, 0.044)); //Vanilla Value is 0.033
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("run_speed_max"), 0, 1.55)); //Vanilla Value is 1.34
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("jump_speed_x_mul"), 0, 0.8)); //Vanilla Value is 0.75
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("jump_speed_x_max"), 0, 1.77)); //Vanilla Value is 1.8
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("jump_initial_y"), 0, 20.35)); //Vanilla Value is 14.0195
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("jump_y"), 0, 33.03)); //Vanilla Value is 25.49
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("mini_jump_y"), 0, 12.55)); //Vanilla Value is 12.24
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("jump_aerial_y"), 0, 30.25)); //Vanilla Value is 26
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("air_accel_x_mul"), 0, 0.03)); //Vanilla Value is 0.03
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("air_speed_x_stable"), 0, 1.09)); //Vanilla Value is 0.83
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("air_brake_x"), 0, 0.0035)); //Vanilla Value is 0.015
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("air_accel_y"), 0, 0.11)); //Vanilla Value is 0.108
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("air_speed_y_stable"), 0, 1.66)); //Vanilla Value is 1.65
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("air_brake_y"), 0, 0.015)); //Vanilla Value is 0.015
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("dive_speed_y"), 0, 3.0)); //Vanilla Value is 2.64
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("weight"), 0, 129.0)); //Vanilla Value is 118
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("landing_attack_air_frame_n"), 0, 12.0));
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("landing_attack_air_frame_f"), 0, 16.0));
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("landing_attack_air_frame_b"), 0, 14.0));
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("landing_attack_air_frame_hi"), 0, 18.0));
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("landing_attack_air_frame_lw"), 0, 15.0));
    update_int_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("attack_combo_max"), 0, 3)); //Vanilla Value is 1
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("combo_attack_12_end"), 0, 30.0)); //Vanilla Value is 0
    update_float_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("combo_attack_13_end"), 0, 35.0)); //Vanilla Value is 0
    update_int_2(*FIGHTER_KIND_GANON, marked_slots.clone(), (hash40("squat_walk_type"), 0, 1)); //Vanilla Value is 0 (false)
    //CSK Stuff
    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
            ui_chara_id: smash::hash40("ui_chara_springtrap"), 
            fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(0x122AF1B944 /* Hash40 of fighter_kind_ganon */), 
            fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(0x122AF1B944 /* Hash40 of fighter_kind_ganon */), 
            ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(0xe53498e68 /* Hash40 of ui_series_fnaf */), 
            fighter_type: the_csk_collection_api::Hash40Type::Overwrite(0x1353795179 /* Hash40 of fighter_type_normal */), 
            alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
            shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
            name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("springtrap")), 
            exhibit_year: the_csk_collection_api::ShortType::Overwrite(2015), 
            exhibit_day_order: the_csk_collection_api::IntType::Overwrite(112103), 
            extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
            ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(0), 
            skill_list_order: the_csk_collection_api::SignedByteType::Overwrite(70), 
            disp_order: the_csk_collection_api::SignedByteType::Optional(Some(88)), 
            save_no: the_csk_collection_api::SignedByteType::Overwrite(25), 
            chara_count: the_csk_collection_api::SignedByteType::Overwrite(1), 
            is_img_ext_skill_page0: the_csk_collection_api::BoolType::Overwrite(false), 
            is_img_ext_skill_page1: the_csk_collection_api::BoolType::Overwrite(false), 
            is_img_ext_skill_page2: the_csk_collection_api::BoolType::Overwrite(false), 
            can_select: the_csk_collection_api::BoolType::Overwrite(true), 
            is_usable_soundtest: the_csk_collection_api::BoolType::Overwrite(true), 
            is_called_pokemon: the_csk_collection_api::BoolType::Overwrite(false), 
            is_mii: the_csk_collection_api::BoolType::Overwrite(false), 
            is_boss: the_csk_collection_api::BoolType::Overwrite(false), 
            is_hidden_boss: the_csk_collection_api::BoolType::Overwrite(false), 
            is_dlc: the_csk_collection_api::BoolType::Overwrite(false), 
            is_patch: the_csk_collection_api::BoolType::Overwrite(false), 
            is_plural_message: the_csk_collection_api::BoolType::Overwrite(false), 
            is_plural_narration: the_csk_collection_api::BoolType::Overwrite(false), 
            is_article: the_csk_collection_api::BoolType::Overwrite(false), 
            result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
            result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
            result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(color_num as _),
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
                (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_springtrap"))),
                (0x1340FBA1B8 /* Hash40 of characall_label_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x13D9F2F002 /* Hash40 of characall_label_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x13AEF5C094 /* Hash40 of characall_label_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1330915537 /* Hash40 of characall_label_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x13479665A1 /* Hash40 of characall_label_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x13DE9F341B /* Hash40 of characall_label_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x13A998048D /* Hash40 of characall_label_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1B8B13E500 /* Hash40 of characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1BFC14D596 /* Hash40 of characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1B651D842C /* Hash40 of characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1B121AB4BA /* Hash40 of characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1B8C7E2119 /* Hash40 of characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1BFB79118F /* Hash40 of characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1B62704035 /* Hash40 of characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x1B157770A3 /* Hash40 of characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)),
                (0x160ab9eb98, the_csk_collection_api::Hash40Type::Overwrite(0xea4221dc6)),
            ])), 
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                (0x915C075DE /* Hash40 of c00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9B3B77E6A /* Hash40 of c01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9825F64F7 /* Hash40 of c02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x924286F43 /* Hash40 of c03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9E18F51CD /* Hash40 of c04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x947F85A79 /* Hash40 of c05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9761040E4 /* Hash40 of c06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9D0674B50 /* Hash40 of c07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9E48F9289 /* Hash40 of n00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x942F8993D /* Hash40 of n01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9731083A0 /* Hash40 of n02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9D5678814 /* Hash40 of n03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x910C0B69A /* Hash40 of n04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9B6B7BD2E /* Hash40 of n05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9875FA7B3 /* Hash40 of n06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x92128AC07 /* Hash40 of n07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9F873561A /* Hash40 of c00_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x95E045DAE /* Hash40 of c01_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x96FEC4733 /* Hash40 of c02_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9C99B4C87 /* Hash40 of c03_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x90C3C7209 /* Hash40 of c04_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x9AA4B79BD /* Hash40 of c05_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x99BA36320 /* Hash40 of c06_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (0x93DD46894 /* Hash40 of c07_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)),
                (smash::hash40("color_start_index"), the_csk_collection_api::UnsignedByteType::Overwrite(lowest_color as _)),
            ])), 
            ..Default::default()
        },
    );
    the_csk_collection_api::add_chara_layout_db_entry_info(the_csk_collection_api::CharacterLayoutDatabaseEntry {
        ui_layout_id: smash::hash40("ui_chara_springtrap_00"), 
        ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_springtrap")), 
        chara_color: the_csk_collection_api::UnsignedByteType::Overwrite(0), 
        eye_0_flash_count: the_csk_collection_api::UnsignedByteType::Overwrite(1), 
        eye_1_flash_count: the_csk_collection_api::UnsignedByteType::Overwrite(1), 
        eye_2_flash_count: the_csk_collection_api::UnsignedByteType::Overwrite(1), 
        eye_0_flash0_pos_x: the_csk_collection_api::FloatType::Overwrite(8.0), 
        eye_0_flash0_pos_y: the_csk_collection_api::FloatType::Overwrite(242.0), 
        eye_0_flash1_pos_x: the_csk_collection_api::FloatType::Overwrite(34.0), 
        eye_0_flash1_pos_y: the_csk_collection_api::FloatType::Overwrite(-62.0), 
        eye_0_flash2_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash2_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash3_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash3_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash4_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash4_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash0_pos_x: the_csk_collection_api::FloatType::Overwrite(4.0), 
        eye_1_flash0_pos_y: the_csk_collection_api::FloatType::Overwrite(230.0), 
        eye_1_flash1_pos_x: the_csk_collection_api::FloatType::Overwrite(34.0), 
        eye_1_flash1_pos_y: the_csk_collection_api::FloatType::Overwrite(-62.0), 
        eye_1_flash2_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash2_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash3_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash3_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash4_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash4_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash0_pos_x: the_csk_collection_api::FloatType::Overwrite(16.0), 
        eye_2_flash0_pos_y: the_csk_collection_api::FloatType::Overwrite(104.0), 
        eye_2_flash1_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash1_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash2_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash2_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash3_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash3_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash4_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash4_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_flash_info_pos_x: the_csk_collection_api::FloatType::Overwrite(3.0), 
        eye_flash_info_pos_y: the_csk_collection_api::FloatType::Overwrite(4.0), 
        chara_1_offset_x: the_csk_collection_api::FloatType::Overwrite(-5.0), 
        chara_1_offset_y: the_csk_collection_api::FloatType::Overwrite(-76.0), 
        chara_1_scale: the_csk_collection_api::FloatType::Overwrite(1.17), 
        chara_1_1_offset_x: the_csk_collection_api::FloatType::Overwrite(-5.0), 
        chara_1_1_offset_y: the_csk_collection_api::FloatType::Overwrite(-80.0), 
        chara_1_1_scale: the_csk_collection_api::FloatType::Overwrite(1.65), 
        chara_1_2_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_2_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_2_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_1_3_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_3_offset_y: the_csk_collection_api::FloatType::Overwrite(-47.0), 
        chara_1_3_scale: the_csk_collection_api::FloatType::Overwrite(1.65), 
        chara_1_4_offset_x: the_csk_collection_api::FloatType::Overwrite(-5.0), 
        chara_1_4_offset_y: the_csk_collection_api::FloatType::Overwrite(-47.0), 
        chara_1_4_scale: the_csk_collection_api::FloatType::Overwrite(1.65), 
        chara_1_5_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_5_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_5_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_3_0_offset_x: the_csk_collection_api::FloatType::Overwrite(95.0), 
        chara_3_0_offset_y: the_csk_collection_api::FloatType::Overwrite(-215.0), 
        chara_3_0_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_3_1_offset_x: the_csk_collection_api::FloatType::Overwrite(92.0), 
        chara_3_1_offset_y: the_csk_collection_api::FloatType::Overwrite(-250.0), 
        chara_3_1_scale: the_csk_collection_api::FloatType::Overwrite(1.05), 
        chara_3_2_offset_x: the_csk_collection_api::FloatType::Overwrite(150.0), 
        chara_3_2_offset_y: the_csk_collection_api::FloatType::Overwrite(-100.0), 
        chara_3_2_scale: the_csk_collection_api::FloatType::Overwrite(0.8), 
        chara_3_3_offset_x: the_csk_collection_api::FloatType::Overwrite(95.0), 
        chara_3_3_offset_y: the_csk_collection_api::FloatType::Overwrite(-215.0), 
        chara_3_3_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_3_4_offset_x: the_csk_collection_api::FloatType::Overwrite(95.0), 
        chara_3_4_offset_y: the_csk_collection_api::FloatType::Overwrite(-215.0), 
        chara_3_4_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_3_5_offset_x: the_csk_collection_api::FloatType::Overwrite(60.0), 
        chara_3_5_offset_y: the_csk_collection_api::FloatType::Overwrite(-240.0), 
        chara_3_5_scale: the_csk_collection_api::FloatType::Overwrite(1.01), 
        chara_3_6_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_3_6_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_3_6_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_3_7_offset_x: the_csk_collection_api::FloatType::Overwrite(95.0), 
        chara_3_7_offset_y: the_csk_collection_api::FloatType::Overwrite(-215.0), 
        chara_3_7_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_5_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_5_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_5_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_select_icon_list_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_select_icon_list_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_select_icon_list_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_7_0_offset_x: the_csk_collection_api::FloatType::Overwrite(-2.0), 
        chara_7_0_offset_y: the_csk_collection_api::FloatType::Overwrite(4.0), 
        chara_7_0_scale: the_csk_collection_api::FloatType::Overwrite(0.96), 
        chara_7_1_offset_x: the_csk_collection_api::FloatType::Overwrite(-2.0), 
        chara_7_1_offset_y: the_csk_collection_api::FloatType::Overwrite(4.0), 
        chara_7_1_scale: the_csk_collection_api::FloatType::Overwrite(0.96), 
        chara_0_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_0_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_0_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        spirits_eye_visible: the_csk_collection_api::BoolType::Overwrite(true), 
        ..Default::default()
    });
}

pub mod common;
mod springtrap;

#[skyline::main(name = "springtrap")]
pub fn main() {
    unsafe {
        //allows online play
        extern "C" {
            fn allow_ui_chara_hash_online(ui_chara_hash: u64);
        }
        allow_ui_chara_hash_online(0x139a45e3cb); //ui_chara_springtrap
    }
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_springtrap");
    the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
        ui_bgm_id: hash40("ui_bgm_zz09_f_springtrap"),
        clone_from_ui_bgm_id: Some(hash40("ui_bgm_z20_f_ganon")),
        stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("set_zz09_f_springtrap")),
        ..Default::default()
    });
    the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry { 
        stream_set_id: hash40("set_zz09_f_springtrap"),
        info0: the_csk_collection_api::Hash40Type::Overwrite(hash40("info_zz09_f_springtrap")),
        ..Default::default()
    });
    the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry { 
        info_id: hash40("info_zz09_f_springtrap"),
        stream_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("stream_zz09_f_springtrap")),
        condition: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_none")),
        condition_process: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_process_add")),
        change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        ..Default::default()
    });
    the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
        stream_id: hash40("stream_zz09_f_springtrap"),
        data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("zz09_f_springtrap")),
        ..Default::default()
    });
    the_csk_collection_api::add_new_bgm_property_entry(&smash_bgm_property::BgmPropertyEntry {
        stream_name: hash40::Hash40::new("zz09_f_springtrap"),
        loop_start_ms: 0,
        loop_start_sample: 0,
        loop_end_ms: 0,
        loop_end_sample: 0,
        duration_ms: 7659,
        duration_sample: 359424 
    });
    the_csk_collection_api::set_fighter_jingle(hash40("ui_chara_springtrap"), "zz09_f_springtrap");
    springtrap::install();
}