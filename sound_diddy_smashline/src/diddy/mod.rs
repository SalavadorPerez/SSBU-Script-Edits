use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(agent = "diddy", script = "sound_batswing4", category = ACMD_SOUND)]
unsafe fn diddy_sound_batswing4(fighter: &mut L2CAgentBase) {
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 45.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE(fighter, Hash40::new("vc_diddy_attack07"));
	}
}

#[acmd_script(agent = "diddy", script = "sound_lipstickswing4", category = ACMD_SOUND)]
unsafe fn diddy_sound_lipstickswing4(fighter: &mut L2CAgentBase) {
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		smash_script::macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
	}
	smash::app::sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE(fighter, Hash40::new("couple_stage_roof"));
	}
}

#[acmd_script(agent = "diddy", script = "sound_starrodswing4", category = ACMD_SOUND)]
unsafe fn diddy_sound_starrodswing4(fighter: &mut L2CAgentBase) {
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		smash_script::macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
	}
	smash::app::sv_animcmd::wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE(fighter, Hash40::new("couple_stage_roof"));
	}
}

#[acmd_script(agent = "diddy", script = "sound_win1", category = ACMD_SOUND)]
unsafe fn diddy_sound_win1(fighter: &mut L2CAgentBase) {
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 14.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE(fighter, Hash40::new("se_diddy_special_n01_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 30.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 39.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_diddy_special_n01_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 43.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 63.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_diddy_special_n01_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 92.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_diddy_squat_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 121.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_diddy_special_n01_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 125.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_win01"));
	}
	smash::app::sv_animcmd::frame(fighter.lua_state_agent, 138.0);
	if macros::is_excute(fighter) {
		smash_script::macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_diddy_001"));
	}
}

pub fn install() {
	smashline::install_acmd_scripts!(
		diddy_sound_batswing4,
		diddy_sound_lipstickswing4,
		diddy_sound_starrodswing4,
		diddy_sound_win1,
	);
}
