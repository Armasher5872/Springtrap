use {
    crate::common::{
        common_func::*,
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
            "koopajr",
            *WEAPON_KIND_KOOPAJR_CANNONBALL,
            "ganon",
            "axe",
            false,
        );
    }
    acmd::install();
    opff::install();
    status::install();
    vtable::install();
}