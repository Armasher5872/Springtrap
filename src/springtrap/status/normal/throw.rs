use super::*;

unsafe extern "C" fn springtrap_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("springtrap_static"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    0.into()
}

unsafe extern "C" fn springtrap_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("springtrap_static"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_INT_EFFECT_ID);
    fighter.sub_throw_uniq_process_exit()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .status(End, *FIGHTER_STATUS_KIND_THROW, springtrap_throw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_THROW, springtrap_throw_exit_status)
    .install()
    ;
}