#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod yoshi;
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;

macro_rules! c_str {($l:tt) => {[$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();};}

#[skyline::main(name = "yoshi_smashline")]
pub fn main() {
	yoshi::install();
	unsafe {
		skyline::nn::ro::LookupSymbol(&mut FIGHTER_CUTIN_MANAGER_ADDR, c_str!("_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"));
	}
}