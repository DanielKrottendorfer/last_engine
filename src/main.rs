use crate::black_sheep::BlackSheep;

mod black_sheep;

fn main() {
    println!("Hello, world!");

    let bs: BlackSheep = black_sheep::BlackSheep::new();
    bs.run();
}
