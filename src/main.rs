#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate crossbeam;
extern crate csv;
extern crate regex;
extern crate serde;
extern crate toml;

pub mod exec;
pub mod marshal;
pub mod norm;
//mod unmarshal;

fn main() {
    println!("Hello, world!");
}
