use crate::black_sheep::BlackSheep;

mod black_sheep;

extern crate cgmath;
extern crate gl;
extern crate rand;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;

fn main() {
    #[cfg(not(feature = "debug_off"))]
    println!("Hello, world!");

    let bs: BlackSheep = black_sheep::BlackSheep::new();
    bs.run();

    #[cfg(not(feature = "debug_off"))]
    println!("Good bye, world!");
}
