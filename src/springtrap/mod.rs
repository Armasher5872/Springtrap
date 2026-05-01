use {
    crate::common::{
        globals::*,
        springtrap_func::*, 
        springtrap_var::*
    },
    smash::{
        app::{
            lua_bind::*, 
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smashline::*,
};

mod acmd;
mod opff;
mod status;
mod vtable;

pub fn install() {
    unsafe {
        FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_AXE += clone_weapon(
            "krool",
            *WEAPON_KIND_KROOL_IRONBALL,
            "ganon",
            "axe",
            false,
        );
        FIGHTER_SPRINGTRAP_GENERATE_ARTICLE_PHANTOM += clone_weapon(
            "koopajr",
            *WEAPON_KIND_KOOPAJR_CANNONBALL,
            "ganon",
            "phantom",
            false,
        );
    }
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
}