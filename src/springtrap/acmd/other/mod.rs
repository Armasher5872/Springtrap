use {
    crate::common::{
        getter_funcs::*,
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
mod appeal_lw;
mod appeal_s;
mod cliff_attack;
mod entry;
mod lose;
mod walk_fast;
mod walk_middle;
mod walk_slow;
mod win_1;
mod win_2;
mod win_3;

pub fn install() {
    appeal_hi::install();
    appeal_lw::install();
    appeal_s::install();
    cliff_attack::install();
    entry::install();
    lose::install();
    walk_fast::install();
    walk_middle::install();
    walk_slow::install();
    win_1::install();
    win_2::install();
    win_3::install();
}