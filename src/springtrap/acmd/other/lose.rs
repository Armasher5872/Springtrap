use super::*;

unsafe extern "C" fn springtrap_lose_game(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    let pos = *PostureModule::pos(boma);
    if pos.x >= 1000.0 && pos.z >= 300.0 {
        if is_excute(agent) {
            PostureModule::set_pos(boma, &Vector3f{x: pos.x+10.0, y: pos.y+12.0, z: pos.z+30.0}); //Repositioning user into lose camera
        }
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_costumes())
    .acmd("game_lose", springtrap_lose_game, Low)
    .install()
    ;
}