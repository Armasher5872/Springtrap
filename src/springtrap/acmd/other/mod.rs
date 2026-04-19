use {
    crate::common::{
        common_func::*,
        springtrap_var::*,
    },
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait,
                wait_loop_sync_mot
            },
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::Vector3f
    },
    smash_script::{
        macros::*,
        *
    },
    smashline::{
        Priority::*,
        *
    },
};

mod appeal_hi;
mod entry;
mod walk_fast;
mod walk_slow;
mod win_1;
mod win_2;
mod win_3;

pub fn install() {
    appeal_hi::install();
    entry::install();
    walk_fast::install();
    walk_slow::install();
    win_1::install();
    win_2::install();
    win_3::install();
}