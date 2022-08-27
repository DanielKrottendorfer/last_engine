

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

    let bs = black_sheep::BlackSheep::new(
        |_ecs| {
            || {
                println!("123");
            }
        },
        |_ecs| {
            |_i: f32| {
                println!("123");
            }
        },
    );
    bs.run();

    #[cfg(not(feature = "debug_off"))]
    println!("Good bye, world!");
}
