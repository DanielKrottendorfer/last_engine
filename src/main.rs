

mod black_sheep;

fn main() {
    println!("Hello, world!");

    let bs = black_sheep::BlackSheep::new();
    bs.run();
}
