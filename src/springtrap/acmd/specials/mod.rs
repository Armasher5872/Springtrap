use {
    crate::common::springtrap_var::*,
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::{
                frame,
                wait
            },
            *
        },
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
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

mod axe_bb_idle;
mod axe_chica_attack;
mod axe_chica_fall;
mod axe_chica_turn;
mod axe_chica_walk;
mod axe_fly;
mod axe_foxy_attack;
mod axe_freddy_attack;
mod axe_freddy_fall;
mod axe_freddy_turn;
mod axe_freddy_walk;
mod axe_hit_stick;
mod axe_hit_stuck;
mod axe_stick;
mod axe_stuck;
mod special_hi_end;
mod special_hi_move;
mod special_hi;
mod special_lw;
mod special_n_high_fire;
mod special_n_hold;
mod special_n_low_fire;
mod special_n_recall_end;
mod special_n_recall_loop;
mod special_n_start;
mod special_s_attack;
mod special_s_hold;
mod special_s;

pub fn install() {
    axe_bb_idle::install();
    axe_chica_attack::install();
    axe_chica_fall::install();
    axe_chica_turn::install();
    axe_chica_walk::install();
    axe_fly::install();
    axe_foxy_attack::install();
    axe_freddy_attack::install();
    axe_freddy_fall::install();
    axe_freddy_turn::install();
    axe_freddy_walk::install();
    axe_hit_stick::install();
    axe_hit_stuck::install();
    axe_stick::install();
    axe_stuck::install();
    special_hi_end::install();
    special_hi_move::install();
    special_hi::install();
    special_lw::install();
    special_n_high_fire::install();
    special_n_hold::install();
    special_n_low_fire::install();
    special_n_recall_end::install();
    special_n_recall_loop::install();
    special_n_start::install();
    special_s_attack::install();
    special_s_hold::install();
    special_s::install();
}