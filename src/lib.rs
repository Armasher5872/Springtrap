#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod custom;

#[skyline::main(name = "Replace with the same name as your plugin")]
pub fn main() {
    custom::install();
}