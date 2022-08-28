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

    black_sheep::run();

    #[cfg(not(feature = "debug_off"))]
    println!("Good bye, world!");
}
