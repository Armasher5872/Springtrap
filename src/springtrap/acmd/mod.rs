use {
    crate::common::{
        getter_funcs::*,
        springtrap_var::*,
    },
    smash::{
        app::{
            AttackDirectionAxis,
            lua_bind::*,
            sv_animcmd::{
                execute,
                frame,
                FT_ATTACK_ABS_CAMERA_QUAKE,
                wait
            },
            sv_module_access::physics,
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::{
        macros::{
            ATTACK_ABS,
            *
        },
        *
    },
    smashline::{
        Priority::*,
        *
    },
};

mod aerials;
mod grounded;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    grounded::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}